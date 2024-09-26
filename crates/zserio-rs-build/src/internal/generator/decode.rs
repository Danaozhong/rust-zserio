use crate::internal::compiler::fundamental_type::FundamentalZserioTypeReference;
use crate::internal::compiler::symbol_scope::ModelScope;
use crate::internal::generator::casts::expression_requires_cast;
use crate::internal::generator::expression::{generate_boolean_expression, generate_expression};
use crate::internal::generator::index_offsets::extract_indexed_offset_expression;
use crate::internal::generator::packed_contexts::FieldDetails;
use crate::internal::generator::pass_parameters::{
    does_expression_contains_index_operator, get_type_parameter,
};
use crate::internal::generator::types::TypeGenerator;
use codegen::Function;

use crate::internal::generator::array::{array_type_name, initialize_array_trait};

#[allow(clippy::too_many_arguments)]
pub fn decode_type(
    scope: &ModelScope,
    type_generator: &mut TypeGenerator,
    function: &mut Function,
    lvalue_field_name: &String,
    rvalue_field_name: &String,
    native_type: &FundamentalZserioTypeReference,
    field_index: usize,
    packed: bool,
) {
    let fund_type = &native_type.fundamental_type;

    if native_type.is_marshaler {
        // the field is a marshable type (struct, choice, enum)
        if packed {
            // Use packed reading
            function.line(format!(
                "{}.zserio_read_packed(&mut context_node.children[{}], reader)?;",
                rvalue_field_name, field_index,
            ));
        } else {
            // use standard reading
            function.line(format!("{}.zserio_read(reader)?;", rvalue_field_name));
        }
    } else if fund_type.is_builtin {
        // The type should be a native type
        if fund_type.name == "string" {
            // string types
            function.line(format!(
                "{} = ztype::read_string(reader)?;",
                lvalue_field_name
            ));
        } else if fund_type.name == "extern" {
            function.line(format!(
                "{} = ztype::read_extern_type(reader)?;",
                lvalue_field_name
            ));
        } else if fund_type.name == "bytes" {
            function.line(format!(
                "{} = ztype::read_bytes_type(reader)?;",
                lvalue_field_name
            ));
        } else if fund_type.name == "bool" {
            // boolean
            function.line(format!("{} = reader.read_bool()?;", lvalue_field_name));
        } else if packed {
            // packed decoding
            function.line(format!(
                "context_node.children[{}].context.as_mut().unwrap().read(&{}, reader, &mut {}, 0)?;",
                field_index,
                initialize_array_trait(scope, type_generator, fund_type),
                rvalue_field_name,
            ));
        } else {
            // non-packed decoding
            if fund_type.bits != 0 || fund_type.length_expression.is_some() {
                let bit_length_string = match &fund_type.length_expression {
                    Some(bit_length_expression) => {
                        let mut length_expression_string = generate_expression(
                            &bit_length_expression.borrow(),
                            type_generator,
                            scope,
                        );
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
                        "{} = ztype::read_signed_bits(reader, {})?;",
                        lvalue_field_name, bit_length_string
                    ));
                } else {
                    function.line(format!(
                        "{} = ztype::read_unsigned_bits(reader, {})?;",
                        lvalue_field_name, bit_length_string
                    ));
                }
            } else {
                function.line(format!(
                    "{} = ztype::read_{}(reader)?;",
                    &lvalue_field_name, &fund_type.name
                ));
            }
        }
    } else {
        panic!("unexpected zserio data type")
    }
}

pub fn decode_field(
    scope: &ModelScope,
    type_generator: &mut TypeGenerator,
    function: &mut Function,
    field_details: &FieldDetails,
    packed: bool,
) {
    let field = field_details.field.borrow();

    let mut rvalue_field_name = format!("self.{}", field_details.field_name);
    let mut lvalue_field_name = rvalue_field_name.clone();
    let raw_field_type = &field_details.rust_type;
    let mut field_type = raw_field_type.clone();

    // Check if the field uses an optional clause
    if let Some(optional_clause) = &field.optional_clause {
        function.line(format!(
            "if {} {{",
            generate_boolean_expression(&optional_clause.borrow(), type_generator, scope)
        ));
    }

    // Align the byte stream, if alignment is specified.
    if field.alignment != 0 {
        function.line(format!(
            "ztype::align_reader(reader, {})?;",
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
            function.line("ztype::align_reader(reader, 8)?;");
        }
    }

    if field.is_optional {
        function.line("let present = reader.read_bool().unwrap();");
        function.line("if present {");

        // In case the field is optional, create a local variable
        // to store the value temporarily.
        if field.array.is_some() {
            field_type = format!("Vec<{}>", field_type.as_str());
        }

        function.line(format!(
            "let mut optional_value: {} = Default::default();",
            &field_type
        ));
        lvalue_field_name = "optional_value".into();
        rvalue_field_name = "optional_value".into();
    }

    let array_type_name = array_type_name(&field.name);
    if field.array.is_some() {
        // Array fields need to be serialized using the array class, which takes
        // care of the array delta compression.
        function.line(format!(
            "let {}_array_length = {}.zserio_read_array_length(reader)?;",
            field_details.field_name, array_type_name,
        ));
        // initialize the array elements with the default value.
        function.line(format!(
            "{} = vec![Default::default(); {}_array_length];",
            rvalue_field_name, field_details.field_name,
        ));
    }

    // Pass the parameters.
    if !field_details
        .native_type
        .fundamental_type
        .type_arguments
        .is_empty()
    {
        let type_parameters =
            get_type_parameter(scope, &field_details.native_type.fundamental_type);

        // Check if parameters are passed to an array. If yes, a for loop is required.
        let mut element_name = rvalue_field_name.clone();

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
                    "for (param_index, element) in {}.iter_mut().enumerate() {{",
                    rvalue_field_name
                ));
            } else {
                function.line(format!("for element in &mut {} {{", rvalue_field_name));
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
            let mut requires_cloning = String::from("");
            if !type_argument.fully_resolved {
                requires_cloning = ".clone()".into();
            }
            let mut rvalue = format!(
                "{}{}",
                generate_expression(&type_argument, type_generator, scope),
                requires_cloning
            );

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
                "{}.{} = {};",
                element_name,
                type_generator.convert_field_name(&type_parameter.borrow().name),
                rvalue,
            ));
        }

        if field.array.is_some() {
            // Close the for-loop used to pass the parameters to the array elements.
            function.line("}");
        }
    }

    if field.array.is_some() {
        let mut index_offset = String::from("None::<&Vec<u32>>");
        if use_indexed_offset {
            index_offset = format!("Some(&{gen_offset_expression})");
        }
        function.line(format!(
            "{}.zserio_read(reader, &mut {}, {})?;",
            array_type_name, rvalue_field_name, index_offset,
        ));
    } else {
        decode_type(
            scope,
            type_generator,
            function,
            &lvalue_field_name,
            &rvalue_field_name,
            &field_details.native_type,
            field_details.field_index,
            packed && field_details.is_packable,
        );
    }

    if field.is_optional {
        function.line(format!(
            "self.{} = Option::from(optional_value);",
            type_generator.convert_field_name(&field.name)
        ));
        function.line("}"); // close the "if present {"
    }
    // Close the optional clause.
    if field.optional_clause.is_some() {
        function.line("}");
    }
}
