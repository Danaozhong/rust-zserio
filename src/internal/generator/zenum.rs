use codegen::Scope;

use crate::internal::ast::{expression::ExpressionType, zenum::ZEnum};
use crate::internal::compiler::fundamental_type::{
    get_fundamental_type, FundamentalZserioTypeReference,
};
use crate::internal::compiler::symbol_scope::ModelScope;
use crate::internal::generator::{
    array::initialize_array_trait, bitsize::bitsize_type_reference, decode::decode_type,
    encode::encode_type, file_generator::write_to_file, preamble::add_standard_imports,
    types::convert_to_enum_field_name, types::zserio_to_rust_type, types::TypeGenerator,
};
use std::path::Path;

pub fn generate_enum(
    scope: &ModelScope,
    type_generator: &mut TypeGenerator,
    gen_scope: &mut Scope,
    zenum: &ZEnum,
    path: &Path,
    package_name: &str,
) -> String {
    let rust_module_name = type_generator.to_rust_module_name(&zenum.name);
    let rust_type_name = type_generator.to_rust_type_name(&zenum.name);
    let fundamental_type = get_fundamental_type(&zenum.enum_type, scope);
    let rust_type_type = type_generator.ztype_to_rust_type(&fundamental_type.fundamental_type);

    add_standard_imports(gen_scope);

    // generate the struct itself
    let gen_enum = gen_scope.new_enum(&rust_type_name);
    gen_enum.vis("pub");
    gen_enum.derive("Debug");
    gen_enum.derive("Default");
    gen_enum.derive("Clone");
    gen_enum.derive("Copy");
    gen_enum.derive("PartialEq");

    let mut enum_value = 0;
    for item in &zenum.items {
        if let Some(value_expression) = &item.value {
            match value_expression.as_ref().borrow().result_type {
                ExpressionType::Integer(v) => enum_value = v,
                _ => panic!("only integer value expressions are supported"),
            }
        }
        let v = gen_enum.new_variant(format!(
            "{} = {}",
            convert_to_enum_field_name(&item.name),
            enum_value
        ));
        if enum_value == 0 {
            v.annotation("#[default]");
        }
        enum_value += 1;
    }

    let enum_impl = gen_scope.new_impl(&rust_type_name);
    // Generate a function to create a new instance of the enum from an integer type
    let from_int_fn = enum_impl.new_fn("from_int");
    from_int_fn.arg("v", "i64");
    from_int_fn.ret("Result<Self>");
    from_int_fn.line("match v {");

    let mut enum_value = 0;
    for item in &zenum.items {
        if let Some(value_expression) = &item.value {
            match value_expression.as_ref().borrow().result_type {
                ExpressionType::Integer(v) => enum_value = v,
                _ => panic!("only integer value expressions are supported"),
            }
        }
        from_int_fn.line(format!(
            "{} => Ok({}::{}),",
            enum_value,
            rust_type_name,
            convert_to_enum_field_name(&item.name)
        ));
        enum_value += 1;
    }

    from_int_fn
        .line(format!("_ => Err(rust_zserio::ZserioError::DataError(format!(\"unexpected value for {}: {{}}\", v))),", rust_type_name));

    from_int_fn.line("}");

    let z_impl = gen_scope.new_impl(&rust_type_name);
    z_impl.impl_trait("ztype::ZserioPackableObject");

    // Generate a function to create a new instance of the enum
    let new_fn = z_impl.new_fn("new");
    new_fn.ret("Self");
    new_fn.line("Self::default()");

    // generate the functions to serialize/deserialize
    generate_zserio_read(scope, type_generator, z_impl, zenum, &fundamental_type);
    generate_zserio_write(scope, type_generator, z_impl, zenum, &fundamental_type);
    generate_zserio_bitsize(scope, type_generator, z_impl, zenum, &fundamental_type);

    // Generate the packed contexts.
    let create_packing_context_fn = z_impl.new_fn("zserio_create_packing_context");
    create_packing_context_fn.arg("context_node", "&mut PackingContextNode");
    create_packing_context_fn.line("context_node.children.push(PackingContextNode { children: vec![], context: Some(DeltaContext::new()), });");

    let init_packing_context_fn = z_impl.new_fn("zserio_init_packing_context");
    init_packing_context_fn.arg_ref_self();
    init_packing_context_fn.arg("context_node", "&mut PackingContextNode");
    init_packing_context_fn.line(format!(
        "context_node.children[0].context.as_mut().unwrap().init(&{}, &(*self as {}));",
        &initialize_array_trait(scope, type_generator, &fundamental_type.fundamental_type),
        &rust_type_type,
    ));

    write_to_file(
        type_generator,
        &gen_scope.to_string(),
        path,
        package_name,
        &rust_module_name,
    );
    rust_module_name
}

