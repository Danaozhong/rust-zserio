#![allow(clippy::type_complexity)]

use codegen::{Function, Scope};

use crate::internal::ast::zchoice::ZChoice;

use crate::internal::compiler::symbol_scope::ModelScope;
use crate::internal::generator::{
    array::instantiate_zserio_array, bitsize::bitsize_field, decode::decode_field,
    encode::encode_field, expression::generate_expression, file_generator::write_to_file,
    function::generate_function, packed_contexts::generate_init_packed_context_for_field,
    packed_contexts::generate_packed_context_for_field, packed_contexts::FieldDetails,
    preamble::add_standard_imports, types::TypeGenerator,
    zstruct::generate_struct_member_for_field,
};

use std::path::Path;

pub fn generate_choice(
    symbol_scope: &ModelScope,
    type_generator: &mut TypeGenerator,
    codegen_scope: &mut Scope,
    zchoice: &ZChoice,
    path: &Path,
    package_name: &str,
) -> String {
    let rust_module_name = type_generator.to_rust_module_name(&zchoice.name);
    let rust_type_name = type_generator.to_rust_type_name(&zchoice.name);

    let mut field_details = vec![];
    let mut field_idx = 0;

    for case in &zchoice.cases {
        if let Some(field_rc) = &case.field {
            field_details.push(FieldDetails::from_field(
                field_rc,
                field_idx,
                symbol_scope,
                type_generator,
            ));
            field_idx += 1;
        }
    }
    if let Some(default_case) = &zchoice.default_case {
        if let Some(field_rc) = &default_case.field {
            field_details.push(FieldDetails::from_field(
                field_rc,
                field_idx,
                symbol_scope,
                type_generator,
            ));
        }
    }

    add_standard_imports(codegen_scope);

    // generate the ZChoice as a rust structure
    let gen_choice = codegen_scope.new_struct(&rust_type_name);
    gen_choice.vis("pub");
    gen_choice.derive("Debug");
    gen_choice.derive("Default");
    gen_choice.derive("Clone");
    gen_choice.derive("PartialEq");

    // if the field is parameterized, add the parameters as member variables
    for param in &zchoice.type_parameters {
        let param_type =
            type_generator.ztype_to_rust_type(param.as_ref().borrow().zserio_type.as_ref());
        // Possible improvement: currently the parameters are copied instead of taken the reference.
        // It would be great to change that to references to avoid unnecessary copying, but this is
        // painful in rust due to the lifetime checks.
        // Because I am lazy, this implementation will just copy values over.
        let gen_param_field = gen_choice.new_field(
            type_generator.convert_field_name(&param.as_ref().borrow().name),
            param_type,
        );
        gen_param_field.vis("pub");
    }

    // Add the data fields for each choice case field (including the default case) to the generated structure.
    for field in &field_details {
        generate_struct_member_for_field(gen_choice, field);
    }

    // generate the functions to serialize/deserialize
    let choice_impl = codegen_scope.new_impl(&rust_type_name);
    choice_impl.impl_trait("ztype::ZserioPackableObject");

    generate_zserio_read(symbol_scope, type_generator, choice_impl, zchoice);
    generate_zserio_write(symbol_scope, type_generator, choice_impl, zchoice);
    generate_zserio_bitsize(symbol_scope, type_generator, choice_impl, zchoice);

    // Generate the packed contexts.
    let create_packing_context_fn = choice_impl.new_fn("zserio_create_packing_context");
    create_packing_context_fn.arg("context_node", "&mut PackingContextNode");
    for field in &field_details {
        generate_packed_context_for_field(create_packing_context_fn, field);
    }

    let init_packing_context_fn = choice_impl.new_fn("zserio_init_packing_context");
    init_packing_context_fn.arg_ref_self();
    init_packing_context_fn.arg("context_node", "&mut PackingContextNode");
    init_packing_context_fn.ret("Result<()>");
    for field in &field_details {
        generate_init_packed_context_for_field(
            symbol_scope,
            type_generator,
            init_packing_context_fn,
            field,
        );
    }
    init_packing_context_fn.line("Ok(())");

    // Generate all the zserio functions.
    let pub_impl = codegen_scope.new_impl(&rust_type_name);
    for zserio_function in &zchoice.functions {
        generate_function(
            symbol_scope,
            pub_impl,
            type_generator,
            &zserio_function.as_ref().borrow(),
        );
    }

    write_to_file(
        type_generator,
        &codegen_scope.to_string(),
        path,
        package_name,
        &rust_module_name,
    );
    rust_module_name
}

