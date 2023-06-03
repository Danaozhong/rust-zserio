use crate::internal::ast::expression::Expression;
use std::string::String;
#[derive(Clone)]
pub struct TypeReference {
    pub is_builtin: bool,
    pub package: String,
    pub name: String,
    pub bits: i8,
    pub template_arguments: Vec<Box<TypeReference>>,
    pub type_arguments: Vec<Box<Expression>>,
    pub length_expression: Option<Box<Expression>>,
}

pub struct InstantiateType {
    pub name: String,
    pub zserio_type: Option<Box<TypeReference>>,
}
