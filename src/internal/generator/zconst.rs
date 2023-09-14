use crate::internal::ast::zconst::ZConst;
use crate::internal::compiler::fundamental_type::get_fundamental_type;
use crate::internal::compiler::symbol_scope::ModelScope;
use crate::internal::generator::expression::generate_expression;
use crate::internal::generator::file_generator::write_to_file;
use crate::internal::generator::preamble::add_standard_imports;
use crate::internal::generator::{
    types::to_rust_constant_name, types::to_rust_module_name, types::TypeGenerator,
};
use std::path::Path;

use codegen::Scope;

pub fn generate_constant(
    symbol_scope: &ModelScope,
    type_generator: &TypeGenerator,
    codegen_scope: &mut Scope,
    zconst: &ZConst,
    path: &Path,
    package_name: &str,
) -> String {
    let rust_module_name = to_rust_module_name(&zconst.name);

    add_standard_imports(codegen_scope);

    // Constants are not yet supported by the code generator - generate them manually.
    let mut file_content = codegen_scope.to_string();

    // Generate the constants using the fundamental type. Consts can only use native types.
    let fundamental_type = get_fundamental_type(zconst.zserio_type.as_ref(), symbol_scope);
    assert!(
        fundamental_type.fundamental_type.is_builtin,
        "const types need to be fundamental zserio types, but found {:?}",
        fundamental_type.fundamental_type
    );

    let mut field_type = type_generator.ztype_to_rust_type(&fundamental_type.fundamental_type);

    // Workaround for string types: rust does not allow consts of String type,
    // it only supports &str constants.
    if field_type == "String" {
        field_type = "&str".to_owned();
    }

    file_content += format!(
        "pub const {}: {} = {};\n",
        to_rust_constant_name(&zconst.name),
        field_type,
        generate_expression(
            &zconst.value_expression.borrow(),
            type_generator,
            symbol_scope
        ),
    )
    .as_str();

    write_to_file(&file_content, path, package_name, &rust_module_name);
    rust_module_name
}