pub fn generate_choice_match_construct(
    symbol_scope: &ModelScope,
    type_generator: &mut TypeGenerator,
    code_gen_fn: &mut Function,
    zchoice: &ZChoice,
    packed: bool,
    f: &dyn Fn(&ModelScope, &mut TypeGenerator, &mut Function, &FieldDetails, bool),
) {
    let selector_name =
        type_generator.convert_field_name(&zchoice.selector_expression.as_ref().borrow().text);
    let mut context_node_index = 0;

    code_gen_fn.line(format!("match self.{} {{", selector_name));

    for case in &zchoice.cases {
        let mut case_expressions = vec![];
        for case_expr in &case.conditions {
            case_expressions.push(generate_expression(
                &case_expr.as_ref().borrow(),
                type_generator,
                symbol_scope,
            ));
        }
        let selector_expression_evaluation = case_expressions.join(" | ");

        code_gen_fn.line(format!("{} => {{", selector_expression_evaluation));
        if let Some(field) = &case.field {
            let field_details =
                FieldDetails::from_field(field, context_node_index, symbol_scope, type_generator);
            instantiate_zserio_array(
                symbol_scope,
                type_generator,
                code_gen_fn,
                &field.borrow(),
                packed,
                field_details.is_packable,
            );
            context_node_index += 1;
            f(
                symbol_scope,
                type_generator,
                code_gen_fn,
                &field_details,
                packed,
            );
        }
        code_gen_fn.line("},");
    }
    if let Some(default_case) = &zchoice.default_case {
        code_gen_fn.line("_ => (");
        if let Some(field) = &default_case.field {
            let default_field =
                FieldDetails::from_field(field, context_node_index, symbol_scope, type_generator);
            instantiate_zserio_array(
                symbol_scope,
                type_generator,
                code_gen_fn,
                &field.borrow(),
                false,
                default_field.is_packable,
            );
            f(
                symbol_scope,
                type_generator,
                code_gen_fn,
                &default_field,
                packed,
            );
        }
        code_gen_fn.line("),");
    } else {
        code_gen_fn.line("_ => (),");
    }
    code_gen_fn.line("}");
}

fn generate_zserio_read(
    symbol_scope: &ModelScope,
    type_generator: &mut TypeGenerator,
    struct_impl: &mut codegen::Impl,
    choice: &ZChoice,
) {
    let zserio_read_fn = struct_impl.new_fn("zserio_read");
    zserio_read_fn.arg_mut_self();
    zserio_read_fn.arg("reader", "&mut BitReader");
    zserio_read_fn.ret("Result<()>");

    generate_choice_match_construct(
        symbol_scope,
        type_generator,
        zserio_read_fn,
        choice,
        false,
        &decode_field,
    );
    zserio_read_fn.line("Ok(())");

    let zserio_read_packed_fn = struct_impl.new_fn("zserio_read_packed");
    zserio_read_packed_fn.arg_mut_self();
    zserio_read_packed_fn.arg("context_node", "&mut PackingContextNode");
    zserio_read_packed_fn.arg("reader", "&mut BitReader");
    zserio_read_packed_fn.ret("Result<()>");
    generate_choice_match_construct(
        symbol_scope,
        type_generator,
        zserio_read_packed_fn,
        choice,
        true,
        &decode_field,
    );
    zserio_read_packed_fn.line("Ok(())");
}

fn generate_zserio_write(
    symbol_scope: &ModelScope,
    type_generator: &mut TypeGenerator,
    struct_impl: &mut codegen::Impl,
    choice: &ZChoice,
) {
    let zserio_write_fn = struct_impl.new_fn("zserio_write");
    zserio_write_fn.arg_ref_self();
    zserio_write_fn.arg("writer", "&mut BitWriter");
    zserio_write_fn.ret("Result<()>");
    generate_choice_match_construct(
        symbol_scope,
        type_generator,
        zserio_write_fn,
        choice,
        false,
        &encode_field,
    );
    zserio_write_fn.line("Ok(())");

    let zserio_write_packed_fn = struct_impl.new_fn("zserio_write_packed");
    zserio_write_packed_fn.arg_ref_self();
    zserio_write_packed_fn.arg("context_node", "&mut PackingContextNode");
    zserio_write_packed_fn.arg("writer", "&mut BitWriter");
    zserio_write_packed_fn.ret("Result<()>");
    generate_choice_match_construct(
        symbol_scope,
        type_generator,
        zserio_write_packed_fn,
        choice,
        true,
        &encode_field,
    );
    zserio_write_packed_fn.line("Ok(())");
}

fn generate_zserio_bitsize(
    symbol_scope: &ModelScope,
    type_generator: &mut TypeGenerator,
    struct_impl: &mut codegen::Impl,
    choice: &ZChoice,
) {
    let bitsize_fn = struct_impl.new_fn("zserio_bitsize");
    bitsize_fn.ret("Result<u64>");
    bitsize_fn.arg_ref_self();
    bitsize_fn.arg("bit_position", "u64");
    bitsize_fn.line("let mut end_position = bit_position;");
    generate_choice_match_construct(
        symbol_scope,
        type_generator,
        bitsize_fn,
        choice,
        false,
        &bitsize_field,
    );
    bitsize_fn.line("Ok(end_position - bit_position)");

    let bitsize_packed_fn = struct_impl.new_fn("zserio_bitsize_packed");
    bitsize_packed_fn.ret("Result<u64>");
    bitsize_packed_fn.arg_ref_self();
    bitsize_packed_fn.arg("context_node", "&mut PackingContextNode");
    bitsize_packed_fn.arg("bit_position", "u64");
    bitsize_packed_fn.line("let mut end_position = bit_position;");
    generate_choice_match_construct(
        symbol_scope,
        type_generator,
        bitsize_packed_fn,
        choice,
        true,
        &bitsize_field,
    );
    bitsize_packed_fn.line("Ok(end_position - bit_position)");
}
