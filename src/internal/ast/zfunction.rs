use crate::internal::ast::expression::Expression;
use crate::internal::ast::type_reference::TypeReference;

pub struct ZFunction {
    pub name: String,
    pub result: Option<Box<Expression>>,
    pub return_type: Option<Box<TypeReference>>,
}
