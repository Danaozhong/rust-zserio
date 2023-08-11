use crate::internal::ast::expression::{
    EvaluationState, Expression, ExpressionFlag, ExpressionType,
};
use crate::internal::compiler::symbol_scope::Symbol;
use crate::internal::generator::types::{
    convert_field_name, convert_to_enum_field_name, custom_type_to_rust_type,
};
use crate::internal::parser::gen::zseriolexer::DECIMAL_LITERAL;
use crate::internal::parser::gen::zserioparser::{
    AND, BANG, DIVIDE, DOT, EQ, GE, GT, ID, INDEX, LBRACKET, LE, LPAREN, LSHIFT, LT, MINUS, MODULO,
    MULTIPLY, NE, OR, PLUS, QUESTIONMARK, RPAREN, RSHIFT, TILDE, XOR,
};

pub struct ExpressionGenerationResult {
    pub generated_value: String,
    pub is_lvalue: bool,
    pub lvalue_name: String,
}

pub fn generate_expression(expression: &Expression) -> String {
    assert!(expression.evaluation_state == EvaluationState::Completed);
    return match expression.expression_type {
        LPAREN => format!(
            "({})",
            generate_expression(&expression.operand1.as_ref().unwrap())
        ),
        RPAREN => format!(
            "{}()",
            generate_expression(&expression.operand1.as_ref().unwrap())
        ),
        DOT => generate_dot_expression(expression),
        EQ | GE | GT | LE | LT | NE => generate_comparison_expression(expression),
        PLUS | MINUS | MULTIPLY | DIVIDE | MODULO => generate_arithmetic_expression(expression),
        QUESTIONMARK => generate_ternary_expression(expression),
        BANG => generate_logical_negation(expression),
        TILDE => generate_bitwise_negation(expression),
        AND | OR | XOR | LSHIFT | RSHIFT => generate_bitwise_expression(expression),
        ID => generate_identifier_expression(expression),
        DECIMAL_LITERAL => generate_literal_expression(expression),
        /*
        0xFFFFF => (), // Ignore
         */
        _ => panic!("unsupported expression type"),
    };
}

fn generate_arithmetic_expression(expression: &Expression) -> String {
    return format!(
        "{} {} {}",
        generate_expression(&expression.operand1.as_ref().unwrap()),
        match expression.expression_type {
            PLUS => "+",
            MINUS => "-",
            MULTIPLY => "*",
            DIVIDE => "/",
            MODULO => "%",
            _ => panic!("unexpected arithmetic expression operator"),
        },
        generate_expression(&expression.operand2.as_ref().unwrap()),
    );
}

fn generate_dot_expression(expression: &Expression) -> String {
    let op1 = expression.operand1.as_ref().unwrap();

    return match &op1.result_type {
        ExpressionType::Enum(z_enum) => {
            format!(
                "{}::{}",
                custom_type_to_rust_type(&z_enum.borrow().name),
                convert_to_enum_field_name(&expression.operand2.as_ref().unwrap().text)
            )
        }
        ExpressionType::Compound => {
            match op1
                .symbol
                .as_ref()
                .expect("failed to retrieve the symbol of compound expression")
            {
                Symbol::Field(z_field) => {
                    let op2 = expression
                        .operand2
                        .as_ref()
                        .expect("a dot expression must have two operands");

                    let right_side = match op2.flag {
                        ExpressionFlag::IsDotExpressionRightOperand => {
                            convert_field_name(&op2.text)
                        }
                        _ => panic!("failed to generate right side of field dot expression"),
                    };

                    format!(
                        "self.{}.{}",
                        convert_field_name(&z_field.borrow().name),
                        right_side,
                    )
                }
                _ => panic!(
                    "unsupported symbol type for dot expression generation {:?}",
                    op1.symbol
                ),
            }
        }
        _ => panic!("unsupported dot expression {:?}", op1),
    };
}

fn generate_identifier_expression(expression: &Expression) -> String {
    match expression.symbol.as_ref().unwrap() {
        Symbol::Struct(s) => s.borrow().name.clone(),
        Symbol::Enum(e) => e.borrow().name.clone(),
        Symbol::Field(f) => format!("self.{}", f.borrow().name),
        Symbol::Parameter(p) => format!("self.{}", p.borrow().name),
        _ => panic!("unsupported identifier type"),
    }
}

fn generate_ternary_expression(expression: &Expression) -> String {
    format!(
        "
if {} {{
    {}
}} else {{
    {}
}}",
        generate_expression(&expression.operand1.as_ref().unwrap()),
        generate_expression(&expression.operand2.as_ref().unwrap()),
        generate_expression(&expression.operand3.as_ref().unwrap()),
    )
}

fn generate_logical_negation(expression: &Expression) -> String {
    format!(
        "!{}",
        generate_expression(&expression.operand1.as_ref().unwrap())
    )
}

fn generate_bitwise_negation(expression: &Expression) -> String {
    format!(
        "~{}",
        generate_expression(&expression.operand1.as_ref().unwrap())
    )
}

fn generate_bitwise_expression(expression: &Expression) -> String {
    return format!(
        "{} {} {}",
        generate_expression(&expression.operand1.as_ref().unwrap()),
        match expression.expression_type {
            AND => "&",
            OR => "|",
            XOR => "^",
            LSHIFT => "<<",
            RSHIFT => ">>",
            _ => panic!("unexpected bitwise expression operator"),
        },
        generate_expression(&expression.operand2.as_ref().unwrap()),
    );
}

fn generate_comparison_expression(expression: &Expression) -> String {
    return format!(
        "{} {} {}",
        generate_expression(&expression.operand1.as_ref().unwrap()),
        match expression.expression_type {
            EQ => "==",
            LE => "<=",
            LT => "<",
            GT => ">",
            GE => ">=",
            NE => "!=",
            _ => panic!("unexpected comparison expression operator"),
        },
        generate_expression(&expression.operand2.as_ref().unwrap()),
    );
}

fn generate_literal_expression(expression: &Expression) -> String {
    let literal_value = match expression.result_type {
        ExpressionType::Integer(v) => v,
        _ => panic!(),
    };

    match expression.expression_type {
        DECIMAL_LITERAL => i32::to_string(&literal_value),
        _ => panic!("unexpected comparison expression operator"),
    }
}
