use crate::internal::{
    ast::{package::ZPackage, zconst::ZConst, zenum::ZEnum, zstruct::ZStruct, zsubtype::Subtype},
    model::Model,
};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// This enum is used to cover any possible type of symbols.
#[derive(Clone)]
pub enum Symbol {
    Struct(Rc<RefCell<ZStruct>>),
    Enum(Rc<RefCell<ZEnum>>),
    Subtype(Rc<RefCell<Subtype>>),
    Const(Rc<RefCell<ZConst>>),
}
/// A struct to hold a reference to a specific symbol. It provides the package, symbol and symbol name.
pub struct SymbolReference {
    pub symbol: Symbol,
    pub name: String,
    pub package: String,
}
/// The symbol scope contains a list of all existing symbols.
pub struct PackageScope {
    /// These are symbols valid within a struct/function/choice, but not outside of it.
    pub local_symbols: HashMap<String, HashMap<String, Symbol>>,

    // These are symbols within the current file.
    pub file_symbols: HashMap<String, Symbol>,

    // These symbols are defined in other files.
    pub other_symbols: HashMap<String, Symbol>,
}

pub struct ModelScope {
    pub package_scopes: HashMap<String, PackageScope>,
}

impl ModelScope {
    pub fn build_scope(model: &Model) -> Self {
        let mut scope = ModelScope {
            package_scopes: HashMap::new(),
        };
        for package in &model.packages {
            scope
                .package_scopes
                .insert(package.name.clone(), PackageScope::build_scope(package));
        }
        scope
    }
}

impl PackageScope {
    pub fn build_scope(package: &ZPackage) -> Self {
        let mut scope = PackageScope {
            local_symbols: HashMap::new(),
            file_symbols: HashMap::new(),
            other_symbols: HashMap::new(),
        };
        for zstruct in &package.structs {
            scope.file_symbols.insert(
                zstruct.borrow().name.clone(),
                Symbol::Struct(zstruct.clone()),
            );
        }
        for zenum in &package.enums {
            scope
                .file_symbols
                .insert(zenum.borrow().name.clone(), Symbol::Enum(zenum.clone()));
        }

        scope
    }

    /// Searches for a symbol within the current scope.
    pub fn resolve_symbol(
        &self,
        name: &String,
        optional_local_scope: Option<String>,
    ) -> SymbolReference {
        // first, search for the symbol in the local scope (if provided).
        if let Some(local_scope) = &optional_local_scope {
            match self.local_symbols.get(local_scope) {
                Some(symbol_scope) => match symbol_scope.get(name) {
                    Some(symbol) => {
                        return SymbolReference {
                            symbol: symbol.clone(),
                            name: "".into(),
                            package: "".into(),
                        };
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        // second, search in the package scope
        // TODO

        // third, search in other packages
        // TODO

        panic!("not implemented");
    }
}
