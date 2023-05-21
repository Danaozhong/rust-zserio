use crate::internal::generator::file_generator::write_to_file;


use crate::internal::ast::package::ZPackage;
use crate::internal::generator::{
    preamble::get_default_scope, zenum::generate_enum, zstruct::generate_struct,
};
use std::path::Path;

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
        let mut scope = get_default_scope(package);
        generate_enum(&mut scope, &z_enum, package_directory, &package.name);
        module_names.push(&z_enum.name);
    }

    // finally, generate the mod file
    // for now, this is using raw string concatination, as codegen does not support
    // module declarations.
    let mut mod_file_content = String::from("");
    for module_name in module_names {
        mod_file_content += format!("pub mod {};\n", module_name.as_str()).as_str();
    }
    write_to_file(&mod_file_content, package_directory, &package.name, "mod");
}
