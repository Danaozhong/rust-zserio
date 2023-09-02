use crate::internal::ast::zsubtype::Subtype;
use crate::internal::generator::file_generator::write_to_file;
use crate::internal::generator::preamble::add_standard_imports;
use crate::internal::generator::types::{to_rust_module_name, to_rust_type_name, TypeGenerator};
use codegen::Scope;
use std::path::Path;

pub fn generate_subtype(
    codegen_scope: &mut Scope,
    type_generator: &TypeGenerator,
    subtype: &Subtype,
    path: &Path,
    package_name: &str,
) -> String {
    add_standard_imports(codegen_scope);

    let rust_module_name = to_rust_module_name(&subtype.name);
    let type_alias_scope = codegen_scope.new_type_alias(
        to_rust_type_name(&subtype.name),
        &type_generator.ztype_to_rust_type(&subtype.zserio_type),
    );
    type_alias_scope.vis("pub");

    write_to_file(
        &codegen_scope.to_string(),
        path,
        package_name,
        &rust_module_name,
    );
    rust_module_name
}
