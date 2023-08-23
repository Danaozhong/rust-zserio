use codegen::Function;

use crate::internal::ast::{field::Field, type_reference::TypeReference};
use crate::internal::compiler::fundamental_type::get_fundamental_type;
use crate::internal::compiler::symbol_scope::ModelScope;
use crate::internal::generator::types::{
    convert_field_name, zserio_to_rust_type, ztype_to_rust_type,
};

use crate::internal::generator::array::{array_type_name, initialize_array_trait};

pub fn decode_type(
    scope: &ModelScope,
    function: &mut Function,
    lvalue_field_name: &String,
    rvalue_field_name: &String,
    field_type: &TypeReference,
    context_node_index: Option<u8>,
) {
    let native_type = get_fundamental_type(field_type, scope);
    let fund_type = native_type.fundamental_type;

    if native_type.is_marshaler {
        // the field is a marshable type (struct, choice, enum)
        if let Some(node_idx) = context_node_index {
            // Use packed reading
            function.line(format!(
                "{}.zserio_read_packed(&mut context_node.children[{}], reader);",
                rvalue_field_name, node_idx,
            ));
        } else {
            // use standard reading
            function.line(format!("{}.zserio_read(reader);", rvalue_field_name));
        }
    } else if fund_type.is_builtin {
        // The type should be a native type
        if fund_type.name == "string" {
            // string types
            function.line(format!(
                "{} = ztype::read_string(reader);",
                lvalue_field_name
            ));
        } else if fund_type.name == "extern" {
            function.line(format!(
                "{} = ztype::read_extern_type(reader);",
                lvalue_field_name
            ));
        } else if fund_type.name == "bytes" {
            function.line(format!(
                "{} = ztype::read_bytes_type(reader);",
                lvalue_field_name
            ));
        } else if fund_type.name == "bool" {
            // boolean
            function.line(format!(
                "{} = reader.read_bool().expect(\"failed to read bool\");",
                lvalue_field_name
            ));
        } else if let Some(node_idx) = context_node_index {
            // packed decoding
            function.line(format!(
                "{} = context_node.children[{}].context.read(&{}, reader);",
                lvalue_field_name,
                node_idx,
                initialize_array_trait(&fund_type),
            ));
        } else {
            // nonpacked decoding
            if fund_type.bits != 0 {
                if fund_type.name == "int" {
                    function.line(format!(
                        "{} = ztype::read_signed_bits(reader, {});",
                        lvalue_field_name, fund_type.bits
                    ));
                } else {
                    function.line(format!(
                        "{} = ztype::read_unsigned_bits(reader, {});",
                        lvalue_field_name, fund_type.bits
                    ));
                }
            } else {
                let rust_type_name =
                    zserio_to_rust_type(&fund_type.name).expect("failed to determine native type");
                function.line(format!(
                    "{} = ztype::read_{}(reader);",
                    lvalue_field_name, rust_type_name
                ));
            }
        }
    } else {
        panic!("unexpected zserio data type")
    }
}

pub fn decode_field(
    scope: &ModelScope,
    function: &mut Function,
    field: &Field,
    context_node_index: Option<u8>,
) {
    let native_type = get_fundamental_type(&field.field_type, scope);
    let _fund_type = native_type.fundamental_type;
    let mut rvalue_field_name = format!("self.{}", convert_field_name(&field.name));
    let mut lvalue_field_name = rvalue_field_name.clone();

    // TODO alignment

    if field.is_optional {
        function.line("let present = reader.read_bool().unwrap();");
        function.line("if present {");

        // In case the field is optional, create a local variable
        // to store the value temporarly.
        let mut field_type = ztype_to_rust_type(&field.field_type);
        if field.array.is_some() {
            field_type = format!("Vec<{}>", field_type.as_str());
        }

        if native_type.is_marshaler {
            // Todo this currently doesn't work for arrays
            if field.array.is_some() {
                function.line("let mut optional_value = vec![];");
            } else {
                function.line(format!("let mut optional_value = {}::new();", field_type));
            }

            rvalue_field_name = "optional_value".into();
        } else {
            lvalue_field_name = format!("let optional_value: {}", field_type);
        }
    }

    if field.array.is_some() {
        // Array fields need to be serialized using the array class, which takes
        // care of the array delta compression.

        // TODO support @index operator

        function.line(format!(
            "{} = {}.zserio_read(reader);",
            rvalue_field_name,
            array_type_name(&field.name)
        ));
    } else {
        decode_type(
            scope,
            function,
            &lvalue_field_name,
            &rvalue_field_name,
            &field.field_type,
            context_node_index,
        );
    }

    if field.is_optional {
        function.line(format!(
            "self.{} = Option::from(optional_value);",
            convert_field_name(&field.name)
        ));
        function.line("}"); // close the "if present {"
    }
}
