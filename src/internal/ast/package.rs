
use std::string::String;
use crate::internal::ast::{

    zstruct::ZStruct,
    zenum::ZEnum,
};


pub struct ZImport {
    pub package_dir: Vec<String>,
    pub symbol_name: String,
}
pub struct ZPackage {
	pub name: String,
    pub comment: String,

    pub imports: Vec<Box<ZImport>>,
    pub structs: Vec<Box<ZStruct>>,
    pub enums: Vec<Box<ZEnum>>,
}

impl ZPackage {
    fn dummy(&self) {
        // TODO
    }
}