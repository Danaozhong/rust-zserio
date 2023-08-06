use crate::internal::generator::package::generate_package;
use crate::internal::model::Model;
use std::path::Path;

pub fn generate_model(model: &Model, target_directory: &Path, _root_package: &str) {
    for package in model.packages.values() {
        generate_package(package, target_directory);
    }
}
