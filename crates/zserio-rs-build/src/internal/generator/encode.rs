use crate::internal::ast::field::Field;
use crate::internal::compiler::fundamental_type::FundamentalZserioTypeReference;
use crate::internal::compiler::symbol_scope::ModelScope;
use crate::internal::generator::array::initialize_array_trait;
use crate::internal::generator::casts::expression_requires_cast;
use crate::internal::generator::expression::{generate_boolean_expression, generate_expression};
use crate::internal::generator::index_offsets::extract_indexed_offset_expression;
use crate::internal::generator::packed_contexts::FieldDetails;
use crate::internal::generator::pass_parameters::{
    does_expression_contains_index_operator, get_type_parameter,
};
use crate::internal::generator::types::TypeGenerator;

use codegen::Function;

pub fn encode_type(
    scope: &ModelScope,
    type_generator: &mut TypeGenerator,
    function: &mut Function,
    field_name: &String,
    native_type: &FundamentalZserioTypeReference,
    field_index: usize,
    packed: bool,
) {
    let fund_type = &native_type.fundamental_type;
    if native_type.is_marshaler {
        if packed {
            // Use packed reading
            function.line(format!(
                "{}.zserio_write_packed(&mut context_node.children[{}], writer)?;",
                field_name, field_index,
            ));
        } else {
            function.line(format!("{}.zserio_write(writer)?;", field_name));
        }
    } else if fund_type.is_builtin {
        if fund_type.name == "string" {
            // string types
            function.line(format!(
                "ztype::write_string(writer, {}.as_str())?;",
                field_name
            ));
        } else if fund_type.name == "extern" {
            // Write a bit buffer (extern type)
            function.line(format!(
                "ztype::write_extern_type(writer, &{})?;",
                field_name
            ));
        } else if fund_type.name == "bytes" {
            // Write a byte buffer (bytes type)
            function.line(format!(
                "ztype::write_bytes_type(writer, &{})?;",
                field_name
            ));
        } else if fund_type.name == "bool" {
            // Write a single boolean type
            function.line(format!("writer.write_bool({})?;", field_name));
        } else if packed {
            // packed encoding
            function.line(format!(
                "context_node.children[{}].context.as_mut().unwrap().write(&{}, writer, &{})?;",
                field_index,
                initialize_array_trait(scope, type_generator, fund_type),
                field_name,
            ));
        } else if fund_type.bits != 0 || fund_type.length_expression.is_some() {
            let bit_length_string = match &fund_type.length_expression {
                Some(bit_length_expression) => {
                    let mut length_expression_string =
                        generate_expression(&bit_length_expression.borrow(), type_generator, scope);
                    // check if there is a typecast needed
                    if let Some(native_type) = &bit_length_expression.borrow().native_type {
                        if native_type.name != "uint8" {
                            length_expression_string =
                                format!("({}) as u8", length_expression_string);
                        }
                    }
                    length_expression_string
                }
                None => format!("{}", fund_type.bits),
            };
            if fund_type.name == "int" {
                function.line(format!(
                    "ztype::write_signed_bits(writer, {}, {})?;",
                    field_name, bit_length_string
                ));
            } else {
                function.line(format!(
                    "ztype::write_unsigned_bits(writer, {}, {})?;",
                    field_name, bit_length_string
                ));
            }
        } else {
            // for "standard" fixed-width (unsigned) integer types, e.g. int32, uint64
            function.line(format!(
                "ztype::write_{}(writer, {})?;",
                &fund_type.name, field_name
            ));
        }
    } else {
        panic!("unexpected type")
    }
}

/// Decides if a type needs to be passed as a reference, or by copy.
/// Returns true if the type should be passed by reference, otherwise false.
pub fn requires_borrowing(field: &Field, native_type: &FundamentalZserioTypeReference) -> bool {
    if field.array.is_some() {
        return true;
    }
    if native_type.is_marshaler {
        return true;
    }
    // all non-built-in types should be passed by reference.
    if !native_type.fundamental_type.is_builtin {
        return true;
    }
    // for the native core types, strings make sense to pass by reference.
    if native_type.fundamental_type.name == "string" {
        return true;
    }
    false
}

