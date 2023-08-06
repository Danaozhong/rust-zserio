use crate::internal::ast::package::ZPackage;
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
        Model { packages }
    }

    pub fn evaluate(&mut self) {
        // collect symbols
        let mut_self = self;
        let mut scope = ModelScope::build_scope(mut_self);

        // resolve types

        // evaluate templates
        mut_self.instantiate_templates(&mut scope);

        // evaluate expressions
        mut_self.evaluate_package(&mut scope);
    }

    pub fn instantiate_templates(&mut self, scope: &mut ModelScope) {
        for pkg in self.packages.values_mut() {
            scope.scope_stack.push(ScopeLocation {
                package: pkg.name.clone(),
                import_symbol: None,
                symbol_name: None,
            });
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
            scope.scope_stack.pop();
        }
    }

    pub fn evaluate_package(&mut self, scope: &mut ModelScope) {
        for pkg in self.packages.values_mut() {
            for z_struct in pkg.structs.values_mut() {
                scope.scope_stack.push(ScopeLocation {
                    package: pkg.name.clone(),
                    import_symbol: None,
                    symbol_name: Option::from(z_struct.as_ref().borrow().name.clone()),
                });
                z_struct.borrow().evaluate(scope);
                scope.scope_stack.pop();
            }

            for z_enum in &pkg.enums {
                scope.scope_stack.push(ScopeLocation {
                    package: pkg.name.clone(),
                    import_symbol: None,
                    symbol_name: Option::from(z_enum.as_ref().borrow().name.clone()),
                });
                z_enum.borrow_mut().evaluate(scope);
                scope.scope_stack.pop();
            }

            for z_choice in pkg.zchoices.values_mut() {
                scope.scope_stack.push(ScopeLocation {
                    package: pkg.name.clone(),
                    import_symbol: None,
                    symbol_name: Option::from(z_choice.as_ref().borrow().name.clone()),
                });
                z_choice.borrow().evaluate_selector_expression(scope);
                z_choice
                    .borrow_mut()
                    .add_enumeration_type_prefix_to_choice_cases();
                z_choice.borrow().evaluate(scope);

                scope.scope_stack.pop();
            }

            for bitmask in &mut pkg.bitmask_types {
                scope.scope_stack.push(ScopeLocation {
                    package: pkg.name.clone(),
                    import_symbol: None,
                    symbol_name: Option::from(bitmask.as_ref().borrow().name.clone()),
                });
                bitmask.borrow_mut().evaluate(scope);
                scope.scope_stack.pop();
            }
        }
    }
}
