use codegen::Scope;


use crate::internal::ast::{expression::ExpressionType, zenum::ZEnum};
use crate::internal::generator::{
    bitsize::bitsize_type_reference, decode::decode_type, encode::encode_type,
    file_generator::write_to_file, preamble::add_standard_imports,
    types::convert_to_enum_field_name, types::to_rust_module_name, types::to_rust_type_name,
    types::zserio_to_rust_type,
};
use std::path::Path;

pub fn generate_enum(scope: &mut Scope, zenum: &ZEnum, path: &Path, package_name: &str) -> String {
    let rust_module_name = to_rust_module_name(&zenum.name);
    let rust_type_name = to_rust_type_name(&zenum.name);
    add_standard_imports(scope);

    // generate the struct itself
    let gen_enum = scope.new_enum(&rust_type_name);
    gen_enum.vis("pub");
    gen_enum.derive("Copy");
    gen_enum.derive("Clone");
    gen_enum.derive("PartialEq");

    let mut enum_value = 0;
    for item in &zenum.items {
        if let Some(value_expression) = &item.expression {
            match value_expression.result_type {
                ExpressionType::Integer(v) => enum_value = v,
                _ => panic!("only integer value expressions are supported"),
            }
        }
        gen_enum.new_variant(format!(
            "{} = {}",
            convert_to_enum_field_name(&item.name),
            enum_value
        ));
        enum_value += 1;
    }

    let enum_impl = scope.new_impl(&rust_type_name);
    // Generate a function to create a new instance of the enum from an integer type
    let from_int_fn = enum_impl.new_fn("from_int");
    from_int_fn.arg("v", "i64");
    from_int_fn.ret("Self");
    from_int_fn.line("match v {");

    let mut enum_value = 0;
    for item in &zenum.items {
        if let Some(value_expression) = &item.expression {
            match value_expression.result_type {
                ExpressionType::Integer(v) => enum_value = v,
                _ => panic!("only integer value expressions are supported"),
            }
        }
        from_int_fn.line(format!(
            "{} => return {}::{},",
            enum_value,
            rust_type_name,
            convert_to_enum_field_name(&item.name)
        ));
        enum_value += 1;
    }

    from_int_fn.line("_ => panic!(\"unexpected value for enum\"),");

    from_int_fn.line("}");

    let z_impl = scope.new_impl(&rust_type_name);
    z_impl.impl_trait("ztype::ZserioPackableOject");

    // Generate a function to create a new instance of the enum
    let new_fn = z_impl.new_fn("new");
    new_fn.ret("Self");
    new_fn.line(format!(
        "{}::{}",
        &rust_type_name,
        convert_to_enum_field_name(&zenum.items[0].name)
    ));

    // generate the functions to serialize/deserialize
    generate_zserio_read(z_impl, zenum);
    generate_zserio_write(z_impl, zenum);
    generate_zserio_bitsize(z_impl, zenum);

    write_to_file(&scope.to_string(), path, package_name, &rust_module_name);
    rust_module_name
}

fn generate_zserio_read(struct_impl: &mut codegen::Impl, zenum: &ZEnum) {
    let rust_type_name = to_rust_type_name(&zenum.name);
    let temp_var = format!(
        "let v: {}",
        zserio_to_rust_type(&zenum.enum_type.name.as_str()).unwrap()
    );

    let zserio_read_fn = struct_impl.new_fn("zserio_read");
    zserio_read_fn.arg_mut_self();
    zserio_read_fn.arg("reader", "&mut BitReader");
    decode_type(
        zserio_read_fn,
        &temp_var,
        &String::from(""),
        &zenum.enum_type,
        None,
    );
    zserio_read_fn.line(format!("*self = {rust_type_name}::from_int(v as i64);",));

    let zserio_read_packed_fn = struct_impl.new_fn("zserio_read_packed");
    zserio_read_packed_fn.arg_mut_self();
    zserio_read_packed_fn.arg("context_node", "&mut PackingContextNode");
    zserio_read_packed_fn.arg("reader", "&mut BitReader");
    decode_type(
        zserio_read_packed_fn,
        &temp_var,
        &String::from(""),
        &zenum.enum_type,
        Option::from(0),
    );
    zserio_read_packed_fn.line(format!("*self = {rust_type_name}::from_int(v as i64);",));
}

fn generate_zserio_write(impl_codegen: &mut codegen::Impl, zenum: &ZEnum) {
    let rust_type_name = format!(
        "(*self as {})",
        zserio_to_rust_type(&zenum.enum_type.name.as_str()).unwrap()
    );

    let zserio_write_fn = impl_codegen.new_fn("zserio_write");
    zserio_write_fn.arg_ref_self();
    zserio_write_fn.arg("writer", "&mut BitWriter");
    encode_type(zserio_write_fn, &rust_type_name, &zenum.enum_type, None);

    let zserio_write_packed_fn = impl_codegen.new_fn("zserio_write_packed");
    zserio_write_packed_fn.arg_ref_self();
    zserio_write_packed_fn.arg("context_node", "&mut PackingContextNode");
    zserio_write_packed_fn.arg("writer", "&mut BitWriter");
    encode_type(
        zserio_write_packed_fn,
        &rust_type_name,
        &zenum.enum_type,
        Option::from(0),
    );
}

fn generate_zserio_bitsize(impl_codegen: &mut codegen::Impl, zenum: &ZEnum) {
    let rust_type_name = format!(
        "(*self as {})",
        zserio_to_rust_type(&zenum.enum_type.name.as_str()).unwrap()
    );

    let bitsize_fn = impl_codegen.new_fn("zserio_bitsize");
    bitsize_fn.ret("u64");
    bitsize_fn.arg_ref_self();
    bitsize_fn.arg("bit_position", "u64");
    bitsize_fn.line("let mut end_position = bit_position;");
    bitsize_type_reference(bitsize_fn, &rust_type_name, false, &zenum.enum_type, None);
    bitsize_fn.line("end_position - bit_position");

    let bitsize_packed_fn = impl_codegen.new_fn("zserio_bitsize_packed");
    bitsize_packed_fn.ret("u64");
    bitsize_packed_fn.arg_ref_self();
    bitsize_packed_fn.arg("context_node", "&mut PackingContextNode");
    bitsize_packed_fn.arg("bit_position", "u64");
    bitsize_packed_fn.line("let mut end_position = bit_position;");
    bitsize_type_reference(
        bitsize_packed_fn,
        &rust_type_name,
        false,
        &zenum.enum_type,
        Option::from(0),
    );
    bitsize_packed_fn.line("end_position - bit_position");
}
