use codegen::{Function, Scope};

use crate::internal::ast::zchoice::ZChoice;

use crate::internal::ast::field::Field;

use crate::internal::generator::{
    bitsize::bitsize_field, decode::decode_field, encode::encode_field,
    expression::generate_expression, file_generator::write_to_file, function::generate_function,
    new::new_field, new::new_param, preamble::add_standard_imports, types::convert_field_name,
    types::to_rust_module_name, types::to_rust_type_name, types::ztype_to_rust_type,
    zstruct::generate_struct_member_for_field,
};

use std::path::Path;

pub fn generate_choice(
    scope: &mut Scope,
    zchoice: &ZChoice,
    path: &Path,
    package_name: &str,
) -> String {
    let rust_module_name = to_rust_module_name(&zchoice.name);
    let rust_type_name = to_rust_type_name(&zchoice.name);

    add_standard_imports(scope);

    // generate the ZChoice as a rust structure
    let gen_choice = scope.new_struct(&rust_type_name);
    gen_choice.vis("pub");

    // if the field is parameterized, add the parameters as member variables
    for param in &zchoice.type_parameters {
        let param_type = ztype_to_rust_type(param.as_ref().borrow().zserio_type.as_ref());
        // Possible improvement: currently the parameters are copied instead of taken the reference.
        // It would be great to change that to references to avoid unnecessary copying, but this is
        // painful in rust due to the lifetime checks.
        // Because I am lazy, this implementation will just copy values over.
        let gen_param_field = gen_choice.new_field(
            convert_field_name(&param.as_ref().borrow().name),
            param_type,
        );
        gen_param_field.vis("pub");
    }

    // Add the data fields for each choice case to the generated structure.
    for case in &zchoice.cases {
        if let Some(case_field) = &case.field {
            generate_struct_member_for_field(gen_choice, &case_field.borrow());
        }
    }
    // Also add the field for the default case, if it is set.
    if let Some(default_case) = &zchoice.default_case {
        if let Some(case_field) = &default_case.field {
            generate_struct_member_for_field(gen_choice, &case_field.borrow());
        }
    }

    // generate the functions to serialize/deserialize
    let choice_impl = scope.new_impl(&rust_type_name);
    choice_impl.impl_trait("ztype::ZserioPackableOject");

    // Generate a function to create a new instance of the struct
    let new_fn = choice_impl.new_fn("new");
    new_fn.ret("Self");
    new_fn.line(format!("{} {{", &rust_type_name));

    for param in &zchoice.type_parameters {
        new_param(new_fn, &param.as_ref().borrow());
    }

    for case in &zchoice.cases {
        if let Some(case_field) = &case.field {
            new_field(new_fn, &case_field.borrow());
        }
    }
    new_fn.line("}");

    generate_zserio_read(choice_impl, zchoice);
    generate_zserio_write(choice_impl, zchoice);
    generate_zserio_bitsize(choice_impl, zchoice);

    // Generate all the zserio functions.
    let pub_impl = scope.new_impl(&rust_type_name);
    for zserio_function in &zchoice.functions {
        generate_function(pub_impl, &zserio_function.as_ref().borrow());
    }

    write_to_file(&scope.to_string(), path, package_name, &rust_module_name);
    rust_module_name
}

pub fn generate_choice_match_construct(
    code_gen_fn: &mut Function,
    zchoice: &ZChoice,
    packed: bool,
    f: &dyn Fn(&mut Function, &Field, Option<u8>),
) {
    let selector_name = convert_field_name(&zchoice.selector_expression.as_ref().borrow().text);
    let mut context_node_index = 0;

    code_gen_fn.line(format!("match self.{} {{", selector_name));
    for case in &zchoice.cases {
        let mut case_expressions = vec![];
        for case_expr in &case.conditions {
            case_expressions.push(generate_expression(&case_expr.as_ref().borrow()));
        }
        let selector_expression_evaluation = case_expressions.join(" | ");

        code_gen_fn.line(format!("{} => {{", selector_expression_evaluation));
        if let Some(field) = &case.field {
            let mut packing_node_index = None;
            if packed {
                packing_node_index = Option::from(context_node_index);
                context_node_index += 1
            }
            f(code_gen_fn, &field.borrow(), packing_node_index);
        }
        code_gen_fn.line("},");
    }
    if let Some(default_case) = &zchoice.default_case {
        code_gen_fn.line("_ => (");
        if let Some(field) = &default_case.field {
            let mut packing_node_index = None;
            if packed {
                packing_node_index = Option::from(context_node_index);
            }
            f(code_gen_fn, &field.borrow(), packing_node_index);
        }
        code_gen_fn.line("),");
    } else {
        code_gen_fn.line("_ => (),");
    }
    code_gen_fn.line("}");
}

fn generate_zserio_read(struct_impl: &mut codegen::Impl, choice: &ZChoice) {
    let zserio_read_fn = struct_impl.new_fn("zserio_read");
    zserio_read_fn.arg_mut_self();
    zserio_read_fn.arg("reader", "&mut BitReader");
    generate_choice_match_construct(zserio_read_fn, choice, false, &decode_field);

    let zserio_read_packed_fn = struct_impl.new_fn("zserio_read_packed");
    zserio_read_packed_fn.arg_mut_self();
    zserio_read_packed_fn.arg("context_node", "&mut PackingContextNode");
    zserio_read_packed_fn.arg("reader", "&mut BitReader");
    generate_choice_match_construct(zserio_read_packed_fn, choice, true, &decode_field);
}

fn generate_zserio_write(struct_impl: &mut codegen::Impl, choice: &ZChoice) {
    let zserio_write_fn = struct_impl.new_fn("zserio_write");
    zserio_write_fn.arg_ref_self();
    zserio_write_fn.arg("writer", "&mut BitWriter");
    generate_choice_match_construct(zserio_write_fn, choice, false, &encode_field);

    let zserio_write_packed_fn = struct_impl.new_fn("zserio_write_packed");
    zserio_write_packed_fn.arg_ref_self();
    zserio_write_packed_fn.arg("context_node", "&mut PackingContextNode");
    zserio_write_packed_fn.arg("writer", "&mut BitWriter");
    generate_choice_match_construct(zserio_write_packed_fn, choice, true, &encode_field);
}

fn generate_zserio_bitsize(struct_impl: &mut codegen::Impl, choice: &ZChoice) {
    let bitsize_fn = struct_impl.new_fn("zserio_bitsize");
    bitsize_fn.ret("u64");
    bitsize_fn.arg_ref_self();
    bitsize_fn.arg("bit_position", "u64");
    bitsize_fn.line("let mut end_position = bit_position;");
    generate_choice_match_construct(bitsize_fn, choice, false, &bitsize_field);
    bitsize_fn.line("end_position - bit_position");

    let bitsize_packed_fn = struct_impl.new_fn("zserio_bitsize_packed");
    bitsize_packed_fn.ret("u64");
    bitsize_packed_fn.arg_ref_self();
    bitsize_packed_fn.arg("context_node", "&mut PackingContextNode");
    bitsize_packed_fn.arg("bit_position", "u64");
    bitsize_packed_fn.line("let mut end_position = bit_position;");
    generate_choice_match_construct(bitsize_packed_fn, choice, true, &bitsize_field);
    bitsize_packed_fn.line("end_position - bit_position");
}
