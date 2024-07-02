use crate::internal::compiler::fundamental_type::FundamentalZserioTypeReference;
use crate::internal::compiler::symbol_scope::ModelScope;
use crate::internal::generator::encode::requires_borrowing;
use crate::internal::generator::expression::{generate_boolean_expression, generate_expression};
use crate::internal::generator::index_offsets::extract_indexed_offset_expression;
use crate::internal::generator::packed_contexts::FieldDetails;
use crate::internal::generator::pass_parameters::does_expression_contains_index_operator;
use crate::internal::generator::types::TypeGenerator;
use crate::internal::generator::{array::array_type_name, array::initialize_array_trait};
use codegen::Function;

pub fn bitsize_type_reference(
    scope: &ModelScope,
    type_generator: &mut TypeGenerator,
    function: &mut Function,
    field_name: &str,
    native_type: &FundamentalZserioTypeReference,
    field_index: usize,
    packed: bool,
) {
    let type_reference = native_type.fundamental_type.as_ref();
    if native_type.is_marshaler {
        if packed {
            // Use packed bitsize
            function.line(format!(
                "end_position += {}.zserio_bitsize_packed(&mut context_node.children[{}], end_position).unwrap();",
                field_name,
                field_index,
            ));
        } else {
            function.line(format!(
                "end_position += {}.zserio_bitsize(end_position).unwrap();",
                field_name
            ));
        }
    } else if type_reference.is_builtin {
        if type_reference.name == "string" {
            // string types
            function.line(format!(
                "end_position += ztype::bitsize_string({}.as_str()).unwrap();",
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
        } else if packed {
            // packed bitsize
            function.line(format!(
                "end_position += context_node.children[{}].context.as_mut().unwrap().bitsize_of(&{}, end_position, &{})?;",
                field_index,
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
                    format!("ztype::varint_bitsize({}).unwrap() as u64", field_name)
                }
                "varint16" => {
                    format!("ztype::varint16_bitsize({}).unwrap() as u64", field_name)
                }
                "varint32" => {
                    format!("ztype::varint32_bitsize({}).unwrap() as u64", field_name)
                }
                "varint64" => {
                    format!("ztype::varint64_bitsize({}).unwrap() as u64", field_name)
                }
                "varuint" => {
                    format!("ztype::varuint_bitsize({}).unwrap() as u64", field_name)
                }
                "varuint16" => {
                    format!("ztype::varuint16_bitsize({}).unwrap() as u64", field_name)
                }
                "varuint32" => {
                    format!("ztype::varuint32_bitsize({}).unwrap() as u64", field_name)
                }
                "varuint64" => {
                    format!("ztype::varuint64_bitsize({}).unwrap() as u64", field_name)
                }
                "varsize" => {
                    format!("ztype::varsize_bitsize({}).unwrap() as u64", field_name)
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
    type_generator: &mut TypeGenerator,
    function: &mut Function,
    field_details: &FieldDetails,
    packed: bool,
) {
    let field = field_details.field.borrow();
    let mut field_name = format!("self.{}", field_details.field_name);

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
            function.line("end_position += ztype::align_bitsize(end_position, 8);".to_string());
        }
    }

    if field.is_optional {
        function.line("end_position += 1;");
        // If the type is a marshaller, take it by reference.
        let mut borrow_symbol = String::from("");
        if requires_borrowing(&field, &field_details.native_type) {
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
        let mut index_offset = String::from("None::<&Vec<u32>>");
        if use_indexed_offset {
            index_offset = format!("Some(&{gen_offset_expression})");
        }
        function.line(format!(
            "end_position += {}.zserio_bitsize(&{}, {}, end_position)?;",
            array_type_name, field_name, index_offset
        ));
    } else {
        bitsize_type_reference(
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
        // in case the field is optional, end the if condition which checks
        // if the field is set.
        function.line("}");
    }

    // Close the optional clause.
    if field.optional_clause.is_some() {
        function.line("}");
    }
}
