use codegen::Function;

use crate::internal::ast::field::Field;
use crate::internal::generator::native_type::get_fundamental_type;
use crate::internal::generator::types::{array_type_name, convert_name, zserio_to_rust_type};

pub fn decode_field(function: &mut Function, field: &Field) {
    let native_type = get_fundamental_type(&*field.field_type);
    let fund_type = native_type.fundamental_type;
    let mut rvalue_field_name = format!("self.{}", convert_name(&field.name));
    let mut lvalue_field_name = rvalue_field_name.clone();

    // TODO optional clause

    // TODO alignment

    if field.is_optional {
        function.line("let present = reader.read_bool().unwrap();");
        function.line("if present {");
        lvalue_field_name = String::from("let optional_value");
    }

    if field.array.is_some() {
        // Array fields need to be serialized using the array class, which takes
        // care of the array delta compression.

        // TODO support @index operator

        function.line(format!(
            "{} = self.{}.unmarshal_zserio(reader);",
            lvalue_field_name,
            array_type_name(&field.name)
        ));
    } else if native_type.is_marshaler {
        // the field is a marshable type (struct, choice, enum)
        function.line(format!("{}.unmarshal_zserio(reader);", rvalue_field_name));
    } else if fund_type.is_builtin {
        // The type should be a native type
        if fund_type.bits != 0 {
            if fund_type.name == "int" {
                function.line(format!(
                    "{} = ztype::read_signed_bits(reader, {});",
                    lvalue_field_name, fund_type.bits
                ));
            } else {
                function.line(format!(
                    "{} = ztype::reader_unsigned_bits(reader, {});",
                    lvalue_field_name, fund_type.bits
                ));
            }
        } else if fund_type.name == "string" {
            // string types
            function.line(format!(
                "{} = ztype::read_string(reader);",
                lvalue_field_name
            ));
        } else if fund_type.name == "bool" {
            // boolean
            function.line(format!("{} = reader.read_bool();", lvalue_field_name));
        } else {
            let rust_type_name =
                zserio_to_rust_type(&fund_type.name).expect("failed to determine native type");
            function.line(format!(
                "{} = ztype::read_{}(reader);",
                lvalue_field_name, rust_type_name
            ));
        }
    } else {
        panic!("unexpected zserio data type")
    }

    if field.is_optional {
        function.line(format!(
            "{} = Option::from(optional_value);",
            rvalue_field_name
        ));
        function.line("}"); // close the "if present {"
    }
}
