use codegen::Function;

use crate::internal::ast::{field::Field, type_reference::TypeReference};
use crate::internal::generator::native_type::get_fundamental_type;
use crate::internal::generator::types::{convert_field_name, zserio_to_rust_type};

use crate::internal::generator::{array::array_type_name, array::initialize_array_trait};

pub fn encode_type(
    function: &mut Function,
    field_name: &String,
    field_type: &TypeReference,
    context_node_index: Option<u8>,
) {
    let native_type = get_fundamental_type(field_type);
    let fund_type = native_type.fundamental_type;
    if native_type.is_marshaler {
        if let Some(node_idx) = context_node_index {
            // Use packed reading
            function.line(format!(
                "{}.zserio_write_packed(&mut context_node.children[{}], writer);",
                field_name, node_idx,
            ));
        } else {
            function.line(format!("{}.zserio_write(writer);", field_name));
        }
    } else if fund_type.is_builtin {
        if fund_type.name == "string" {
            // string types
            function.line(format!(
                "ztype::write_string(writer, {}.as_str());",
                field_name
            ));
        } else if fund_type.name == "extern" {
            // TODO
        } else if fund_type.name == "bytes" {
            // TODO
        } else if fund_type.name == "bool" {
            // boolean
            function.line(format!(
                "let _ = writer.write_bool({}).unwrap();",
                field_name
            ));
        } else if let Some(node_idx) = context_node_index {
            // packed encoding
            function.line(format!(
                "context_node.children[{}].context.write(&{}, writer, &{});",
                node_idx,
                initialize_array_trait(&fund_type),
                field_name,
            ));
        } else if fund_type.bits != 0 {
            if fund_type.name == "int" {
                function.line(format!(
                    "ztype::write_signed_bits(writer, {}, {});",
                    field_name, fund_type.bits
                ));
            } else {
                function.line(format!(
                    "ztype::write_unsigned_bits(writer, {}, {});",
                    field_name, fund_type.bits
                ));
            }
        } else {
            // for "standard" fixed-width (unsigned) integer types, e.g. int32, uint64
            let rust_type_name =
                zserio_to_rust_type(&fund_type.name).expect("failed to determine native type");
            function.line(format!(
                "ztype::write_{}(writer, {});",
                rust_type_name, field_name
            ));
        }
    } else {
        panic!("unexpected type")
    }
}

pub fn encode_field(function: &mut Function, field: &Field, context_node_index: Option<u8>) {
    let mut field_name = format!("self.{}", convert_field_name(&field.name));

    if field.is_optional {
        function.line(format!("if let Some(x) = {} {{", field_name));
        function.line("writer.write_bool(true).expect(\"failed to write bool\");");
        field_name = "x".into();
    }

    if field.array.is_some() {
        // Array fields need to be deserialized using the array class, which takes
        // care of the array delta compression.

        // TODO support @index operator

        function.line(format!(
            "{}.zserio_write(writer, &{});",
            array_type_name(&field.name),
            field_name
        ));
    } else {
        encode_type(function, &field_name, &field.field_type, context_node_index);
    }

    if field.is_optional {
        function.line("} else {");
        // write a "0" if the field is not set.
        function.line("writer.write_bool(false).expect(\"failed to write bool\");");
        function.line("}");
    }
}