fn generate_zserio_read(
    scope: &ModelScope,
    type_generator: &mut TypeGenerator,
    struct_impl: &mut codegen::Impl,
    zenum: &ZEnum,
    fundamental_type: &FundamentalZserioTypeReference,
) {
    let rust_type_name = type_generator.to_rust_type_name(&zenum.name);
    let zserio_read_fn = struct_impl.new_fn("zserio_read");
    zserio_read_fn.arg_mut_self();
    zserio_read_fn.arg("reader", "&mut BitReader");
    zserio_read_fn.ret("Result<()>");
    decode_type(
        scope,
        type_generator,
        zserio_read_fn,
        &format!(
            "let v: {}",
            zserio_to_rust_type(&zenum.enum_type.name).unwrap()
        ),
        &String::from(""),
        fundamental_type,
        0,
        false,
    );
    zserio_read_fn.line(format!("*self = {rust_type_name}::from_int(v as i64)?;",));
    zserio_read_fn.line("Ok(())");

    let zserio_read_packed_fn = struct_impl.new_fn("zserio_read_packed");
    zserio_read_packed_fn.arg_mut_self();
    zserio_read_packed_fn.arg("context_node", "&mut PackingContextNode");
    zserio_read_packed_fn.arg("reader", "&mut BitReader");
    zserio_read_packed_fn.ret("Result<()>");
    zserio_read_packed_fn.line(format!(
        "let mut v: {} = 0;",
        zserio_to_rust_type(zenum.enum_type.name.as_str()).unwrap()
    ));
    decode_type(
        scope,
        type_generator,
        zserio_read_packed_fn,
        &String::from(""),
        &String::from("v"),
        fundamental_type,
        0,
        true,
    );
    zserio_read_packed_fn.line(format!("*self = {rust_type_name}::from_int(v as i64)?;",));
    zserio_read_packed_fn.line("Ok(())");
}

fn generate_zserio_write(
    scope: &ModelScope,
    type_generator: &mut TypeGenerator,
    impl_codegen: &mut codegen::Impl,
    zenum: &ZEnum,
    fundamental_type: &FundamentalZserioTypeReference,
) {
    let rust_type_name = format!(
        "(*self as {})",
        zserio_to_rust_type(zenum.enum_type.name.as_str()).unwrap()
    );

    let zserio_write_fn = impl_codegen.new_fn("zserio_write");
    zserio_write_fn.arg_ref_self();
    zserio_write_fn.arg("writer", "&mut BitWriter");
    zserio_write_fn.ret("Result<()>");
    encode_type(
        scope,
        type_generator,
        zserio_write_fn,
        &rust_type_name,
        fundamental_type,
        0,
        false,
    );
    zserio_write_fn.line("Ok(())");

    let zserio_write_packed_fn = impl_codegen.new_fn("zserio_write_packed");
    zserio_write_packed_fn.arg_ref_self();
    zserio_write_packed_fn.arg("context_node", "&mut PackingContextNode");
    zserio_write_packed_fn.arg("writer", "&mut BitWriter");
    zserio_write_packed_fn.ret("Result<()>");
    encode_type(
        scope,
        type_generator,
        zserio_write_packed_fn,
        &rust_type_name,
        fundamental_type,
        0,
        true,
    );
    zserio_write_packed_fn.line("Ok(())");
}

fn generate_zserio_bitsize(
    scope: &ModelScope,
    type_generator: &mut TypeGenerator,
    impl_codegen: &mut codegen::Impl,
    zenum: &ZEnum,
    fundamental_type: &FundamentalZserioTypeReference,
) {
    let rust_type_name = format!(
        "(*self as {})",
        zserio_to_rust_type(zenum.enum_type.name.as_str()).unwrap()
    );

    let bitsize_fn = impl_codegen.new_fn("zserio_bitsize");
    bitsize_fn.ret("Result<u64>");
    bitsize_fn.arg_ref_self();
    bitsize_fn.arg("bit_position", "u64");
    bitsize_fn.line("let mut end_position = bit_position;");
    bitsize_type_reference(
        scope,
        type_generator,
        bitsize_fn,
        &rust_type_name,
        fundamental_type,
        0,
        false,
    );
    bitsize_fn.line("Ok(end_position - bit_position)");

    let bitsize_packed_fn = impl_codegen.new_fn("zserio_bitsize_packed");
    bitsize_packed_fn.ret("Result<u64>");
    bitsize_packed_fn.arg_ref_self();
    bitsize_packed_fn.arg("context_node", "&mut PackingContextNode");
    bitsize_packed_fn.arg("bit_position", "u64");
    bitsize_packed_fn.line("let mut end_position = bit_position;");
    bitsize_type_reference(
        scope,
        type_generator,
        bitsize_packed_fn,
        &rust_type_name,
        fundamental_type,
        0,
        true,
    );
    bitsize_packed_fn.line("Ok(end_position - bit_position)");
}
