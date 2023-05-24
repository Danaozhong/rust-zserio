use std::string::String;

use super::type_reference::TypeReference;

pub struct ZEnumItem {
    pub name: String,
    pub comment: String,
    //pub expression: Expression,
}

pub struct ZEnum {
    pub name: String,
    pub comment: String,
    pub items: Vec<ZEnumItem>,
    pub enum_type: Box<TypeReference>,
}
