use crate::internal::generator::package::generate_package;
use crate::internal::model::Model;
use std::path::Path;

pub fn generate_model(model: &mut Model, target_directory: &Path, root_package: &String) {
    let scope = &mut model.scope;
    for package in model.packages.values() {
        generate_package(scope, package, target_directory, root_package);
    }
}
