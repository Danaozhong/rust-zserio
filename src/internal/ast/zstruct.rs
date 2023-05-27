use crate::internal::ast::field::Field;
use std::string::String;
pub struct ZStruct {
    pub name: String,
    pub comment: String,
    pub template_params: Vec<String>,
    pub fields: Vec<Field>,
}
