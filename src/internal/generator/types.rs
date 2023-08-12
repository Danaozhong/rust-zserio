use crate::internal::ast::type_reference::TypeReference;
use convert_case::{Case, Casing};
use std::result::Result;

const RESERVED_RUST_KEYWORDS: &[&str] = &["type", "struct"];

pub fn to_rust_module_name(name: &String) -> String {
    name.to_case(Case::Snake)
}

pub fn to_rust_type_name(name: &String) -> String {
    name.to_case(Case::UpperCamel)
}

pub fn convert_field_name(name: &String) -> String {
    if RESERVED_RUST_KEYWORDS.contains(&name.as_str()) {
        return format!("z_{}", name.to_case(Case::Snake));
    }
    name.to_case(Case::Snake)
}

pub fn convert_to_enum_field_name(name: &String) -> String {
    name.to_case(Case::UpperCamel)
}

pub fn convert_to_function_name(name: &String) -> String {
    // Converts a function name from zserio style to rust style (snake case).
    name.to_case(Case::Snake)
}

pub fn ztype_to_rust_type(ztype: &TypeReference) -> String {
    if ztype.is_builtin {
        // the type is a zserio built-in type, such as int32, string, bool
        return zserio_to_rust_type(&ztype.name)
            .unwrap_or_else(|_| panic!("type mapping failed {:?}", &ztype.name));
    }
    // the type is a custom type, defined in some zserio file.
    custom_type_to_rust_type(&ztype.name)
}

pub fn custom_type_to_rust_type(name: &String) -> String {
    format!("{}::{}", to_rust_module_name(name), to_rust_type_name(name))
}

pub fn zserio_to_rust_type(name: &str) -> Result<String, &'static str> {
    match name {
        "int8" => Ok("i8".into()),
        "int16" => Ok("i16".into()),
        "int32" => Ok("i32".into()),
        "int64" => Ok("i64".into()),
        "varint16" => Ok("i16".into()),
        "varint32" => Ok("i32".into()),
        "varint64" => Ok("i64".into()),
        "varint" => Ok("i64".into()),
        "uint8" => Ok("u8".into()),
        "uint16" => Ok("u16".into()),
        "uint32" => Ok("u32".into()),
        "varuint16" => Ok("u16".into()),
        "varuint32" => Ok("u32".into()),
        "varuint64" => Ok("u64".into()),
        "varsize" => Ok("u32".into()),
        "varuint" => Ok("u64".into()),
        "string" => Ok("String".into()),
        "float16" => Ok("f32".into()),
        "float32" => Ok("f32".into()),
        "float64" => Ok("f64".into()),
        "bool" => Ok("bool".into()),
        "bit" => Ok("u64".into()),
        "int" => Ok("i64".into()),
        "extern" => Ok("ztype::ExternType".into()),
        "bytes" => Ok("ztype::BytesType".into()),
        _ => Err("not found"),
    }
}

pub fn zserio_type_bit_size(name: &str) -> Result<u8, &'static str> {
    match name {
        "int8" => Ok(8),
        "int16" => Ok(16),
        "int32" => Ok(32),
        "int64" => Ok(64),
        "varint32" => Ok(32),
        "uint8" => Ok(8),
        "uint16" => Ok(16),
        "uint32" => Ok(32),
        "varuint32" => Ok(32),
        "float16" => Ok(16),
        "float32" => Ok(32),
        "float64" => Ok(64),
        "bool" => Ok(1),
        _ => Err("not found"),
    }
}
