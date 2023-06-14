use crate::internal::ast::package::ZPackage;
use crate::internal::compiler::symbol_scope::ModelScope;
use crate::internal::model::package::package_from_file;

use std::path::Path;
use walkdir::WalkDir;

pub mod package;

pub struct Model {
    pub packages: Vec<ZPackage>,
}

impl Model {
    /// Loads a complete zserio model from a directory.
    pub fn from_filesystem(directory: &Path) -> Self {
        let mut packages = Vec::new();

        for entry in WalkDir::new(directory) {
            let path = entry.unwrap().path().to_owned();
            if path.is_dir() {
                continue;
            }
            let mut file_name: String = path.file_name().unwrap().to_str().unwrap().into();
            file_name = file_name.to_ascii_lowercase();
            if file_name.ends_with(".zs") || file_name.ends_with(".zserio") {
                let package = package_from_file(&path);
                packages.push(package);
            }
        }
        Model { packages }
    }

    pub fn evaluate(&mut self) {
        // collect symbols
        let mut_self = self;
        let scope = ModelScope::build_scope(mut_self);

        // resolve types

        // evaluate templates

        // evaluate expressions
        mut_self.evaluate_structs(&scope);
    }

    pub fn evaluate_structs(&mut self, scope: &ModelScope) {
        for pkg in &mut self.packages {
            for z_struct in &mut pkg.structs {
                z_struct.borrow_mut().evaluate(scope);
            }

            for z_enum in &mut pkg.enums {
                z_enum.borrow_mut().evaluate(scope);
            }
        }
    }
}
