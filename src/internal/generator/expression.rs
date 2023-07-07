use crate::internal::ast::expression::{EvaluationState, Expression, ExpressionType};
use crate::internal::generator::types::{convert_to_enum_field_name, custom_type_to_rust_type};
use crate::internal::parser::gen::zserioparser::{
    AND, BANG, DIVIDE, DOT, ID, INDEX, LBRACKET, LPAREN, LSHIFT, MINUS, MODULO, MULTIPLY, OR, PLUS,
    RPAREN, RSHIFT, TILDE, XOR,
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
        /*
        PLUS => self.evaluate_arithmetic_expression(),
        MINUS => self.evaluate_arithmetic_expression(),
        MULTIPLY => self.evaluate_arithmetic_expression(),
        DIVIDE => self.evaluate_arithmetic_expression(),
        MODULO => self.evaluate_arithmetic_expression(),
        BANG => self.evaluate_logical_negation(),
        TILDE => self.evaluate_bitwise_negation(),
        AND => self.evaluate_bitwise_expression(),
        OR => self.evaluate_bitwise_expression(),
        XOR => self.evaluate_bitwise_expression(),
        LSHIFT => self.evaluate_bitwise_expression(),
        RSHIFT => self.evaluate_bitwise_expression(),
        INDEX => self.evaluate_index_expression(),
        */
        ID => generate_identifier_expression(expression),
        /*
        0xFFFFF => (), // Ignore
         */
        _ => panic!("unsupported expression type"),
    };
}

fn generate_dot_expression(expression: &Expression) -> String {
    let op1 = expression.operand1.as_ref().unwrap();

    return match &op1.result_type {
        ExpressionType::Enum(z_enum) => {
            format!(
                "{}::{}",
                custom_type_to_rust_type(&z_enum.as_ref().borrow().name),
                convert_to_enum_field_name(&expression.operand2.as_ref().unwrap().text)
            )
        }
        _ => panic!("unsupported dot expression"),
    };
}

fn generate_identifier_expression(expression: &Expression) -> String {
    expression.text.clone()
}
