use convert_case::{Case, Casing};
use std::result::Result;

pub fn convert_name(name: &String) -> String {
    name.to_case(Case::Snake)
}

pub fn array_type_name(name: &String) -> String {
    String::from("zs_array_") + name
}

pub fn zserio_to_rust_type(name: &String) -> Result<String, &'static str> {
    match name.as_str() {
        "int8" => return Ok("i8".into()),
        "int16" => return Ok("i16".into()),
        "int32" => return Ok("i32".into()),
        "int64" => return Ok("i64".into()),
        "varint32" => return Ok("i32".into()),
        "uint8" => return Ok("u8".into()),
        "uint16" => return Ok("u16".into()),
        "uint32" => return Ok("u32".into()),
        "varuint32" => return Ok("u32".into()),
        "string" => return Ok("String".into()),
        "float16" => return Ok("f32".into()),
        "float32" => return Ok("f32".into()),
        "float64" => return Ok("f64".into()),
        "bool" => return Ok("bool".into()),
        "bit" => return Ok("u64".into()),
        "int" => return Ok("i64".into()),
        _ => return Err("not found"),
    }
}

pub fn zserio_type_bit_size(name: &String) -> Result<u8, &'static str> {
    match name.as_str() {
        "int8" => return Ok(8),
        "int16" => return Ok(16),
        "int32" => return Ok(32),
        "int64" => return Ok(64),
        "varint32" => return Ok(32),
        "uint8" => return Ok(8),
        "uint16" => return Ok(16),
        "uint32" => return Ok(32),
        "varuint32" => return Ok(32),
        "float16" => return Ok(16),
        "float32" => return Ok(32),
        "float64" => return Ok(64),
        "bool" => return Ok(1),
        _ => return Err("not found"),
    }
}
