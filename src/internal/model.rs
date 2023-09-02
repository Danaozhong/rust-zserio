use crate::internal::ast::package::ZPackage;
use crate::internal::compiler::resolve_types::{
    resolve_choice_types, resolve_struct_types, resolve_subtype, resolve_union_types,
};
use crate::internal::compiler::symbol_scope::ModelScope;
use crate::internal::model::package::package_from_file;
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;

use super::compiler::symbol_scope::ScopeLocation;
pub mod package;
use crate::internal::compiler::template_instantiation::instantiate_type;

pub struct Model {
    pub packages: HashMap<String, ZPackage>,
    pub scope: ModelScope,
}

impl Model {
    /// Loads a complete zserio model from a directory.
    pub fn from_filesystem(directory: &Path) -> Self {
        let mut packages = HashMap::new();

        for entry in WalkDir::new(directory) {
            let path = entry.unwrap().path().to_owned();
            if path.is_dir() {
                continue;
            }
            let mut file_name: String = path.file_name().unwrap().to_str().unwrap().into();
            file_name = file_name.to_ascii_lowercase();
            if file_name.ends_with(".zs") || file_name.ends_with(".zserio") {
                let package = package_from_file(&path);
                packages.insert(package.name.clone(), package);
            }
        }
        Model {
            packages,
            scope: ModelScope {
                package_scopes: HashMap::new(),
                scope_stack: vec![],
            },
        }
    }

    pub fn evaluate(&mut self) {
        self.scope = ModelScope::build_scope(&self.packages);

        // evaluate templates
        self.instantiate_templates();

        // resolve types
        self.resolve_types();

        // evaluate expressions
        self.evaluate_package();
    }

    pub fn resolve_types(&mut self) {
        let scope = &mut self.scope;
        for pkg in self.packages.values_mut() {
            for z_struct in pkg.structs.values_mut() {
                scope.scope_stack.push(ScopeLocation {
                    package: pkg.name.clone(),
                    import_symbol: None,
                    symbol_name: Option::from(z_struct.borrow().name.clone()),
                });
                resolve_struct_types(&z_struct.borrow(), scope);
                scope.scope_stack.pop();
            }

            for z_choice in pkg.zchoices.values_mut() {
                scope.scope_stack.push(ScopeLocation {
                    package: pkg.name.clone(),
                    import_symbol: None,
                    symbol_name: Option::from(z_choice.borrow().name.clone()),
                });
                resolve_choice_types(&z_choice.borrow(), scope);
                scope.scope_stack.pop();
            }

            for z_union in &pkg.zunions {
                scope.scope_stack.push(ScopeLocation {
                    package: pkg.name.clone(),
                    import_symbol: None,
                    symbol_name: Option::from(z_union.borrow().name.clone()),
                });
                resolve_union_types(&z_union.borrow_mut(), scope);
                scope.scope_stack.pop();
            }

            for z_subtype in &pkg.subtypes {
                scope.scope_stack.push(ScopeLocation {
                    package: pkg.name.clone(),
                    import_symbol: None,
                    symbol_name: Option::from(z_subtype.borrow().name.clone()),
                });
                resolve_subtype(&mut z_subtype.borrow_mut(), scope);
                scope.scope_stack.pop();
            }
        }
    }

    pub fn instantiate_templates(&mut self) {
        let scope = &mut self.scope;
        for pkg in self.packages.values_mut() {
            scope.scope_stack.push(ScopeLocation {
                package: pkg.name.clone(),
                import_symbol: None,
                symbol_name: None,
            });

            // Generate template instantiations using the "instantiate" keyword.
            // Need to clone "instantiated_types", so that the package (pkg) can
            // be modified.
            let type_instantiations = pkg.instantiated_types.clone();
            for rc_instantiation in &type_instantiations {
                let instantiation = rc_instantiation.as_ref().borrow_mut();
                instantiate_type(
                    pkg,
                    scope,
                    &instantiation.zserio_type.clone(),
                    &instantiation.name.clone(),
                );
            }

            // Generate template instantiations using the "subtype" keyword.
            let subtypes = pkg.subtypes.clone();
            for rc_subtype in subtypes {
                let subtype = rc_subtype.borrow();
                if !subtype.zserio_type.template_arguments.is_empty() {
                    instantiate_type(
                        pkg,
                        scope,
                        &subtype.zserio_type.clone(),
                        &subtype.name.clone(),
                    );
                }
            }
            scope.scope_stack.pop();
        }
    }

    pub fn evaluate_package(&mut self) {
        let scope = &mut self.scope;
        for pkg in self.packages.values_mut() {
            for z_struct in pkg.structs.values_mut() {
                scope.scope_stack.push(ScopeLocation {
                    package: pkg.name.clone(),
                    import_symbol: None,
                    symbol_name: Option::from(z_struct.borrow().name.clone()),
                });
                z_struct.borrow().evaluate(scope);
                scope.scope_stack.pop();
            }

            for z_enum in &pkg.enums {
                scope.scope_stack.push(ScopeLocation {
                    package: pkg.name.clone(),
                    import_symbol: None,
                    symbol_name: Option::from(z_enum.borrow().name.clone()),
                });
                z_enum.borrow_mut().evaluate(scope);
                scope.scope_stack.pop();
            }

            for z_choice in pkg.zchoices.values_mut() {
                scope.scope_stack.push(ScopeLocation {
                    package: pkg.name.clone(),
                    import_symbol: None,
                    symbol_name: Option::from(z_choice.borrow().name.clone()),
                });
                z_choice.borrow().evaluate_selector_expression(scope);
                z_choice
                    .borrow_mut()
                    .add_enumeration_type_prefix_to_choice_cases();
                z_choice.borrow().evaluate(scope);

                scope.scope_stack.pop();
            }

            for z_union in &pkg.zunions {
                scope.scope_stack.push(ScopeLocation {
                    package: pkg.name.clone(),
                    import_symbol: None,
                    symbol_name: Option::from(z_union.borrow().name.clone()),
                });
                z_union.borrow().evaluate(scope);
                scope.scope_stack.pop();
            }

            for z_const in &mut pkg.consts {
                scope.scope_stack.push(ScopeLocation {
                    package: pkg.name.clone(),
                    import_symbol: None,
                    symbol_name: Option::from(z_const.borrow().name.clone()),
                });
                z_const.borrow_mut().evaluate(scope);
                scope.scope_stack.pop();
            }

            for bitmask in &mut pkg.bitmask_types {
                scope.scope_stack.push(ScopeLocation {
                    package: pkg.name.clone(),
                    import_symbol: None,
                    symbol_name: Option::from(bitmask.borrow().name.clone()),
                });
                bitmask.borrow_mut().evaluate(scope);
                scope.scope_stack.pop();
            }
        }
    }
}
