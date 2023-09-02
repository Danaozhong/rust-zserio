use crate::internal::ast::zconst::ZConst;
use crate::internal::generator::expression::generate_expression;
use crate::internal::generator::file_generator::write_to_file;
use crate::internal::generator::preamble::add_standard_imports;
use crate::internal::generator::{
    types::to_rust_constant_name, types::to_rust_module_name, types::TypeGenerator,
};
use std::path::Path;

use codegen::Scope;

pub fn generate_constant(
    codegen_scope: &mut Scope,
    type_generator: &TypeGenerator,
    zconst: &ZConst,
    path: &Path,
    package_name: &str,
) -> String {
    let rust_module_name = to_rust_module_name(&zconst.name);

    add_standard_imports(codegen_scope);

    // Constants are not yet supported by the code generator - generate them manually.
    let mut file_content = codegen_scope.to_string();

    // Generate the constants
    let field_type = type_generator.ztype_to_rust_type(zconst.zserio_type.as_ref());

    file_content += format!(
        "pub const {}: {} = {};\n",
        to_rust_constant_name(&zconst.name),
        field_type,
        generate_expression(&zconst.value_expression.borrow(), type_generator),
    )
    .as_str();

    write_to_file(&file_content, path, package_name, &rust_module_name);
    rust_module_name
}
