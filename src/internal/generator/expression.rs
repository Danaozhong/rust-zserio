use crate::internal::ast::expression::{
    EvaluationState, Expression, ExpressionFlag, ExpressionType,
};
use crate::internal::compiler::symbol_scope::Symbol;
use crate::internal::generator::types::{
    convert_field_name, convert_to_enum_field_name, custom_type_to_rust_type, to_rust_type_name,
};
use crate::internal::parser::gen::zserioparser::{
    AND, BANG, BINARY_LITERAL, BOOL_LITERAL, DECIMAL_LITERAL, DIVIDE, DOT, DOUBLE_LITERAL, EQ,
    FLOAT_LITERAL, GE, GT, HEXADECIMAL_LITERAL, ID, LE, LENGTHOF, LOGICAL_AND, LOGICAL_OR, LPAREN,
    LSHIFT, LT, MINUS, MODULO, MULTIPLY, NE, OCTAL_LITERAL, OR, PLUS, QUESTIONMARK, RPAREN, RSHIFT,
    TILDE, VALUEOF, XOR,
};

pub struct ExpressionGenerationResult {
    pub generated_value: String,
    pub is_lvalue: bool,
    pub lvalue_name: String,
}

pub fn generate_expression(expression: &Expression) -> String {
    assert!(expression.evaluation_state == EvaluationState::Completed);
    match expression.expression_type {
        LPAREN => format!(
            "({})",
            generate_expression(expression.operand1.as_ref().unwrap())
        ),
        RPAREN => format!(
            "{}()",
            generate_expression(expression.operand1.as_ref().unwrap())
        ),
        DOT => generate_dot_expression(expression),
        VALUEOF => generate_valueof_expression(expression),
        LENGTHOF => generate_lengthof_expression(expression),
        EQ | GE | GT | LE | LT | NE => generate_comparison_expression(expression),
        PLUS | MINUS | MULTIPLY | DIVIDE | MODULO => generate_arithmetic_expression(expression),
        QUESTIONMARK => generate_ternary_expression(expression),
        BANG => generate_logical_negation(expression),
        TILDE => generate_bitwise_negation(expression),
        AND | OR | XOR | LSHIFT | RSHIFT => generate_bitwise_expression(expression),
        LOGICAL_AND | LOGICAL_OR => generate_logical_expression(expression),
        ID => generate_identifier_expression(expression),
        BOOL_LITERAL | OCTAL_LITERAL | HEXADECIMAL_LITERAL | BINARY_LITERAL | DECIMAL_LITERAL
        | FLOAT_LITERAL | DOUBLE_LITERAL => generate_literal_expression(expression),
        /*
        0xFFFFF => (), // Ignore
         */
        _ => panic!("unsupported expression type"),
    }
}

fn generate_unary_arithmetic_expression(expression: &Expression) -> String {
    format!(
        "{}{}",
        match expression.expression_type {
            PLUS => "+",
            MINUS => "-",
            _ => panic!("unexpected unary arithmetic expression operator"),
        },
        generate_expression(expression.operand1.as_ref().unwrap()),
    )
}

