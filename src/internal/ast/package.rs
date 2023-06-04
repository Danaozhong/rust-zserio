use crate::internal::ast::{zenum::ZEnum, zstruct::ZStruct};
use std::string::String;

use super::{
    type_reference::InstantiateType, zbitmask::ZBitmaskType, zchoice::ZChoice, zconst::ZConst,
    zsubtype::Subtype, zunion::ZUnion,
};

pub struct ZImport {
    pub package_dir: Vec<String>,
    pub symbol_name: String,
}
pub struct ZPackage {
    pub name: String,
    pub comment: String,

    pub imports: Vec<ZImport>,
    pub structs: Vec<ZStruct>,
    pub zchoices: Vec<ZChoice>,
    pub zunions: Vec<ZUnion>,
    pub enums: Vec<ZEnum>,
    pub consts: Vec<ZConst>,
    pub subtypes: Vec<Subtype>,
    pub instantiated_types: Vec<InstantiateType>,
    pub bitmask_types: Vec<ZBitmaskType>,
}
