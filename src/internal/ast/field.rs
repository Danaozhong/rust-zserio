use crate::internal::ast::{expression::Expression, type_reference::TypeReference};
use crate::internal::compiler::symbol_scope::ModelScope;
use std::string::String;

pub struct Array {
    pub is_packed: bool,
    pub is_implicit: bool,
    pub array_length_expression: Option<Box<Expression>>,
}

pub struct Field {
    pub name: String,
    pub zserio_name: String,
    pub comment: String,
    pub is_optional: bool,
    pub alignment: u8,
    pub field_type: Box<TypeReference>,

    // Specifies if the field is an array
    pub array: Option<Array>,
}

impl Field {
    pub fn evaluate(&mut self, scope: &ModelScope) {
        // TODO wait for expression evaluation PR to be merged
    }
}
