use crate::internal::ast::zsubtype::Subtype;
use crate::internal::generator::file_generator::write_to_file;
use crate::internal::generator::types::{
    custom_type_to_rust_type, to_rust_module_name, to_rust_type_name,
};
use codegen::Scope;
use std::path::Path;

pub fn generate_subtype(
    scope: &mut Scope,
    subtype: &Subtype,
    path: &Path,
    package_name: &str,
) -> String {
    let rust_module_name = to_rust_module_name(&subtype.name);
    let type_alias_scope = scope.new_type_alias(
        to_rust_type_name(&subtype.name),
        &custom_type_to_rust_type(&subtype.zserio_type.name),
    );
    type_alias_scope.vis("pub");

    write_to_file(&scope.to_string(), path, package_name, &rust_module_name);
    rust_module_name
}
