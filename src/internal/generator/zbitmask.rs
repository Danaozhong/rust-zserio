use codegen::Scope;

use crate::internal::ast::zbitmask::ZBitmaskType;
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

pub fn generate_bitmask(
    scope: &ModelScope,
    type_generator: &mut TypeGenerator,
    gen_scope: &mut Scope,
    zbitmask: &ZBitmaskType,
    path: &Path,
    package_name: &str,
) -> String {
    let rust_module_name = type_generator.to_rust_module_name(&zbitmask.name);
    let rust_type_name = type_generator.to_rust_type_name(&zbitmask.name);
    let fundamental_type = get_fundamental_type(&zbitmask.zserio_type, scope);
    let bitmask_rust_type = type_generator.ztype_to_rust_type(&fundamental_type.fundamental_type);

    add_standard_imports(gen_scope);
    gen_scope.import("bitmask_enum", "bitmask");

    let mut file_content = gen_scope.to_string();
    file_content += "\n\n";
    file_content += format!("#[bitmask({})]", bitmask_rust_type).as_str();

    let mut bitmask_scope = Scope::new();

    // generate the bitmask type itself.
    // Because we want to have the type implementing the type::ZserioPackableObject type trait,
    // we wrap it into a bitmask_enum.
    // https://github.com/Lukas3674/rust-bitmask-enum
    // The zserio-related traits are implemented using this bitmask enum type..
    let bitmask_enum = bitmask_scope.new_enum(&rust_type_name);
    bitmask_enum.vis("pub");

    for item in &zbitmask.values {
        bitmask_enum.new_variant(format!(
            "{} = {}",
            convert_to_enum_field_name(&item.name),
            item.value
        ));
    }

    let z_impl = bitmask_scope.new_impl(&rust_type_name);
    z_impl.impl_trait("ztype::ZserioPackableObject");

    // Generate a function to create a new instance of the enum
    let new_fn = z_impl.new_fn("new");
    new_fn.ret("Self");
    new_fn.line(format!(
        "{}::{}",
        &rust_type_name,
        convert_to_enum_field_name(&zbitmask.values[0].name)
    ));

    // generate the functions to serialize/deserialize
    generate_zserio_read(scope, type_generator, z_impl, zbitmask, &fundamental_type);
    generate_zserio_write(scope, type_generator, z_impl, &fundamental_type);
    generate_zserio_bitsize(scope, type_generator, z_impl, zbitmask, &fundamental_type);

    // Generate the packed contexts.
    let create_packing_context_fn = z_impl.new_fn("zserio_create_packing_context");
    create_packing_context_fn.arg("context_node", "&mut PackingContextNode");
    create_packing_context_fn.line("context_node.context = Some(DeltaContext::new());");
    create_packing_context_fn.line("context_node.children.push(PackingContextNode { children: vec![], context: Some(DeltaContext::new()), });");

    let init_packing_context_fn = z_impl.new_fn("zserio_init_packing_context");
    init_packing_context_fn.arg_ref_self();
    init_packing_context_fn.arg("context_node", "&mut PackingContextNode");
    init_packing_context_fn.line(format!(
        "context_node.children[0].context.as_mut().unwrap().init(&{}, &(self.bits as {}));",
        &initialize_array_trait(scope, type_generator, &fundamental_type.fundamental_type),
        &bitmask_rust_type,
    ));

    file_content += bitmask_scope.to_string().as_str();
    write_to_file(
        type_generator,
        &file_content,
        path,
        package_name,
        &rust_module_name,
    );
    rust_module_name
}

fn generate_zserio_read(
    scope: &ModelScope,
    type_generator: &mut TypeGenerator,
    impl_codegen: &mut codegen::Impl,
    zbitmask: &ZBitmaskType,
    fundamental_type: &FundamentalZserioTypeReference,
) {
    let zserio_read_fn = impl_codegen.new_fn("zserio_read");
    zserio_read_fn.arg_mut_self();
    zserio_read_fn.arg("reader", "&mut BitReader");
    zserio_read_fn.ret("Result<()>");
    decode_type(
        scope,
        type_generator,
        zserio_read_fn,
        &format!(
            "let v: {}",
            zserio_to_rust_type(zbitmask.zserio_type.name.as_str()).unwrap()
        ),
        &String::from(""),
        fundamental_type,
        0,
        false,
    );
    zserio_read_fn.line("self.bits = v;");
    zserio_read_fn.line("Ok(())");

    let zserio_read_packed_fn = impl_codegen.new_fn("zserio_read_packed");
    zserio_read_packed_fn.arg_mut_self();
    zserio_read_packed_fn.arg("context_node", "&mut PackingContextNode");
    zserio_read_packed_fn.arg("reader", "&mut BitReader");
    zserio_read_packed_fn.ret("Result<()>");
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
        fundamental_type,
        0,
        true,
    );
    zserio_read_packed_fn.line("self.bits = v;");
    zserio_read_packed_fn.line("Ok(())");
}

fn generate_zserio_write(
    scope: &ModelScope,
    type_generator: &mut TypeGenerator,
    impl_codegen: &mut codegen::Impl,
    fundamental_type: &FundamentalZserioTypeReference,
) {
    let rust_type_name = "self.bits".into();

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
    zbitmask: &ZBitmaskType,
    fundamental_type: &FundamentalZserioTypeReference,
) {
    let rust_type_name = format!(
        "(self.bits as {})",
        zserio_to_rust_type(zbitmask.zserio_type.name.as_str()).unwrap()
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
