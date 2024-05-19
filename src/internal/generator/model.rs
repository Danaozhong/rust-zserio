use crate::internal::generator::mod_file::generate_top_level_mod_file;
use crate::internal::generator::package::generate_package;
use crate::internal::generator::types::TypeGenerator;

use crate::internal::model::Model;
use std::path::Path;

pub fn generate_model(model: &mut Model, package_directory: &Path, root_package: &str) {
    let mut type_generator = TypeGenerator::new(root_package.to_owned());

    let scope = &mut model.scope;
    // Generate the rust interface files for all packages.
    for package in model.packages.values() {
        println!("generating rust code for package {0}...", package.name);
        generate_package(&mut type_generator, scope, package, package_directory);
    }

    // Generate the overall mod file.
    generate_top_level_mod_file(&mut type_generator, model, package_directory, root_package);
}
