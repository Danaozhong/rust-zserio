use crate::internal::generator::mod_file::generate_top_level_mod_file;
use crate::internal::generator::package::generate_package;
use crate::internal::model::Model;
use std::path::Path;

pub fn generate_model(model: &mut Model, package_directory: &Path, root_package: &str) {
    let scope = &mut model.scope;
    // Generate the rust interface files for all packages.
    for package in model.packages.values() {
        generate_package(scope, package, package_directory, root_package);
    }

    // Generate the overall mod file.
    generate_top_level_mod_file(model, package_directory, root_package);
}
