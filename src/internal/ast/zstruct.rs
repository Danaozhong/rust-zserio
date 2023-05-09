use std::string::String;
use crate::internal::ast::field::Field;
pub struct ZStruct {
    pub name: String,
    pub comment: String,
    pub template_params: Vec<String>,
    pub fields: Vec<Box<Field>>,
}
