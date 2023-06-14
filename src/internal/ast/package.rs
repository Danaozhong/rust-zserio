use crate::internal::ast::{zenum::ZEnum, zstruct::ZStruct};
use std::cell::RefCell;
use std::rc::Rc;
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
    pub structs: Vec<Rc<RefCell<ZStruct>>>,
    pub zchoices: Vec<Rc<RefCell<ZChoice>>>,
    pub zunions: Vec<Rc<RefCell<ZUnion>>>,
    pub enums: Vec<Rc<RefCell<ZEnum>>>,
    pub consts: Vec<Rc<RefCell<ZConst>>>,
    pub subtypes: Vec<Rc<RefCell<Subtype>>>,
    pub instantiated_types: Vec<Rc<RefCell<InstantiateType>>>,
    pub bitmask_types: Vec<Rc<RefCell<ZBitmaskType>>>,
}
