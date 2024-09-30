use crate::internal::ast::package::ZPackage;
use crate::internal::compiler::symbol_scope::ModelScope;
use crate::internal::generator::file_generator::write_to_file;
use crate::internal::generator::{
    subtype::generate_subtype, types::TypeGenerator, zbitmask::generate_bitmask,
    zchoice::generate_choice, zconst::generate_constant, zenum::generate_enum,
    zstruct::generate_struct, zunion::generate_union,
};
use codegen::Scope;
use std::path::Path;

pub fn generate_package(
    type_generator: &mut TypeGenerator,
    symbol_scope: &ModelScope,
    package: &ZPackage,
    package_directory: &Path,
) {
    let package_name = &package.name;
    let mut module_names = Vec::new();

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
            type_generator,
            &mut gen_scope,
            &z_struct,
            package_directory,
            package_name,
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
            type_generator,
            &mut gen_scope,
            &z_choice,
            package_directory,
            package_name,
        ));
    }

    // Generate  the rust code for zserio Union types.
    for zunion_ref_cell in package.zunions.values() {
        let zunion = zunion_ref_cell.borrow();
        // Ignore templates, only generate code for instantiated unions.
        if !zunion.template_parameters.is_empty() {
            continue;
        }
        let mut gen_scope = Scope::new();
        module_names.push(generate_union(
            symbol_scope,
            type_generator,
            &mut gen_scope,
            &zunion,
            package_directory,
            package_name,
        ));
    }

    // Generate the rust code for Enums.
    for z_enum_ref_cell in &package.enums {
        let z_enum = z_enum_ref_cell.borrow();
        let mut gen_scope = Scope::new();
        module_names.push(generate_enum(
            symbol_scope,
            type_generator,
            &mut gen_scope,
            &z_enum,
            package_directory,
            package_name,
        ));
    }

    // Generate the rust code for bitmask types.
    for zbitmask_ref in &package.bitmask_types {
        let zbitmask = zbitmask_ref.borrow();
        let mut gen_scope = Scope::new();
        module_names.push(generate_bitmask(
            symbol_scope,
            type_generator,
            &mut gen_scope,
            &zbitmask,
            package_directory,
            package_name,
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
            type_generator,
            &zsubtype,
            package_directory,
            package_name,
        ));
    }

    // Generate the constant
    for zconst_ref in &package.consts {
        let zconst = zconst_ref.borrow();
        let mut codegen_scope = Scope::new();
        module_names.push(generate_constant(
            symbol_scope,
            type_generator,
            &mut codegen_scope,
            &zconst,
            package_directory,
            package_name,
        ));
    }

    // Finally, generate the mod file for each package.
    // For now, this is using raw string concatenation, as codegen does not support
    // module declarations.
    let mut mod_file_content = String::from("");

    mod_file_content += "#![allow(unused_imports)]\n";
    mod_file_content += "#![allow(unused_mut)]\n";
    mod_file_content += "#![allow(unused_parens)]\n";
    mod_file_content += "#![allow(unused_variables)]\n";
    mod_file_content += "#![allow(unused_assignments)]\n";
    mod_file_content += "#![allow(unreachable_patterns)]\n";
    mod_file_content += "#![allow(clippy::clone_on_copy)]\n";
    mod_file_content += "#![allow(clippy::field_reassign_with_default)]\n";
    mod_file_content += "#![allow(clippy::needless_borrow)]\n";
    mod_file_content += "#![allow(clippy::format_in_format_args)]\n";
    mod_file_content += "#![allow(clippy::needless_update)]\n";
    mod_file_content += "#![allow(clippy::double_parens)]\n";
    // In many places we cast the result of a generated expression, which could
    // result in casting to the same type or not using "###_type" notation for
    // numeric literals.
    mod_file_content += "#![allow(clippy::unnecessary_cast)]\n";
    // We have no control over zserio package names people use, so do not
    // complain about generated module names.
    mod_file_content += "#![allow(clippy::module_inception)]\n";

    // Sort module names so our output is in a predictable order.
    module_names.sort_unstable();
    for module_name in &module_names {
        mod_file_content += format!("pub mod {module_name};\n").as_str();
    }

    for module_name in &module_names {
        mod_file_content += format!("pub use {module_name}::*;\n").as_str();
    }

    write_to_file(
        type_generator,
        &mod_file_content,
        package_directory,
        package_name,
        "mod",
    );
}
