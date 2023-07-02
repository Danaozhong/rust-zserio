use crate::internal::{
    ast::{
        package::{ZImport, ZPackage},
        zconst::ZConst,
        zenum::{add_enum_to_scope, ZEnum},
        zstruct::add_struct_to_scope,
        zstruct::ZStruct,
        zsubtype::Subtype,
    },
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
    EnumItem(Rc<RefCell<ZEnum>>, usize),
    Subtype(Rc<RefCell<Subtype>>),
    Const(Rc<RefCell<ZConst>>),
    Field(Rc<RefCell<ZStruct>>, usize),
    Parameter(Rc<RefCell<ZStruct>>, usize),
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
    pub imports: Vec<ZImport>,
}

pub struct ScopeLocation {
    pub package: String,
    pub import_symbol: Option<String>,
    pub symbol_name: Option<String>,
}
/// ModelScope contains all scopes in the entire zserio model.
pub struct ModelScope {
    /// The scopes for each package, ordered by package name. The package
    /// name includes the full path.
    pub package_scopes: HashMap<String, PackageScope>,

    /// The area of the scope which is currently used. It contains inforamtion
    /// in which package to search, and which symbol.
    /// The scope is organized as a stack: at some point during evaluation, the
    /// scope may shift to another module.
    pub scope_stack: Vec<ScopeLocation>,
}

impl ModelScope {
    pub fn build_scope(model: &Model) -> Self {
        let mut scope = ModelScope {
            package_scopes: HashMap::new(),
            scope_stack: vec![],
        };
        for package in &model.packages {
            scope
                .package_scopes
                .insert(package.name.clone(), PackageScope::build_scope(package));
        }
        scope
    }

    pub fn resolve_symbol(&self, name: &String) -> SymbolReference {
        let evaluation_scope = self.scope_stack.last().unwrap();
        let package_scope = self.package_scopes.get(&evaluation_scope.package).unwrap();

        // Try if the symbol can be found in the current package.
        match package_scope.resolve_symbol(name, evaluation_scope) {
            Some(symbol) => return symbol,
            _ => (),
        }

        // Try to find the symbol in the imported packages.
        let mut imports_to_process = vec![];
        for reference in package_scope.imports.iter() {
            imports_to_process.push(reference);
        }

        while imports_to_process.len() > 0 {
            let mut new_imports_to_process = vec![];

            for import in imports_to_process {
                let package_name = import.package_dir.join(".");
                let imported_package = self.package_scopes.get(&package_name).unwrap();
                match imported_package.resolve_symbol(
                    name,
                    &ScopeLocation {
                        package: package_name,
                        import_symbol: import.symbol_name.clone(),
                        symbol_name: None,
                    },
                ) {
                    Some(symbol) => return symbol,
                    _ => (),
                }

                // If the symbol was not found in one of the current imports,
                // it may be located in an import of the current import. Keep
                // track of them to process them later.
                new_imports_to_process.extend(&imported_package.imports);
            }
            // If the symbol was not found, it may be found in an import of the
            // packages that were just processed.
            imports_to_process = new_imports_to_process;
        }
        // If this line is reached,
        panic!("symbol not found");
    }
}

impl PackageScope {
    pub fn build_scope(package: &ZPackage) -> Self {
        let mut scope = PackageScope {
            local_symbols: HashMap::new(),
            file_symbols: HashMap::new(),
            imports: package.imports.to_vec(),
        };

        for zstruct in &package.structs {
            add_struct_to_scope(zstruct, &mut scope);
        }
        for zenum in &package.enums {
            add_enum_to_scope(zenum, &mut scope);
        }

        scope
    }

    /// Searches for a symbol within the current scope.
    pub fn resolve_symbol(
        &self,
        name: &String,
        evaluation_scope: &ScopeLocation,
    ) -> Option<SymbolReference> {
        // first, search for the symbol within the current symbol (e.g. struct, enum, etc)
        if let Some(symbol_name) = &evaluation_scope.symbol_name {
            match self.local_symbols.get(symbol_name) {
                Some(symbol_scope) => match symbol_scope.get(name) {
                    Some(symbol) => {
                        return Option::from(SymbolReference {
                            symbol: symbol.clone(),
                            name: "".into(),
                            package: "".into(),
                        });
                    }
                    _ => (),
                },
                _ => panic!("a nonexisting symbol was referenced within the package"),
            }
        }

        // second, search in the package scope
        match self.file_symbols.get(name) {
            Some(symbol) => {
                return Option::from(SymbolReference {
                    symbol: symbol.clone(),
                    name: "".into(),
                    package: "".into(),
                })
            }
            _ => (),
        }
        // The symbol was not found in the current package.
        None
    }
}
