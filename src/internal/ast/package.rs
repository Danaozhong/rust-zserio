use crate::internal::ast::{zenum::ZEnum, zstruct::ZStruct};
use std::string::String;

pub struct ZImport {
    pub package_dir: Vec<String>,
    pub symbol_name: String,
}
pub struct ZPackage {
    pub name: String,
    pub comment: String,

    pub imports: Vec<ZImport>,
    pub structs: Vec<ZStruct>,
    pub enums: Vec<ZEnum>,
}
