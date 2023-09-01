#![allow(clippy::type_complexity)]
use crate::internal::ast::field::Field;
use crate::internal::ast::zunion::ZUnion;
use crate::internal::compiler::symbol_scope::ModelScope;
use crate::internal::generator::array::instantiate_zserio_array;
use crate::internal::generator::{
    bitsize::bitsize_field, decode::decode_field, encode::encode_field,
    file_generator::write_to_file, function::generate_function, new::new_field, new::new_param,
    preamble::add_standard_imports, types::convert_field_name,
    types::convert_to_union_selector_name, types::to_rust_module_name, types::to_rust_type_name,
    types::ztype_to_rust_type,
};
use codegen::Scope;
use codegen::Struct;

use std::path::Path;

pub fn generate_struct_member_for_field(gen_struct: &mut Struct, field: &Field) {
    let mut field_type = ztype_to_rust_type(field.field_type.as_ref());

    if field.array.is_some() {
        field_type = format!("Vec<{}>", field_type.as_str());
    }
    if field.is_optional {
        field_type = format!("Option<{}>", field_type.as_str());
    }
    let gen_field = gen_struct.new_field(&convert_field_name(&field.name), &field_type);
    gen_field.vis("pub");
}

pub fn generate_union(
    symbol_scope: &ModelScope,
    codegen_scope: &mut Scope,
    zunion: &ZUnion,
    path: &Path,
    package_name: &str,
) -> String {
    let rust_module_name = to_rust_module_name(&zunion.name);
    let rust_type_name = to_rust_type_name(&zunion.name);

    // add the imports
    add_standard_imports(codegen_scope);

    // Generate the union selector enum. This enumeration represents
    // the current selection of the union.
    let selector_type_name = format!("{}Selector", &rust_type_name);
    let union_selector_gen_scope = codegen_scope.new_enum(&selector_type_name);
    union_selector_gen_scope.derive("Copy");
    union_selector_gen_scope.derive("Clone");
    union_selector_gen_scope.derive("PartialEq");

    union_selector_gen_scope.vis("pub");
    for (field_index, field) in zunion.fields.iter().enumerate() {
        let selector_union_name = convert_to_union_selector_name(&field.borrow().name);
        union_selector_gen_scope.new_variant(format!("{} = {}", selector_union_name, field_index));
    }

    // We also need to generate a function to generate the selector
    // type from an integer (to be able to set the value during
    // deserialization).
    let union_selector_impl_gen_scope = codegen_scope.new_impl(&selector_type_name);
    let union_selector_from_fn_scope = union_selector_impl_gen_scope.new_fn("from_u32");
    union_selector_from_fn_scope.arg("value", "u32");
    union_selector_from_fn_scope.ret(&selector_type_name);
    union_selector_from_fn_scope.vis("pub");
    union_selector_from_fn_scope.line("match value {");
    for (field_index, field) in zunion.fields.iter().enumerate() {
        let selector_union_name = convert_to_union_selector_name(&field.borrow().name);
        union_selector_from_fn_scope.line(format!(
            "{} => {}::{},",
            field_index, selector_type_name, selector_union_name
        ));
    }

    union_selector_from_fn_scope.line("_ => panic!(\"unsupprted value\"),");
    union_selector_from_fn_scope.line("}");

    // generate the union itself
    let gen_union = codegen_scope.new_struct(&rust_type_name);
    gen_union.vis("pub");
    gen_union.derive("Clone");
    gen_union.derive("PartialEq");

    // if the union is parameterized, add the parameters as struct fields
    for param in &zunion.type_parameters {
        let param_type = ztype_to_rust_type(param.borrow().zserio_type.as_ref());
        // Possible improvement: currently the parameters are copied instead of taken the reference.
        // It would be great to change that to references to avoid unnecessary copying, but this is
        // painful in rust due to the lifetime checks.
        // Because I am lazy, this implementation will just copy values over.
        let gen_param_field = gen_union.new_field(
            convert_field_name(&param.as_ref().borrow().name),
            param_type,
        );
        gen_param_field.vis("pub");
    }

    // One variable selects the current value of the union type.
    let union_selector_field = gen_union.new_field("union_selector", &selector_type_name);
    union_selector_field.vis("pub");

    // Add the data fields to the union
    for field in &zunion.fields {
        generate_struct_member_for_field(gen_union, &field.borrow());
    }

    // generate the functions to serialize/deserialize
    let union_impl = codegen_scope.new_impl(&rust_type_name);
    union_impl.impl_trait("ztype::ZserioPackableOject");

    // Generate a function to create a new instance of the struct
    let new_fn = union_impl.new_fn("new");
    new_fn.ret("Self");
    new_fn.line(format!("{} {{", &rust_type_name));

    // Set the default value for the selector
    new_fn.line(format!(
        "union_selector: {}::{},",
        &selector_type_name,
        &convert_to_union_selector_name(&zunion.fields[0].borrow().name)
    ));

    // Set the default value for the parameters
    for param in &zunion.type_parameters {
        new_param(symbol_scope, new_fn, &param.as_ref().borrow());
    }

    // Set the default values for the fields
    for field in &zunion.fields {
        new_field(symbol_scope, new_fn, &field.borrow());
    }
    new_fn.line("}");

    // Generate the functions to read, write and bitcount the data to/from zserio format.
    generate_zserio_read(symbol_scope, union_impl, zunion);
    generate_zserio_write(symbol_scope, union_impl, zunion);
    generate_zserio_bitsize(symbol_scope, union_impl, zunion);

    // Generate all the zserio functions.
    let pub_impl = codegen_scope.new_impl(&rust_type_name);
    for zserio_function in &zunion.functions {
        generate_function(pub_impl, &zserio_function.as_ref().borrow());
    }

    write_to_file(
        &codegen_scope.to_string(),
        path,
        package_name,
        &rust_module_name,
    );
    rust_module_name
}

