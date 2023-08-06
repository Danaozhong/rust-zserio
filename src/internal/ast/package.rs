use crate::internal::ast::{zenum::ZEnum, zstruct::ZStruct};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::string::String;

use super::{
    type_reference::InstantiateType, zbitmask::ZBitmaskType, zchoice::ZChoice, zconst::ZConst,
    zsubtype::Subtype, zunion::ZUnion,
};
#[derive(Clone)]
pub struct ZImport {
    pub package_dir: Vec<String>,

    /// The symbol name that is imported from the package. If the entire package is imported
    /// via "*", this will be empty.
    pub symbol_name: Option<String>,
}
pub struct ZPackage {
    pub name: String,
    pub comment: String,

    pub imports: Vec<ZImport>,
    pub structs: HashMap<String, Rc<RefCell<ZStruct>>>,
    pub zchoices: HashMap<String, Rc<RefCell<ZChoice>>>,
    pub zunions: Vec<Rc<RefCell<ZUnion>>>,
    pub enums: Vec<Rc<RefCell<ZEnum>>>,
    pub consts: Vec<Rc<RefCell<ZConst>>>,
    pub subtypes: Vec<Rc<RefCell<Subtype>>>,
    pub instantiated_types: Vec<Rc<RefCell<InstantiateType>>>,
    pub bitmask_types: Vec<Rc<RefCell<ZBitmaskType>>>,
}
