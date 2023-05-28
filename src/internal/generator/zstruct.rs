use codegen::Scope;

use crate::internal::ast::zstruct::ZStruct;
use crate::internal::generator::array::instantiate_zserio_arrays;

use crate::internal::generator::{
    bitsize::bitsize_field, decode::decode_field, encode::encode_field,
    file_generator::write_to_file, new::new_field, preamble::add_standard_imports,
    types::convert_field_name, types::to_rust_module_name, types::to_rust_type_name,
    types::ztype_to_rust_type,
};

use std::path::Path;

pub fn generate_struct(
    scope: &mut Scope,
    zstruct: &ZStruct,
    path: &Path,
    package_name: &str,
) -> String {
    let rust_module_name = to_rust_module_name(&zstruct.name);
    let rust_type_name = to_rust_type_name(&zstruct.name);

    add_standard_imports(scope);
    // add the imports
    // generate the struct itself
    let gen_struct = scope.new_struct(&rust_type_name);
    gen_struct.vis("pub");

    for field in &zstruct.fields {
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

    // generate the functions to serialize/deserialize
    let struct_impl = scope.new_impl(&rust_type_name);
    struct_impl.impl_trait("ztype::ZserioPackableOject");

    // Generate a function to create a new instance of the struct
    let new_fn = struct_impl.new_fn("new");
    new_fn.ret("Self");
    new_fn.line(format!("{} {{", &rust_type_name));

    for field in &zstruct.fields {
        new_field(new_fn, field);
    }
    new_fn.line("}");

    let marshal_fn = struct_impl.new_fn("zserio_write");
    marshal_fn.arg_ref_self();
    marshal_fn.arg("writer", "&mut BitWriter");

    // create the array traits
    instantiate_zserio_arrays(marshal_fn, &zstruct.fields);
    for field in &zstruct.fields {
        encode_field(marshal_fn, field);
    }

    let zserio_write_packed_fn = struct_impl.new_fn("zserio_write_packed");
    zserio_write_packed_fn.arg_ref_self();
    zserio_write_packed_fn.arg("context_node", "&mut PackingContextNode");
    zserio_write_packed_fn.arg("writer", "&mut BitWriter");

    let unmarshal_fn = struct_impl.new_fn("zserio_read");
    unmarshal_fn.arg_mut_self();
    unmarshal_fn.arg("reader", "&mut BitReader");

    instantiate_zserio_arrays(unmarshal_fn, &zstruct.fields);

    for field in &zstruct.fields {
        decode_field(unmarshal_fn, field);
    }

    let zserio_read_packed_fn = struct_impl.new_fn("zserio_read_packed");
    zserio_read_packed_fn.arg_mut_self();
    zserio_read_packed_fn.arg("context_node", "&mut PackingContextNode");
    zserio_read_packed_fn.arg("reader", "&mut BitReader");

    let bitsize_fn = struct_impl.new_fn("zserio_bitsize");
    bitsize_fn.ret("u64");
    bitsize_fn.arg_ref_self();
    bitsize_fn.arg("bit_position", "u64");

    instantiate_zserio_arrays(bitsize_fn, &zstruct.fields);

    bitsize_fn.line("let mut end_position = bit_position;");
    for field in &zstruct.fields {
        bitsize_field(bitsize_fn, field);
    }
    bitsize_fn.line("end_position - bit_position");

    write_to_file(&scope.to_string(), path, package_name, &rust_module_name);
    rust_module_name
}