pub fn encode_field(
    scope: &ModelScope,
    type_generator: &mut TypeGenerator,
    function: &mut Function,
    field_details: &FieldDetails,
    packed: bool,
) {
    let mut field_name = format!("self.{}", field_details.field_name);
    let field = field_details.field.borrow();

    // Check if the field uses an optional clause. If yes, open a condition.
    if let Some(optional_clause) = &field_details.field.borrow().optional_clause {
        function.line(format!(
            "if {} {{",
            generate_boolean_expression(&optional_clause.borrow(), type_generator, scope)
        ));
    }

    // Align the byte stream, if alignment is specified.
    if field.alignment != 0 {
        function.line(format!(
            "ztype::align_writer(writer, {})?;",
            field.alignment
        ));
    }

    // Check if there is an offset set for this field.
    let mut gen_offset_expression = String::new();
    let mut use_indexed_offset = false;
    if let Some(offset) = &field.offset {
        let offset_expr = &offset.borrow();
        use_indexed_offset = does_expression_contains_index_operator(offset_expr);
        gen_offset_expression = generate_expression(
            &extract_indexed_offset_expression(offset_expr),
            type_generator,
            scope,
        );

        if !use_indexed_offset {
            function.line("ztype::align_writer(writer, 8)?;".to_string());
        }
    }

    let mut field_is_borrowed = false;
    if field.is_optional {
        // If the type is a marshaller, take it by reference.
        field_is_borrowed = requires_borrowing(&field, &field_details.native_type);

        let mut borrow_symbol = String::from("");
        if field_is_borrowed {
            borrow_symbol = "&".into();
        }

        function.line(format!(
            "if let Some(x) = {}{} {{",
            borrow_symbol, field_name
        ));
        function.line("writer.write_bool(true)?;");
        field_name = "x".into();
    }

    // Parameter check: by design decision, the objects passed to the encoding
    // part are not mutable. As such, we cannot pass the parameters here ourselves.
    // We can, however, ensure, that the parameters were correctly set, and trigger
    // an error if they were not set correctly.
    if !field_details
        .native_type
        .fundamental_type
        .type_arguments
        .is_empty()
    {
        let type_parameters =
            get_type_parameter(scope, &field_details.native_type.fundamental_type);

        // Check if parameters are passed to an array. If yes, a for loop is required.
        let mut element_name = field_name.clone();

        if field.array.is_some() {
            element_name = String::from("element");
            let mut parameter_passing_uses_index_operator = false;
            for type_param in &field_details.native_type.fundamental_type.type_arguments {
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
                let mut borrow_symbol = String::from("");
                if !field_is_borrowed {
                    borrow_symbol = "&".into();
                }
                function.line(format!("for element in {}{} {{", borrow_symbol, field_name));
            }
        }

        for (param_index, type_argument_rc) in field_details
            .native_type
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

            // Check if casts are needed.
            let mut rvalue = generate_expression(&type_argument, type_generator, scope);
            if expression_requires_cast(
                &type_parameter.borrow().zserio_type,
                type_generator,
                &type_argument,
            ) {
                rvalue = format!(
                    "{} as {}",
                    rvalue,
                    type_generator.ztype_to_rust_type(&type_parameter.borrow().zserio_type)
                );
            }
            function.line(format!(
                "assert!({}.{} == {});",
                &element_name,
                TypeGenerator::convert_field_name(&type_parameter.borrow().name),
                rvalue,
            ));
        }

        if field_details.field.borrow().array.is_some() {
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
                &generate_expression(&array_length_expr.borrow(), type_generator, scope),
            ));
        }

        // Array fields need to be deserialized using the array class, which takes
        // care of the array delta compression.
        let mut index_offset = String::from("None::<&Vec<u32>>");
        if use_indexed_offset {
            index_offset = format!("Some(&{gen_offset_expression})");
        }
        function.line(format!(
            "{}.zserio_write(writer, &{}, {})?;",
            field_details.rust_array_type_name, field_name, index_offset
        ));
    } else {
        encode_type(
            scope,
            type_generator,
            function,
            &field_name,
            &field_details.native_type,
            field_details.field_index,
            packed && field_details.is_packable,
        );
    }

    if field.is_optional {
        function.line("} else {");
        // write a "0" if the field is not set.
        function.line("writer.write_bool(false)?;");
        function.line("}");
    }

    // Close the optional clause.
    if field.optional_clause.is_some() {
        function.line("}");
    }
}
