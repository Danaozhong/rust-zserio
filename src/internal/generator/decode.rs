use crate::internal::generator::expression::generate_expression;
use crate::internal::generator::pass_parameters::{
    does_expression_contains_index_operator, get_type_parameter,
};
use codegen::Function;

use crate::internal::ast::{field::Field, type_reference::TypeReference};
use crate::internal::compiler::fundamental_type::get_fundamental_type;
use crate::internal::compiler::symbol_scope::ModelScope;
use crate::internal::generator::new::get_default_initializer;
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
                "context_node.children[{}].context.read(&{}, reader, &mut {}, 0);",
                node_idx,
                initialize_array_trait(&fund_type),
                rvalue_field_name,
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
    let field_name = convert_field_name(&field.name);
    let mut rvalue_field_name = format!("self.{}", field_name);
    let mut lvalue_field_name = rvalue_field_name.clone();
    let mut field_type = ztype_to_rust_type(&field.field_type);
    // TODO alignment

    if field.is_optional {
        function.line("let present = reader.read_bool().unwrap();");
        function.line("if present {");

        // In case the field is optional, create a local variable
        // to store the value temporarily.
        if field.array.is_some() {
            field_type = format!("Vec<{}>", field_type.as_str());
        }

        if native_type.is_marshaler {
            if field.array.is_some() {
                function.line("let mut optional_value = vec![];");
            } else {
                function.line(format!("let mut optional_value = {}::new();", field_type));
            }
        } else {
            let default_value = get_default_initializer(
                false, // We have already checked if the field is optional.
                false, // The underlying type will never be an array (no multidimensional array support in zserio).
                native_type.is_marshaler,
                &native_type.fundamental_type.name,
                &field_type,
            );
            function.line(format!(
                "let mut optional_value: {} = {};",
                &field_type, &default_value
            ));
        }
        lvalue_field_name = "optional_value".into();
        rvalue_field_name = "optional_value".into();
    }

    let array_type_name = array_type_name(&field.name);
    if field.array.is_some() {
        // Array fields need to be serialized using the array class, which takes
        // care of the array delta compression.

        // Read the length of the array, either from the bit stream,
        // or from the array definition in zserio.
        function.line(format!(
            "let {}_array_length = {}.zserio_read_array_length(reader);",
            field_name, array_type_name,
        ));

        // initialize the array elements with empty values.
        let default_value = get_default_initializer(
            false, // The underlying type will never be optional (already checked).
            false, // The underlying type will never be an array (no 2D array support in zserio).
            native_type.is_marshaler,
            &native_type.fundamental_type.name,
            &field_type,
        );
        function.line(format!(
            "{} = vec![{}; {}_array_length];",
            rvalue_field_name, default_value, field_name,
        ));
    }

    // Pass the parameters.
    if !native_type.fundamental_type.type_arguments.is_empty() {
        let type_parameters = get_type_parameter(scope, &native_type.fundamental_type);

        // Check if parameters are passed to an array. If yes, a for loop is required.
        let mut element_name = rvalue_field_name.clone();

        if field.array.is_some() {
            element_name = String::from("element");
            let mut parameter_passing_uses_index_operator = false;
            for type_param in &native_type.fundamental_type.type_arguments {
                if does_expression_contains_index_operator(&type_param.borrow()) {
                    parameter_passing_uses_index_operator = true;
                    break;
                }
            }
            // Depending on if the @index operator is used, the for loop need an index variable,
            // to be able to assign the parameters to the correct index.
            if parameter_passing_uses_index_operator {
                function.line(format!(
                    "for (param_index, element) in {}.iter_mut().enumerate() {{",
                    rvalue_field_name
                ));
            } else {
                function.line(format!("for element in &mut {} {{", rvalue_field_name));
            }
        }

        for (param_index, type_argument_rc) in native_type
            .fundamental_type
            .type_arguments
            .iter()
            .enumerate()
        {
            let type_argument = type_argument_rc.borrow();
            let type_parameter = &type_parameters[param_index];

            function.line(format!(
                "{}.{} = {}.clone();",
                element_name,
                convert_field_name(&type_parameter.borrow().name),
                generate_expression(&type_argument),
            ));
        }

        if field.array.is_some() {
            // Close the for-loop used to pass the parameters to the array elements.
            function.line("}");
        }
    }

    if field.array.is_some() {
        function.line(format!(
            "{}.zserio_read(reader, &mut {});",
            array_type_name, rvalue_field_name,
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
