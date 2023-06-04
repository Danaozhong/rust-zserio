use super::type_reference::TypeReference;
use crate::internal::ast::expression::Expression;

pub struct ZConst {
    pub name: String,
    pub comment: String,
    pub zserio_type: Option<Box<TypeReference>>,
    pub value_expression: Option<Box<Expression>>,
}
