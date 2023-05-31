use codegen::Scope;

use crate::internal::ast::zenum::ZEnum;
use crate::internal::generator::{
    bitsize::bitsize_type_reference, file_generator::write_to_file, preamble::add_standard_imports,
    types::convert_to_enum_field_name, types::to_rust_module_name, types::to_rust_type_name,
};
use std::path::Path;

pub fn generate_enum(scope: &mut Scope, zenum: &ZEnum, path: &Path, package_name: &str) -> String {
    let rust_module_name = to_rust_module_name(&zenum.name);
    let rust_type_name = to_rust_type_name(&zenum.name);
    add_standard_imports(scope);

    // generate the struct itself
    let gen_enum = scope.new_enum(&rust_type_name);
    gen_enum.vis("pub");

    for item in &zenum.items {
        gen_enum.new_variant(convert_to_enum_field_name(&item.name));
    }

    let z_impl = scope.new_impl(&rust_type_name);
    z_impl.impl_trait("ztype::ZserioPackableOject");

    // Generate a function to create a new instance of the struct
    let new_fn = z_impl.new_fn("new");
    new_fn.ret("Self");
    new_fn.line(format!(
        "{}::{}",
        &rust_type_name,
        convert_to_enum_field_name(&zenum.items[0].name)
    ));

    // generate the functions to serialize/deserialize
    let marshal_fn = z_impl.new_fn("zserio_write");
    marshal_fn.arg_ref_self();
    marshal_fn.arg("writer", "&mut BitWriter");

    let zserio_write_packed_fn = z_impl.new_fn("zserio_write_packed");
    zserio_write_packed_fn.arg_ref_self();
    zserio_write_packed_fn.arg("context_node", "&mut PackingContextNode");
    zserio_write_packed_fn.arg("writer", "&mut BitWriter");

    let unmarshal_fn = z_impl.new_fn("zserio_read");
    unmarshal_fn.arg_mut_self();
    unmarshal_fn.arg("reader", "&mut BitReader");

    let zserio_read_packed_fn = z_impl.new_fn("zserio_read_packed");
    zserio_read_packed_fn.arg_mut_self();
    zserio_read_packed_fn.arg("context_node", "&mut PackingContextNode");
    zserio_read_packed_fn.arg("reader", "&mut BitReader");

    let bitsize_fn = z_impl.new_fn("zserio_bitsize");
    bitsize_fn.ret("u64");
    bitsize_fn.arg_ref_self();
    bitsize_fn.arg("bit_position", "u64");

    bitsize_fn.line("let mut end_position = bit_position;");
    bitsize_type_reference(bitsize_fn, "this", false, &zenum.enum_type);
    bitsize_fn.line("end_position - bit_position");

    write_to_file(&scope.to_string(), path, package_name, &rust_module_name);
    rust_module_name
}
