use std::io::Write;

use codegen::Scope;

use crate::internal::ast::zstruct::ZStruct;
use crate::internal::generator::{
    bitsize::bitsize_field, decode::decode_field, encode::encode_field,
    file_generator::write_to_file, preamble::add_standard_imports, types::convert_name,
    types::zserio_to_rust_type,
};
use std::path::{Path, PathBuf};

pub fn generate_struct(
    mut scope: &mut Scope,
    zstruct: &ZStruct,
    path: &Path,
    package_name: &String,
) {
    add_standard_imports(&mut scope);
    // add the imports
    // generate the struct itself
    let gen_struct = scope.new_struct(&zstruct.name);
    gen_struct.vis("pub");

    for field in &zstruct.fields {
        let mut field_type = field.field_type.name.clone();
        if field.field_type.is_builtin {
            // the type is a zserio built-in type, such as int32, string, bool
            field_type = zserio_to_rust_type(&field_type).expect("type mapping failed");
        } else {
            // the type is a custom type, defined in some zserio file.
            field_type = field_type + "::" + field.field_type.name.as_str();
        }
        if field.array.is_some() {
            field_type = format!("Vec<{}>", field_type.as_str());
        }
        if field.is_optional {
            field_type = format!("Option<{}>", field_type.as_str());
        }
        let field_name = convert_name(&field.name);
        let gen_field = gen_struct.new_field(&field_name, &field_type);
        gen_field.vis("pub");
    }

    // generate the functions to serialize/deserialize
    let mut struct_impl = scope.new_impl(&zstruct.name);

    let marshal_fn = struct_impl.new_fn("marshal_zserio");
    marshal_fn.vis("pub");
    marshal_fn.arg_ref_self();
    marshal_fn.arg("writer", "&mut BitWriter");
    for field in &zstruct.fields {
        encode_field(marshal_fn, field);
    }

    let mut unmarshal_fn = struct_impl.new_fn("unmarshal_zserio");
    unmarshal_fn.vis("pub");
    unmarshal_fn.arg_mut_self();
    unmarshal_fn.arg("reader", "&mut BitReader");
    for field in &zstruct.fields {
        decode_field(unmarshal_fn, field);
    }

    let bitsize_fn = struct_impl.new_fn("zserio_bitsize");
    bitsize_fn.vis("pub");
    bitsize_fn.ret("u64");
    bitsize_fn.arg_ref_self();
    bitsize_fn.arg("bit_position", "u64");

    bitsize_fn.line("let mut end_position = bit_position;");
    for field in &zstruct.fields {
        bitsize_field(bitsize_fn, field);
    }
    bitsize_fn.line("end_position - bit_position");

    write_to_file(&scope.to_string(), path, package_name, &zstruct.name);
}