fn generate_arithmetic_expression(expression: &Expression) -> String {
    if expression.operand2.is_none() {
        // an arithmetic expression may be +5 or -5, i.e. a sign
        // of a float or integer expression.
        return generate_unary_arithmetic_expression(expression);
    }
    format!(
        "{} {} {}",
        generate_expression(expression.operand1.as_ref().unwrap()),
        match expression.expression_type {
            PLUS => "+",
            MINUS => "-",
            MULTIPLY => "*",
            DIVIDE => "/",
            MODULO => "%",
            _ => panic!("unexpected arithmetic expression operator"),
        },
        generate_expression(expression.operand2.as_ref().unwrap()),
    )
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
        ExpressionType::BitMask(z_bitmask) => {
            format!(
                "{}::{}",
                custom_type_to_rust_type(&z_bitmask.borrow().name),
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
                Symbol::Parameter(z_param) => {
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
                        convert_field_name(&z_param.borrow().name),
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

fn generate_valueof_expression(expression: &Expression) -> String {
    format!(
        "ztype::valueof({})",
        generate_expression(expression.operand1.as_ref().unwrap())
    )
}

fn generate_lengthof_expression(expression: &Expression) -> String {
    format!(
        "ztype::lengthof({})",
        generate_expression(expression.operand1.as_ref().unwrap())
    )
}

fn generate_identifier_expression(expression: &Expression) -> String {
    match expression.symbol.as_ref().unwrap() {
        Symbol::Struct(s) => to_rust_type_name(&s.borrow().name),
        Symbol::Choice(c) => to_rust_type_name(&c.borrow().name),
        Symbol::Union(u) => to_rust_type_name(&u.borrow().name),
        Symbol::Enum(e) => to_rust_type_name(&e.borrow().name),
        Symbol::Bitmask(bitmask) => to_rust_type_name(&bitmask.borrow().name),
        Symbol::Field(f) => format!("self.{}", convert_field_name(&f.borrow().name)),
        Symbol::Parameter(p) => format!("self.{}", convert_field_name(&p.borrow().name)),
        Symbol::Function(z_function) => {
            format!("self.{}", convert_field_name(&z_function.borrow().name))
        }
        _ => panic!("unsupported identifier type {:?}", expression.symbol),
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
        generate_expression(expression.operand1.as_ref().unwrap()),
        generate_expression(expression.operand2.as_ref().unwrap()),
        generate_expression(expression.operand3.as_ref().unwrap()),
    )
}

fn generate_logical_negation(expression: &Expression) -> String {
    format!(
        "!{}",
        generate_expression(expression.operand1.as_ref().unwrap())
    )
}

fn generate_bitwise_negation(expression: &Expression) -> String {
    format!(
        "~{}",
        generate_expression(expression.operand1.as_ref().unwrap())
    )
}

fn generate_bitwise_expression(expression: &Expression) -> String {
    format!(
        "{} {} {}",
        generate_expression(expression.operand1.as_ref().unwrap()),
        match expression.expression_type {
            AND => "&",
            OR => "|",
            XOR => "^",
            LSHIFT => "<<",
            RSHIFT => ">>",
            _ => panic!("unexpected bitwise expression operator"),
        },
        generate_expression(expression.operand2.as_ref().unwrap()),
    )
}

fn generate_logical_expression(expression: &Expression) -> String {
    format!(
        "{} {} {}",
        generate_expression(expression.operand1.as_ref().unwrap()),
        match expression.expression_type {
            LOGICAL_AND => "&&",
            LOGICAL_OR => "||",
            _ => panic!("unexpected logical expression operator"),
        },
        generate_expression(expression.operand2.as_ref().unwrap()),
    )
}

fn generate_comparison_expression(expression: &Expression) -> String {
    format!(
        "{} {} {}",
        generate_expression(expression.operand1.as_ref().unwrap()),
        match expression.expression_type {
            EQ => "==",
            LE => "<=",
            LT => "<",
            GT => ">",
            GE => ">=",
            NE => "!=",
            _ => panic!("unexpected comparison expression operator"),
        },
        generate_expression(expression.operand2.as_ref().unwrap()),
    )
}

fn generate_literal_expression(expression: &Expression) -> String {
    let mut float_value = 0.0;
    let mut int_value = 0;
    let mut bool_value = false;
    match expression.result_type {
        ExpressionType::Integer(v) => int_value = v,
        ExpressionType::Bool(b) => bool_value = b,
        ExpressionType::Float(f) => float_value = f,
        _ => panic!(),
    };

    match expression.expression_type {
        DECIMAL_LITERAL => i32::to_string(&int_value),
        HEXADECIMAL_LITERAL => format!("{int_value:#x}"),
        OCTAL_LITERAL => format!("{int_value:#o}"),
        FLOAT_LITERAL | DOUBLE_LITERAL => format!("{}", &float_value),
        BOOL_LITERAL => {
            if bool_value {
                String::from("true")
            } else {
                String::from("false")
            }
        }

        _ => panic!("unexpected comparison expression operator"),
    }
}