pub fn generate_union_match_construct(
    symbol_scope: &ModelScope,
    code_gen_fn: &mut codegen::Function,
    zunion: &ZUnion,
    packed: bool,
    f: &dyn Fn(&ModelScope, &mut codegen::Function, &Field, Option<u8>),
) {
    let mut context_node_index = 0;
    let rust_type_name = to_rust_type_name(&zunion.name);
    let selector_type_name = format!("{}Selector", &rust_type_name);

    code_gen_fn.line("match &self.union_selector {");
    for field in &zunion.fields {
        code_gen_fn.line(format!(
            "{}::{} => {{",
            selector_type_name,
            convert_to_union_selector_name(&field.borrow().name)
        ));
        instantiate_zserio_array(symbol_scope, code_gen_fn, &field.borrow(), false);
        let mut packing_node_index = None;
        if packed {
            packing_node_index = Option::from(context_node_index);
            context_node_index += 1
        }
        f(
            symbol_scope,
            code_gen_fn,
            &field.borrow(),
            packing_node_index,
        );
        code_gen_fn.line("},");
    }
    code_gen_fn.line("}");
}

fn generate_zserio_read(
    symbol_scope: &ModelScope,
    struct_impl: &mut codegen::Impl,
    union: &ZUnion,
) {
    let rust_type_name = to_rust_type_name(&union.name);
    let zserio_read_fn = struct_impl.new_fn("zserio_read");
    zserio_read_fn.arg_mut_self();
    zserio_read_fn.arg("reader", "&mut BitReader");

    zserio_read_fn.line(format!(
        "self.union_selector = {}Selector::from_u32(ztype::read_varsize(reader));",
        rust_type_name
    ));

    generate_union_match_construct(symbol_scope, zserio_read_fn, union, false, &decode_field);

    let zserio_read_packed_fn = struct_impl.new_fn("zserio_read_packed");
    zserio_read_packed_fn.arg_mut_self();
    zserio_read_packed_fn.arg("context_node", "&mut PackingContextNode");
    zserio_read_packed_fn.arg("reader", "&mut BitReader");
    generate_union_match_construct(
        symbol_scope,
        zserio_read_packed_fn,
        union,
        true,
        &decode_field,
    );
}

fn generate_zserio_write(
    symbol_scope: &ModelScope,
    struct_impl: &mut codegen::Impl,
    union: &ZUnion,
) {
    let zserio_write_fn = struct_impl.new_fn("zserio_write");
    zserio_write_fn.arg_ref_self();
    zserio_write_fn.arg("writer", "&mut BitWriter");
    zserio_write_fn.line("ztype::write_varsize(writer, self.union_selector as u32);".to_string());
    generate_union_match_construct(symbol_scope, zserio_write_fn, union, false, &encode_field);

    let zserio_write_packed_fn = struct_impl.new_fn("zserio_write_packed");
    zserio_write_packed_fn.arg_ref_self();
    zserio_write_packed_fn.arg("context_node", "&mut PackingContextNode");
    zserio_write_packed_fn.arg("writer", "&mut BitWriter");
    zserio_write_packed_fn
        .line("ztype::write_varsize(writer, self.union_selector as u32);".to_string());
    generate_union_match_construct(
        symbol_scope,
        zserio_write_packed_fn,
        union,
        true,
        &encode_field,
    );
}

fn generate_zserio_bitsize(
    symbol_scope: &ModelScope,
    struct_impl: &mut codegen::Impl,
    union: &ZUnion,
) {
    let bitsize_fn = struct_impl.new_fn("zserio_bitsize");
    bitsize_fn.ret("u64");
    bitsize_fn.arg_ref_self();
    bitsize_fn.arg("bit_position", "u64");
    bitsize_fn.line("let mut end_position = bit_position;");
    bitsize_fn.line("end_position += ztype::varsize_bitsize(self.union_selector as u32) as u64;");
    generate_union_match_construct(symbol_scope, bitsize_fn, union, false, &bitsize_field);
    bitsize_fn.line("end_position - bit_position");

    let bitsize_packed_fn = struct_impl.new_fn("zserio_bitsize_packed");
    bitsize_packed_fn.ret("u64");
    bitsize_packed_fn.arg_ref_self();
    bitsize_packed_fn.arg("context_node", "&mut PackingContextNode");
    bitsize_packed_fn.arg("bit_position", "u64");
    bitsize_packed_fn.line("let mut end_position = bit_position;");
    generate_union_match_construct(symbol_scope, bitsize_packed_fn, union, true, &bitsize_field);
    bitsize_packed_fn.line("end_position - bit_position");
}
