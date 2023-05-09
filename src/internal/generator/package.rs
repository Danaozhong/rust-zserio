
use codegen::Scope;
use crate::internal::generator::file_generator::write_to_file;

use crate::internal::ast::package::ZPackage;
use crate::internal::generator::{
    zstruct::generate_struct,
    zenum::generate_enum,
    preamble::get_default_scope,
};
use std::path::{Path};

pub fn generate_package(package: &ZPackage, package_directory: &Path) {
    let mut module_names = Vec::new();
    
    // Generate  the rust code for structures, ...
    for z_struct in &package.structs {
        // ignore templates, only generate code for instantiated structs
        if z_struct.template_params.len() > 0 {
            continue;
        }
        let mut scope = get_default_scope(package);
        generate_struct(&mut scope, z_struct, package_directory, &package.name);
        module_names.push(&z_struct.name);
    }

    // and for zserio enumerations
    for z_enum in &package.enums {
        generate_enum(&z_enum, package_directory, &package.name);
        module_names.push(&z_enum.name);
    }

    // finally, generate the mod file
    /*
    let mut scope = Scope::new();
    for module_name in module_names {
        let module = scope.new_module(module_name);
        module.vis("pub");
    }
    write_to_file(
        &scope.to_string(), 
        package_directory,
        &package.name,
        "mod",
    );
     */
}