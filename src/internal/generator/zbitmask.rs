use codegen::Scope;

use crate::internal::ast::{expression::ExpressionType, zbitmask::ZBitmaskType};
use crate::internal::compiler::fundamental_type::get_fundamental_type;
use crate::internal::compiler::symbol_scope::ModelScope;
use crate::internal::generator::{
    array::initialize_array_trait, bitsize::bitsize_type_reference, decode::decode_type,
    encode::encode_type, file_generator::write_to_file, preamble::add_standard_imports,
    types::to_rust_constant_name, types::to_rust_module_name, types::to_rust_type_name,
    types::zserio_to_rust_type, types::TypeGenerator,
};
use std::path::Path;

pub fn generate_bitmask(
    scope: &ModelScope,
    type_generator: &TypeGenerator,
    gen_scope: &mut Scope,
    zbitmask: &ZBitmaskType,
    path: &Path,
    package_name: &str,
) -> String {
    let rust_module_name = to_rust_module_name(&zbitmask.name);
    let rust_type_name = to_rust_type_name(&zbitmask.name);
    let fundamental_type = get_fundamental_type(&zbitmask.zserio_type, scope);
    let bitmask_rust_type = type_generator.ztype_to_rust_type(&fundamental_type.fundamental_type);

    add_standard_imports(gen_scope);

    // generate the bitmask type itself.
    // Because we want to have the type implementing the type::ZserioPackableObject type trait,
    // we wrap it into a small struct. The trait is implemented using this wrapper struct.
    let bitmask_wrapper_struct = gen_scope.new_struct(&rust_type_name);
    bitmask_wrapper_struct.vis("pub");
    bitmask_wrapper_struct.derive("Clone");
    bitmask_wrapper_struct.derive("Copy");
    bitmask_wrapper_struct.derive("PartialEq");

    let bitmask_value_field = bitmask_wrapper_struct.new_field("bitmask_value", &bitmask_rust_type);
    bitmask_value_field.vis("pub");

    // the code generator doesn't support consts (https://github.com/carllerche/codegen/issues/17),
    // therefore, we need to manually create them.
    let mut file_content = gen_scope.to_string() + "\n\n";

    let mut bitmask_value = 1;
    for item in &zbitmask.values {
        if let Some(value_expression) = &item.value {
            match value_expression.as_ref().borrow().result_type {
                ExpressionType::Integer(v) => bitmask_value = v,
                _ => panic!("only integer value expressions are supported"),
            }
        }

        file_content += format!(
            "pub const {}: {} = {};\n",
            &to_rust_constant_name(&item.name),
            &bitmask_rust_type,
            bitmask_value,
        )
        .as_str();
        bitmask_value *= 2;
    }

    let mut new_scope = Scope::new();

    let z_impl = new_scope.new_impl(&rust_type_name);
    z_impl.impl_trait("ztype::ZserioPackableObject");

    // Generate a function to create a new instance of the enum
    let new_fn = z_impl.new_fn("new");
    new_fn.ret("Self");
    new_fn.line(format!(
        "{} {{ bitmask_value: {} }}",
        &rust_type_name,
        to_rust_constant_name(&zbitmask.values[0].name)
    ));

    // generate the functions to serialize/deserialize
    generate_zserio_read(scope, type_generator, z_impl, zbitmask);
    generate_zserio_write(scope, type_generator, z_impl, zbitmask);
    generate_zserio_bitsize(scope, type_generator, z_impl, zbitmask);

    // Generate the packed contexts.
    let create_packing_context_fn = z_impl.new_fn("zserio_create_packing_context");
    create_packing_context_fn.arg("context_node", "&mut PackingContextNode");
    create_packing_context_fn.line("context_node.context = Some(DeltaContext::new());");
    create_packing_context_fn.line("context_node.children.push(PackingContextNode { children: vec![], context: Some(DeltaContext::new()), });");

    let init_packing_context_fn = z_impl.new_fn("zserio_init_packing_context");
    init_packing_context_fn.arg_ref_self();
    init_packing_context_fn.arg("context_node", "&mut PackingContextNode");
    init_packing_context_fn.line(format!("context_node.children[0].context.as_mut().unwrap().init(&{}, &(self.bitmask_value as {}));", 
        &initialize_array_trait(scope, type_generator, &fundamental_type.fundamental_type),
        &bitmask_rust_type,
    ));

    file_content += new_scope.to_string().as_str();

    write_to_file(&file_content, path, package_name, &rust_module_name);
    rust_module_name
}

