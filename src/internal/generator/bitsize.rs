use crate::internal::ast::field::Field;
use crate::internal::ast::type_reference::TypeReference;
use crate::internal::compiler::fundamental_type::get_fundamental_type;
use crate::internal::compiler::symbol_scope::ModelScope;
use crate::internal::generator::encode::requires_borrowing;
use crate::internal::generator::expression::generate_boolean_expression;
use crate::internal::generator::types::{convert_field_name, TypeGenerator};
use codegen::Function;

use crate::internal::generator::{array::array_type_name, array::initialize_array_trait};

pub fn bitsize_type_reference(
    scope: &ModelScope,
    type_generator: &TypeGenerator,
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
        } else if type_reference.name == "extern" {
            function.line(format!("end_position += {}.bit_size as u64;", field_name));
        } else if type_reference.name == "bytes" {
            function.line(format!(
                "end_position += ({}.byte_size as u64) * 8;",
                field_name
            ));
        } else if type_reference.name == "bool" {
            // boolean
            function.line("end_position += 1;");
        } else if type_reference.bits != 0 {
            function.line(format!("end_position += {};", type_reference.bits));
        } else if let Some(node_idx) = context_node_index {
            // packed bitsize
            function.line(format!(
                "end_position += context_node.children[{}].context.as_mut().unwrap().bitsize_of(&{}, end_position, &{});",
                node_idx,
                initialize_array_trait(scope, type_generator, type_reference),
                field_name,
            ));
        } else {
            let bit_length = match type_reference.name.as_str() {
                "uint8" => String::from("8"),
                "uint16" => String::from("16"),
                "uint32" => String::from("32"),
                "uint64" => String::from("64"),
                "int8" => String::from("8"),
                "int16" => String::from("16"),
                "int32" => String::from("32"),
                "int64" => String::from("64"),
                "float16" => String::from("16"),
                "float32" => String::from("32"),
                "float64" => String::from("64"),
                "varint" => {
                    format!("ztype::varint_bitsize({}) as u64", field_name)
                }
                "varint16" => {
                    format!("ztype::varint16_bitsize({}) as u64", field_name)
                }
                "varint32" => {
                    format!("ztype::varint32_bitsize({}) as u64", field_name)
                }
                "varint64" => {
                    format!("ztype::varint64_bitsize({}) as u64", field_name)
                }
                "varuint" => {
                    format!("ztype::varuint_bitsize({}) as u64", field_name)
                }
                "varuint16" => {
                    format!("ztype::varuint16_bitsize({}) as u64", field_name)
                }
                "varuint32" => {
                    format!("ztype::varuint32_bitsize({}) as u64", field_name)
                }
                "varuint64" => {
                    format!("ztype::varuint64_bitsize({}) as u64", field_name)
                }
                "varsize" => {
                    format!("ztype::varsize_bitsize({}) as u64", field_name)
                }
                "int" | "bit" => {
                    format!("{} as u64", type_reference.bits)
                }
                _ => panic!("failed"),
            };
            function.line(format!("end_position += {};", bit_length,));
        }
    }
}

pub fn bitsize_field(
    scope: &ModelScope,
    type_generator: &TypeGenerator,
    function: &mut Function,
    field: &Field,
    context_node_index: Option<u8>,
) {
    let native_type = get_fundamental_type(&field.field_type, scope);
    let mut field_name = format!("self.{}", convert_field_name(&field.name));

    // Check if the field uses an optional clause
    if let Some(optional_clause) = &field.optional_clause {
        function.line(format!(
            "if {} {{",
            generate_boolean_expression(&optional_clause.borrow(), type_generator, scope)
        ));
    }

    // Align the bit count, if alignment is specified.
    if field.alignment != 0 {
        function.line(format!(
            "end_position += ztype::align_bitsize(end_position, {});",
            field.alignment
        ));
    }

    if field.is_optional {
        function.line("end_position += 1;");
        // If the type is a marshaller, take it by reference.
        let mut borrow_symbol = String::from("");
        if requires_borrowing(&native_type) {
            borrow_symbol = "&".into();
        }

        function.line(format!(
            "if let Some(x) = {}{} {{",
            borrow_symbol, field_name
        ));
        field_name = "x".into();
    }

    if field.array.is_some() {
        let array_type_name = array_type_name(&field.name);
        function.line(format!(
            "end_position += {}.zserio_bitsize(&{}, end_position);",
            array_type_name, field_name,
        ));
    } else {
        bitsize_type_reference(
            scope,
            type_generator,
            function,
            &field_name,
            native_type.is_marshaler,
            &native_type.fundamental_type,
            context_node_index,
        );
    }
    if field.is_optional {
        // in case the field is optional, end the if condition which checks
        // if the field is set.
        function.line("}");
    }

    // Close the optional clause.
    if field.optional_clause.is_some() {
        function.line("}");
    }
}
