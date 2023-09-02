use crate::internal::ast::zfunction::ZFunction;
use crate::internal::generator::expression::generate_expression;
use crate::internal::generator::types::{convert_to_function_name, TypeGenerator};
use codegen::Impl;

pub fn generate_function(
    codegen_scope: &mut Impl,
    type_generator: &TypeGenerator,
    zserio_function: &ZFunction,
) {
    // Generates a rust function for a zserio function.

    // Generate the function (empty for now), using the return type, name, and self-reference.
    let fn_gen_scope =
        codegen_scope.new_fn(convert_to_function_name(&zserio_function.name).as_str());
    fn_gen_scope.ret(
        type_generator
            .ztype_to_rust_type(&zserio_function.return_type)
            .as_str(),
    );
    fn_gen_scope.arg_ref_self();
    fn_gen_scope.vis("pub");

    // Generate the function content.
    fn_gen_scope.line(format!(
        "({}) as {}",
        generate_expression(&zserio_function.result.as_ref().borrow(), type_generator),
        type_generator.ztype_to_rust_type(&zserio_function.return_type),
    ));
}
