use crate::internal::ast::{
    field::Field,
    package::{ZImport, ZPackage},
    parameter::Parameter,
    type_reference::TypeReference,
    zbitmask::{add_bitmask_to_scope, ZBitmaskType},
    zchoice::add_choice_to_scope,
    zchoice::ZChoice,
    zconst::{add_const_to_scope, ZConst},
    zenum::{add_enum_to_scope, ZEnum},
    zfunction::ZFunction,
    zstruct::add_struct_to_scope,
    zstruct::ZStruct,
    zsubtype::{add_subtype_to_scope, Subtype},
    zunion::{add_zunion_to_scope, ZUnion},
};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// This enum is used to cover any possible type of symbols.
#[derive(Clone, Debug)]
pub enum Symbol {
    Struct(Rc<RefCell<ZStruct>>),
    Choice(Rc<RefCell<ZChoice>>),
    Union(Rc<RefCell<ZUnion>>),
    Enum(Rc<RefCell<ZEnum>>),
    EnumItem(Rc<RefCell<ZEnum>>, usize),
    Subtype(Rc<RefCell<Subtype>>),
    Const(Rc<RefCell<ZConst>>),
    Bitmask(Rc<RefCell<ZBitmaskType>>),
    Field(Rc<RefCell<Field>>),
    Parameter(Rc<RefCell<Parameter>>),
    Function(Rc<RefCell<ZFunction>>),
}
/// A struct to hold a reference to a specific symbol. It provides the package, symbol and symbol name.
#[derive(Clone, Debug)]
pub struct SymbolReference {
    pub symbol: Symbol,
    pub name: String,
    pub package: String,
}

/// The symbol scope contains a list of all existing symbols.
pub struct PackageScope {
    pub name: String,

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
    pub fn build_scope(packages: &HashMap<String, ZPackage>) -> Self {
        let mut scope = ModelScope {
            package_scopes: HashMap::new(),
            scope_stack: vec![],
        };
        for package in packages.values() {
            scope
                .package_scopes
                .insert(package.name.clone(), PackageScope::build_scope(package));
        }
        scope
    }

    pub fn get_package_scope(&mut self) -> &mut PackageScope {
        let evaluation_scope = self.scope_stack.last().unwrap();
        self.package_scopes
            .get_mut(&evaluation_scope.package)
            .unwrap()
    }

    pub fn get_symbol(&self, type_ref: &TypeReference) -> SymbolReference {
        match self.package_scopes.get(&type_ref.package) {
            Some(package_scope) => package_scope.get_symbol(&type_ref.name),
            _ => panic!(""),
        }
    }

    /// This function resolves a symbol based on the imports in the current package.
    pub fn resolve_symbol(
        &self,
        name: &String,
        ignore_symbol_local_scopes: bool,
    ) -> SymbolReference {
        let evaluation_scope = self.scope_stack.last().unwrap();
        let package_scope = self.package_scopes.get(&evaluation_scope.package).unwrap();

        // Try if the symbol can be found in the current package.
        if let Some(symbol) =
            package_scope.resolve_symbol(name, ignore_symbol_local_scopes, evaluation_scope)
        {
            return symbol;
        }

        // Try to find the symbol in the imported packages.
        let mut imports_to_process = vec![];
        for reference in package_scope.imports.iter() {
            imports_to_process.push(reference);
        }

        while !imports_to_process.is_empty() {
            let mut new_imports_to_process = vec![];

            for import in imports_to_process {
                let package_name = import.package_dir.join(".");
                let imported_package = self.package_scopes.get(&package_name).unwrap();
                if let Some(symbol) = imported_package.resolve_symbol(
                    name,
                    ignore_symbol_local_scopes,
                    &ScopeLocation {
                        package: package_name,
                        import_symbol: import.symbol_name.clone(),
                        symbol_name: None,
                    },
                ) {
                    return symbol;
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
        // If this line is reached, the symbol lookup has failed, and the symbol
        // was not found anywhere.
        panic!("symbol not found: {:?}", name);
    }
}

impl PackageScope {
    pub fn build_scope(package: &ZPackage) -> Self {
        let mut scope = PackageScope {
            name: package.name.clone(),
            local_symbols: HashMap::new(),
            file_symbols: HashMap::new(),
            imports: package.imports.to_vec(),
        };

        for zconst in &package.consts {
            add_const_to_scope(zconst, &mut scope);
        }
        for zbitmask in &package.bitmask_types {
            add_bitmask_to_scope(zbitmask, &mut scope);
        }
        for zstruct in package.structs.values() {
            add_struct_to_scope(zstruct, &mut scope);
        }
        for zchoice in package.zchoices.values() {
            add_choice_to_scope(zchoice, &mut scope);
        }
        for zunion in package.zunions.values() {
            add_zunion_to_scope(zunion, &mut scope);
        }
        for zenum in &package.enums {
            add_enum_to_scope(zenum, &mut scope);
        }
        for subtype in &package.subtypes {
            add_subtype_to_scope(subtype, &mut scope)
        }

        scope
    }

    pub fn get_symbol(&self, name: &String) -> SymbolReference {
        match self.file_symbols.get(name) {
            Some(symbol) => SymbolReference {
                symbol: symbol.clone(),
                name: get_symbol_name(symbol),
                package: self.name.clone(),
            },
            _ => panic!(""),
        }
    }

    /// Searches for a symbol within the current scope.
    pub fn resolve_symbol(
        &self,
        name: &String,
        ignore_symbol_local_scopes: bool,
        evaluation_scope: &ScopeLocation,
    ) -> Option<SymbolReference> {
        if !ignore_symbol_local_scopes {
            // Unless explicitly ignored, search for the symbol within
            // the current symbol (e.g. struct, enum, etc)
            if let Some(symbol_name) = &evaluation_scope.symbol_name {
                if let Some(symbol_scope) = self.local_symbols.get(symbol_name) {
                    if let Some(symbol) = symbol_scope.get(name) {
                        return Option::from(SymbolReference {
                            symbol: symbol.clone(),
                            name: get_symbol_name(symbol),
                            package: self.name.clone(),
                        });
                    }
                }
            }
        }

        // second, search in the package scope
        if let Some(symbol) = self.file_symbols.get(name) {
            return Option::from(SymbolReference {
                symbol: symbol.clone(),
                name: get_symbol_name(symbol),
                package: self.name.clone(),
            });
        }
        // The symbol was not found in the current package.
        None
    }
}

fn get_symbol_name(symbol: &Symbol) -> String {
    return match symbol {
        Symbol::Struct(s) => s.as_ref().borrow().name.clone(),
        Symbol::Choice(c) => c.as_ref().borrow().name.clone(),
        Symbol::Subtype(s) => s.as_ref().borrow().name.clone(),
        Symbol::Enum(e) => e.as_ref().borrow().name.clone(),
        Symbol::Bitmask(bitmask) => bitmask.as_ref().borrow().name.clone(),
        _ => "".into(),
    };
}
