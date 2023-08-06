use crate::internal::generator::file_generator::write_to_file;

use crate::internal::ast::package::ZPackage;
use crate::internal::generator::{
    preamble::get_default_scope, types::to_rust_module_name, zchoice::generate_choice,
    zenum::generate_enum, zstruct::generate_struct,
};
use std::path::Path;

pub fn generate_package(package: &ZPackage, package_directory: &Path) {
    let package_name = to_rust_module_name(&package.name);
    let mut module_names = Vec::new();

    // Generate  the rust code for zserio structures.
    for z_struct_ref_cell in package.structs.values() {
        let z_struct = z_struct_ref_cell.borrow();
        // ignore templates, only generate code for instantiated structs
        if !z_struct.template_parameters.is_empty() {
            continue;
        }
        let mut scope = get_default_scope(package);
        module_names.push(generate_struct(
            &mut scope,
            &z_struct,
            package_directory,
            &package_name,
        ));
    }

    // Generate  the rust code for zserio structures.
    for z_choice_ref_cell in package.zchoices.values() {
        let z_choice = z_choice_ref_cell.borrow();
        // Ignore templates, only generate code for instantiated choices.
        if !z_choice.template_parameters.is_empty() {
            continue;
        }
        let mut scope = get_default_scope(package);
        module_names.push(generate_choice(
            &mut scope,
            &z_choice,
            package_directory,
            &package_name,
        ));
    }

    // Generate the rust code for Enums.
    for z_enum_ref_cell in &package.enums {
        let z_enum = z_enum_ref_cell.borrow();
        let mut scope = get_default_scope(package);
        module_names.push(generate_enum(
            &mut scope,
            &z_enum,
            package_directory,
            &package_name,
        ));
    }

    // Finally, generate the mod file for each package.
    // For now, this is using raw string concatination, as codegen does not support
    // module declarations.
    let mut mod_file_content = String::from("");
    for module_name in module_names {
        mod_file_content += format!("pub mod {};\n", module_name).as_str();
    }
    write_to_file(&mod_file_content, package_directory, &package_name, "mod");
}
