use crate::internal::ast::type_reference::TypeReference;
use convert_case::{Case, Casing};
use std::result::Result;

pub fn to_rust_module_name(name: &String) -> String {
    name.to_case(Case::Snake)
}

pub fn to_rust_type_name(name: &String) -> String {
    name.to_case(Case::UpperCamel)
}

pub fn convert_field_name(name: &String) -> String {
    name.to_case(Case::Snake)
}

pub fn convert_to_enum_field_name(name: &String) -> String {
    name.to_case(Case::UpperCamel)
}

pub fn ztype_to_rust_type(ztype: &TypeReference) -> String {
    if ztype.is_builtin {
        // the type is a zserio built-in type, such as int32, string, bool
        return zserio_to_rust_type(&ztype.name).expect("type mapping failed");
    }
    // the type is a custom type, defined in some zserio file.
    to_rust_module_name(&ztype.name) + "::" + to_rust_type_name(&ztype.name).as_str()
}

pub fn zserio_to_rust_type(name: &str) -> Result<String, &'static str> {
    match name {
        "int8" => Ok("i8".into()),
        "int16" => Ok("i16".into()),
        "int32" => Ok("i32".into()),
        "int64" => Ok("i64".into()),
        "varint32" => Ok("i32".into()),
        "uint8" => Ok("u8".into()),
        "uint16" => Ok("u16".into()),
        "uint32" => Ok("u32".into()),
        "varuint32" => Ok("u32".into()),
        "string" => Ok("String".into()),
        "float16" => Ok("f32".into()),
        "float32" => Ok("f32".into()),
        "float64" => Ok("f64".into()),
        "bool" => Ok("bool".into()),
        "bit" => Ok("u64".into()),
        "int" => Ok("i64".into()),
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
