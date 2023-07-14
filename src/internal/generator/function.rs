use crate::internal::ast::zfunction::ZFunction;
use crate::internal::generator::expression::generate_expression;
use crate::internal::generator::types::{convert_to_function_name, ztype_to_rust_type};
use codegen::Impl;

pub fn generate_function(codegen_scope: &mut Impl, zserio_function: &ZFunction) {
    // Generates a rust function for a zserio function.

    // Generate the function (empty for now), using the return type, name, and self-reference.
    let fn_gen_scope =
        codegen_scope.new_fn(convert_to_function_name(&zserio_function.name).as_str());
    fn_gen_scope.ret(ztype_to_rust_type(&zserio_function.return_type).as_str());
    fn_gen_scope.arg_ref_self();
    fn_gen_scope.vis("pub");

    // Generate the function content.
    fn_gen_scope.line(generate_expression(
        &zserio_function.result.as_ref().borrow(),
    ));
}
