use codegen::Function;

use crate::internal::ast::field::Field;
use crate::internal::ast::type_reference::TypeReference;
use crate::internal::generator::native_type::get_fundamental_type;
use crate::internal::generator::types::convert_field_name;

use crate::internal::generator::{array::array_type_name, array::initialize_array_trait};

pub fn bitsize_type_reference(
    function: &mut Function,
    field_name: &str,
    is_marshaler: bool,
    type_reference: &TypeReference,
    context_node_index: Option<u8>,
) {
    if is_marshaler {
        if let Some(node_idx) = context_node_index {
            // Use packed bitsize
            function.line(format!(
                "end_position += {}.zserio_bitsize_packed(&mut context_node.children[{}], end_position);",
                field_name,
                node_idx,
            ));
        } else {
            function.line(format!(
                "end_position += {}.zserio_bitsize(end_position);",
                field_name
            ));
        }
    } else if type_reference.is_builtin {
        if type_reference.name == "string" {
            // string types
            function.line(format!(
                "end_position += ztype::bitsize_string({}.as_str());",
                field_name
            ));
        } else if type_reference.name == "bool" {
            // boolean
            function.line("end_position += 1;");
        } else if let Some(node_idx) = context_node_index {
            // packed bitsize
            function.line(format!(
                "end_position += context_node.children[{}].context.bitsize_of(&{}, end_position, &{});",
                node_idx,
                initialize_array_trait(type_reference),
                field_name,
            ));
        } else if type_reference.bits != 0 {
            function.line(format!("end_position += {};", type_reference.bits));
        } else {
            match type_reference.name.as_str() {
                "uint8" => {
                    function.line("end_position += 8;");
                }
                "uint16" => {
                    function.line("end_position += 16;");
                }
                "uint32" => {
                    function.line("end_position += 32;");
                }
                "uint64" => {
                    function.line("end_position += 64;");
                }
                "int8" => {
                    function.line("end_position += 8;");
                }
                "int16" => {
                    function.line("end_position += 16;");
                }
                "int32" => {
                    function.line("end_position += 32;");
                }
                "int64" => {
                    function.line("end_position += 64;");
                }
                "float16" => {
                    function.line("end_position += 16;");
                }
                "float32" => {
                    function.line("end_position += 32;");
                }
                "float64" => {
                    function.line("end_position += 64;");
                }
                "varint" => {
                    function.line(format!(
                        "end_position += ztype::signed_bit_size({});",
                        field_name
                    ));
                }
                "varint16" => {
                    function.line(format!(
                        "end_position += ztype::signed_bit_size({});",
                        field_name
                    ));
                }
                "varint32" => {
                    function.line(format!(
                        "end_position += ztype::signed_bit_size({});",
                        field_name
                    ));
                }
                "varint64" => {
                    function.line(format!(
                        "end_position += ztype::signed_bit_size({});",
                        field_name
                    ));
                }
                "varuint" => {
                    function.line(format!(
                        "end_position += ztype::unsigned_bit_size({});",
                        field_name
                    ));
                }
                "varuint16" => {
                    function.line(format!(
                        "end_position += ztype::unsigned_bit_size({});",
                        field_name
                    ));
                }
                "varuint32" => {
                    function.line(format!(
                        "end_position += ztype::unsigned_bit_size({});",
                        field_name
                    ));
                }
                "varuint64" => {
                    function.line(format!(
                        "end_position += ztype::unsigned_bit_size({});",
                        field_name
                    ));
                }
                _ => panic!("failed"),
            }
        }
    }
}

pub fn bitsize_field(function: &mut Function, field: &Field, context_node_index: Option<u8>) {
    let native_type = get_fundamental_type(&field.field_type);
    let fund_type = native_type.fundamental_type;
    let mut field_name = format!("self.{}", convert_field_name(&field.name));

    if field.is_optional {
        function.line("end_position += 1;");
        function.line(format!("match {} {{", field_name));
        function.line("None => {}, ");
        function.line("Some(x) => {");
        field_name = "x".into();
    }

    if field.array.is_some() {
        function.line(format!(
            "end_position += {}.zserio_bitsize(&{}, end_position);",
            array_type_name(&field.name),
            field_name,
        ));
    } else {
        bitsize_type_reference(
            function,
            &field_name,
            native_type.is_marshaler,
            &fund_type,
            context_node_index,
        );
    }
    if field.is_optional {
        // in case the field is optional, end the if condition which checks
        // if the field is set.
        function.line("},");
        function.line("};");
    }
}
