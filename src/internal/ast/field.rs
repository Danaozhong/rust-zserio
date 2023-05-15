use crate::internal::ast::type_reference::TypeReference;
use std::string::String;

pub struct Field {
    pub name: String,
    pub zserio_name: String,
    pub comment: String,
    pub is_optional: bool,
    pub alignment: u8,
    pub field_type: Box<TypeReference>,
}
