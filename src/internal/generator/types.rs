use std::result::Result;
use convert_case::{Case, Casing};

pub fn convert_name(name: &String) -> String {
    name.to_case(Case::Snake)
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
        "string" => return Ok("string".into()),
        "float16" => return Ok("f32".into()),
        "float32" => return Ok("f32".into()),
        "float64" => return Ok("f64".into()),
        "bool" => return Ok("bool".into()),
        _ => return Err("not found"),
    }
}
