use crate::internal::generator::package::generate_package;
use crate::internal::model::model::Model;
use std::path::Path;

struct GeneratorConfig {
    root_path: String,
}

pub fn generate_model(model: &Model, target_directory: &Path, _root_package: &str) {
    for package in &model.packages {
        generate_package(package, target_directory);
    }
}
