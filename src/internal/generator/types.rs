use convert_case::{Case, Casing};
use std::result::Result;
use crate::internal::ast::field::Field;
use crate::internal::ast::type_reference::TypeReference;

pub fn convert_name(name: &String) -> String {
    name.to_case(Case::Snake)
}

pub fn array_type_name(name: &String) -> String {
    String::from("zs_array_") + name
}

pub fn field_to_rust_type(field: &Field) -> String {
    if field.field_type.is_builtin {
        // the type is a zserio built-in type, such as int32, string, bool
        return zserio_to_rust_type(&field.field_type.name).expect("type mapping failed");
    }
    // the type is a custom type, defined in some zserio file.
    field.field_type.name.clone() + "::" + field.field_type.name.as_str()
}

pub fn get_array_trait_for_type(zserio_type: &TypeReference) -> String {
    if !zserio_type.is_builtin {
        return format!("object_array_traits::<{}>", zserio_type.name).into();
    } else {
        match zserio_type.name.as_str() {
            "int8" => return "signed_bit_field_array_traits".into(),
            "int16" => return "signed_bit_field_array_traits".into(),
            "int32" => return "signed_bit_field_array_traits".into(),
            "int64" => return "signed_bit_field_array_traits".into(),
            "varint32" => return "var_uint_array_traits".into(),
            "uint8" => return "bit_field_array_traits".into(),
            "uint16" => return "bit_field_array_traits".into(),
            "uint32" => return "bit_field_array_traits".into(),
            "varuint32" => return "varuint32_array_traits".into(),
            "string" => return "string_array_traits".into(),
            "float16" => return "f16_array_traits".into(),
            "float32" => return "f32_array_traits".into(),
            "float64" => return "f64_array_traits".into(),
            "bool" => return "boolean_array_traits".into(),
            "bit" => return "bit_field_array_traits".into(),
            "int" => return "signed_bit_field_array_traits".into(),
            "extern" => return "object_array_traits::<ztype::extern_type>".into(),
            _ => panic!("failed to identify array trait")
        }
    }
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
