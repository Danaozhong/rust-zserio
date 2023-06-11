use crate::internal::ast::{zconst::ZConst, zenum::ZEnum, zstruct::ZStruct, zsubtype::Subtype};

use std::collections::HashMap;
use std::rc::Rc;

/// This enum is used to cover any possible type of symbols.
#[derive(Clone)]
pub enum Symbol {
    Struct(Rc<ZStruct>),
    Enum(Rc<ZEnum>),
    Subtype(Rc<Subtype>),
    Const(Rc<ZConst>),
}
/// A struct to hold a reference to a specific symbol. It provides the package, symbol and symbol name.
pub struct SymbolReference {
    pub symbol: Symbol,
    pub name: String,
    pub package: String,
}
/// The symbol scope contains a list of all existing symbols.
pub struct SymbolScope {
    /// These are symbols valid within a struct/function/choice, but not outside of it.
    pub local_symbols: HashMap<String, HashMap<String, Symbol>>,

    // These are symbols within the current file.
    pub file_symbols: HashMap<String, Symbol>,

    // These symbols are defined in other files.
    pub other_symbols: HashMap<String, Symbol>,
}

impl SymbolScope {
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
