use crate::internal::ast::expression::Expression;
use crate::internal::parser::gen::zserioparser::LBRACKET;

/// Generates the expression that is used for indexed expressions.
pub fn extract_indexed_offset_expression(expression: &Expression) -> Expression {
    // for @index offset expressions, only generate the variable name, not the
    // full array expression.
    // For example:
    // Do not generate
    // offset_var[@index]
    // but generate
    // offset_var
    // only.
    if expression.expression_type == LBRACKET {
        return *(expression.operand1.as_ref().unwrap().clone());
    }
    // if the offset expression does not contain an @index array expression, it can be generated
    // normally.
    expression.clone()
}