fn generate_zserio_read(
    scope: &ModelScope,
    type_generator: &TypeGenerator,
    impl_codegen: &mut codegen::Impl,
    zbitmask: &ZBitmaskType,
) {
    let zserio_read_fn = impl_codegen.new_fn("zserio_read");
    zserio_read_fn.arg_mut_self();
    zserio_read_fn.arg("reader", "&mut BitReader");
    decode_type(
        scope,
        type_generator,
        zserio_read_fn,
        &format!(
            "let v: {}",
            zserio_to_rust_type(zbitmask.zserio_type.name.as_str()).unwrap()
        ),
        &String::from(""),
        &zbitmask.zserio_type,
        None,
    );
    zserio_read_fn.line("self.bitmask_value = v;");

    let zserio_read_packed_fn = impl_codegen.new_fn("zserio_read_packed");
    zserio_read_packed_fn.arg_mut_self();
    zserio_read_packed_fn.arg("context_node", "&mut PackingContextNode");
    zserio_read_packed_fn.arg("reader", "&mut BitReader");
    zserio_read_packed_fn.line(format!(
        "let mut v: {} = 0;",
        zserio_to_rust_type(zbitmask.zserio_type.name.as_str()).unwrap()
    ));
    decode_type(
        scope,
        type_generator,
        zserio_read_packed_fn,
        &String::from(""),
        &String::from("v"),
        &zbitmask.zserio_type,
        Option::from(0),
    );
    zserio_read_packed_fn.line("self.bitmask_value = v");
}

fn generate_zserio_write(
    scope: &ModelScope,
    type_generator: &TypeGenerator,
    impl_codegen: &mut codegen::Impl,
    zbitmask: &ZBitmaskType,
) {
    let rust_type_name = "self.bitmask_value".into();

    let zserio_write_fn = impl_codegen.new_fn("zserio_write");
    zserio_write_fn.arg_ref_self();
    zserio_write_fn.arg("writer", "&mut BitWriter");
    encode_type(
        scope,
        type_generator,
        zserio_write_fn,
        &rust_type_name,
        &zbitmask.zserio_type,
        None,
    );

    let zserio_write_packed_fn = impl_codegen.new_fn("zserio_write_packed");
    zserio_write_packed_fn.arg_ref_self();
    zserio_write_packed_fn.arg("context_node", "&mut PackingContextNode");
    zserio_write_packed_fn.arg("writer", "&mut BitWriter");
    encode_type(
        scope,
        type_generator,
        zserio_write_packed_fn,
        &rust_type_name,
        &zbitmask.zserio_type,
        Option::from(0),
    );
}

fn generate_zserio_bitsize(
    scope: &ModelScope,
    type_generator: &TypeGenerator,
    impl_codegen: &mut codegen::Impl,
    zbitmask: &ZBitmaskType,
) {
    let rust_type_name = format!(
        "(self.bitmask_value as {})",
        zserio_to_rust_type(zbitmask.zserio_type.name.as_str()).unwrap()
    );

    let bitsize_fn = impl_codegen.new_fn("zserio_bitsize");
    bitsize_fn.ret("u64");
    bitsize_fn.arg_ref_self();
    bitsize_fn.arg("bit_position", "u64");
    bitsize_fn.line("let mut end_position = bit_position;");
    bitsize_type_reference(
        scope,
        type_generator,
        bitsize_fn,
        &rust_type_name,
        false,
        &zbitmask.zserio_type,
        None,
    );
    bitsize_fn.line("end_position - bit_position");

    let bitsize_packed_fn = impl_codegen.new_fn("zserio_bitsize_packed");
    bitsize_packed_fn.ret("u64");
    bitsize_packed_fn.arg_ref_self();
    bitsize_packed_fn.arg("context_node", "&mut PackingContextNode");
    bitsize_packed_fn.arg("bit_position", "u64");
    bitsize_packed_fn.line("let mut end_position = bit_position;");
    bitsize_type_reference(
        scope,
        type_generator,
        bitsize_packed_fn,
        &rust_type_name,
        false,
        &zbitmask.zserio_type,
        Option::from(0),
    );
    bitsize_packed_fn.line("end_position - bit_position");
}
