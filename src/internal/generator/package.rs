use crate::internal::ast::package::ZPackage;
use crate::internal::compiler::symbol_scope::ModelScope;
use crate::internal::generator::file_generator::write_to_file;
use crate::internal::generator::{
    subtype::generate_subtype, types::to_rust_module_name, types::TypeGenerator,
    zchoice::generate_choice, zenum::generate_enum, zstruct::generate_struct,
    zunion::generate_union,
};
use codegen::Scope;
use std::path::Path;

pub fn generate_package(
    symbol_scope: &ModelScope,
    package: &ZPackage,
    package_directory: &Path,
    root_package: &str,
) {
    let package_name = to_rust_module_name(&package.name);
    let mut module_names = Vec::new();
    let type_generator = TypeGenerator {
        root_package_name: root_package.to_owned(),
    };

    // Generate  the rust code for zserio structures.
    for z_struct_ref_cell in package.structs.values() {
        let z_struct = z_struct_ref_cell.borrow();
        // ignore templates, only generate code for instantiated structs
        if !z_struct.template_parameters.is_empty() {
            continue;
        }
        let mut gen_scope = Scope::new();
        module_names.push(generate_struct(
            symbol_scope,
            &type_generator,
            &mut gen_scope,
            &z_struct,
            package_directory,
            &package_name,
        ));
    }

    // Generate  the rust code for zserio Choices.
    for z_choice_ref_cell in package.zchoices.values() {
        let z_choice = z_choice_ref_cell.borrow();
        // Ignore templates, only generate code for instantiated choices.
        if !z_choice.template_parameters.is_empty() {
            continue;
        }
        let mut gen_scope = Scope::new();
        module_names.push(generate_choice(
            symbol_scope,
            &type_generator,
            &mut gen_scope,
            &z_choice,
            package_directory,
            &package_name,
        ));
    }

    // Generate  the rust code for zserio Union types.
    for zunion_ref_cell in &package.zunions {
        let zunion = zunion_ref_cell.borrow();
        // Ignore templates, only generate code for instantiated unions.
        if !zunion.template_parameters.is_empty() {
            continue;
        }
        let mut gen_scope = Scope::new();
        module_names.push(generate_union(
            symbol_scope,
            &type_generator,
            &mut gen_scope,
            &zunion,
            package_directory,
            &package_name,
        ));
    }

    // Generate the rust code for Enums.
    for z_enum_ref_cell in &package.enums {
        let z_enum = z_enum_ref_cell.borrow();
        let mut gen_scope = Scope::new();
        module_names.push(generate_enum(
            symbol_scope,
            &type_generator,
            &mut gen_scope,
            &z_enum,
            package_directory,
            &package_name,
        ));
    }

    // Generate the subtypes
    for zsubtype_ref in &package.subtypes {
        let zsubtype = zsubtype_ref.borrow();
        // Ignore templated subtypes
        if !zsubtype.zserio_type.template_arguments.is_empty() {
            continue;
        }
        let mut codegen_scope = Scope::new();
        module_names.push(generate_subtype(
            &mut codegen_scope,
            &type_generator,
            &zsubtype,
            package_directory,
            &package_name,
        ));
    }

    // Finally, generate the mod file for each package.
    // For now, this is using raw string concatination, as codegen does not support
    // module declarations.
    let mut mod_file_content = String::from("");

    // https://github.com/Danaozhong/rust-zserio/issues/28
    // The generated code does not pass clippy format checks.
    // Fixing these is moved to later, once the generated code works correctly.
    mod_file_content += "#![allow(clippy::all)]\n";
    mod_file_content += "#![allow(warnings)]\n";

    for module_name in module_names {
        mod_file_content += format!("pub mod {};\n", module_name).as_str();
    }

    write_to_file(&mod_file_content, package_directory, &package_name, "mod");
}
