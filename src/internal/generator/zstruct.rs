

use codegen::Scope;

use crate::internal::ast::zstruct::ZStruct;
use crate::internal::generator::types::{array_type_name, zserio_to_rust_type};
use crate::internal::generator::{
    bitsize::bitsize_field, decode::decode_field, encode::encode_field, new::new_field,
    file_generator::write_to_file, preamble::add_standard_imports, types::convert_name, types::field_to_rust_type,
};

use std::path::{Path};

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
        let mut field_type = field_to_rust_type(field);

        // if the field is an array, add an additional member to store the array type details.
        // This must happen before wrapping the type in Vec<>, or Optional<>, as this is
        // used as a template argument.
        if field.array.is_some() {
            gen_struct.new_field(
                &array_type_name(&field.name),
                format!("ztype::Array<{}>", field_type),
            );
        }

        if field.array.is_some() {
            field_type = format!("Vec<{}>", field_type.as_str());
        }
        if field.is_optional {
            field_type = format!("Option<{}>", field_type.as_str());
        }
        let gen_field = gen_struct.new_field(&convert_name(&field.name), &field_type);
        gen_field.vis("pub");
    }

    // generate the functions to serialize/deserialize
    let struct_impl = scope.new_impl(&zstruct.name);

    // Create a function to ge
    let new_fn = struct_impl.new_fn("new");
    new_fn.vis("pub");
    new_fn.ret(&zstruct.name);
    new_fn.line(format!("return {} {{", &zstruct.name));

    for field in &zstruct.fields {
        new_field(new_fn, field);
    }
    new_fn.line("};");

    let marshal_fn = struct_impl.new_fn("marshal_zserio");
    marshal_fn.vis("pub");
    marshal_fn.arg_ref_self();
    marshal_fn.arg("writer", "&mut BitWriter");
    for field in &zstruct.fields {
        encode_field(marshal_fn, field);
    }

    let unmarshal_fn = struct_impl.new_fn("unmarshal_zserio");
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
