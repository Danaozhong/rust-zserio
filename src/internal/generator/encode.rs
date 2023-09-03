use crate::internal::ast::{field::Field, type_reference::TypeReference};
use crate::internal::compiler::fundamental_type::{
    get_fundamental_type, FundamentalZserioTypeReference,
};
use crate::internal::compiler::symbol_scope::ModelScope;
use crate::internal::generator::expression::{generate_boolean_expression, generate_expression};
use crate::internal::generator::pass_parameters::{
    does_expression_contains_index_operator, get_type_parameter,
};
use crate::internal::generator::types::{convert_field_name, TypeGenerator};
use crate::internal::generator::{array::array_type_name, array::initialize_array_trait};
use codegen::Function;

pub fn encode_type(
    scope: &ModelScope,
    type_generator: &TypeGenerator,
    function: &mut Function,
    field_name: &String,
    field_type: &TypeReference,
    context_node_index: Option<u8>,
) {
    let native_type = get_fundamental_type(field_type, scope);
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
            // Write a bit buffer (extern type)
            function.line(format!(
                "ztype::write_extern_type(writer, &{});",
                field_name
            ));
        } else if fund_type.name == "bytes" {
            // Write a byte buffer (bytes type)
            function.line(format!("ztype::write_bytes_type(writer, &{});", field_name));
        } else if fund_type.name == "bool" {
            // Write a single boolean type
            function.line(format!(
                "let _ = writer.write_bool({}).unwrap();",
                field_name
            ));
        } else if let Some(node_idx) = context_node_index {
            // packed encoding
            function.line(format!(
                "context_node.children[{}].context.write(&{}, writer, &{});",
                node_idx,
                initialize_array_trait(type_generator, &fund_type),
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
            function.line(format!(
                "ztype::write_{}(writer, {});",
                &fund_type.name, field_name
            ));
        }
    } else {
        panic!("unexpected type")
    }
}

pub fn requires_borrowing(native_type: &FundamentalZserioTypeReference) -> bool {
    if native_type.is_marshaler {
        return true;
    }
    if !native_type.fundamental_type.is_builtin {
        return true;
    }
    if native_type.fundamental_type.name == "string" {
        return true;
    }
    false
}

pub fn encode_field(
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
            generate_boolean_expression(&optional_clause.borrow(), type_generator)
        ));
    }

    // Align the byte stream, if alignment is specified.
    if field.alignment != 0 {
        function.line(format!("writer.align({});", field.alignment));
    }

    if field.is_optional {
        // If the type is a marshaller, take it by reference.
        let mut borrow_symbol = String::from("");
        if requires_borrowing(&native_type) {
            borrow_symbol = "&".into();
        }

        function.line(format!(
            "if let Some(x) = {}{} {{",
            borrow_symbol, field_name
        ));
        function.line("writer.write_bool(true).expect(\"failed to write bool\");");
        field_name = "x".into();
    }

    // Parameter check: by design decision, the objects passed to the encoding
    // part are not mutable. As such, we cannot pass the parameters here ourselves.
    // We can, however, ensure, that the parameters were correctly set, and trigger
    // an error if they were not set correctly.
    if !native_type.fundamental_type.type_arguments.is_empty() {
        let type_parameters = get_type_parameter(scope, &native_type.fundamental_type);

        // Check if parameters are passed to an array. If yes, a for loop is required.
        let mut element_name = field_name.clone();

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
                    "for (param_index, element) in {}.iter().enumerate() {{",
                    field_name
                ));
            } else {
                function.line(format!("for element in &{} {{", field_name));
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

            // TODO: for now, we are just using assertions.
            // In the future, this should be replaced by using
            // proper error handling, and reporting the error back to
            // the caller.
            function.line(format!(
                "assert!({}.{} == {});",
                &element_name,
                convert_field_name(&type_parameter.borrow().name),
                generate_expression(&type_argument, type_generator),
            ));
        }

        if field.array.is_some() {
            // Close the for-loop used to pass the parameters to the array elements.
            function.line("}");
        }
    }

    if let Some(array_information) = &field.array {
        // For arrays with fixed length, ensure that the array length is correct.
        if let Some(array_length_expr) = &array_information.array_length_expression {
            function.line(format!(
                "assert!({}.len() == ({}) as usize);",
                &field_name,
                &generate_expression(&array_length_expr.borrow(), type_generator),
            ));
        }

        // Array fields need to be deserialized using the array class, which takes
        // care of the array delta compression.
        function.line(format!(
            "{}.zserio_write(writer, &{});",
            array_type_name(&field.name),
            field_name
        ));
    } else {
        encode_type(
            scope,
            type_generator,
            function,
            &field_name,
            &field.field_type,
            context_node_index,
        );
    }

    if field.is_optional {
        function.line("} else {");
        // write a "0" if the field is not set.
        function.line("writer.write_bool(false).expect(\"failed to write bool\");");
        function.line("}");
    }

    // Close the optional clause.
    if field.optional_clause.is_some() {
        function.line("}");
    }
}
