use codegen::Function;

use crate::internal::ast::field::Field;
use crate::internal::generator::native_type::get_fundamental_type;
use crate::internal::generator::types::{convert_name, zserio_to_rust_type};

use crate::internal::generator::array::array_type_name;

pub fn encode_field(function: &mut Function, field: &Field) {
    let native_type = get_fundamental_type(&field.field_type);
    let fund_type = native_type.fundamental_type;
    let mut field_name = format!("self.{}", convert_name(&field.name));

    if field.is_optional {
        function.line(format!("match {} {{", field_name));
        function.line("Some(x) => {");
        function.line("let _ = writer.write_bool(true);");
        field_name = "x".into();
    }

    if field.array.is_some() {
        // Array fields need to be deserialized using the array class, which takes
        // care of the array delta compression.

        // TODO support @index operator

        function.line(format!(
            "{}.marshal_zserio(writer, &{});",
            array_type_name(&field.name),
            field_name
        ));
    } else if native_type.is_marshaler {
        function.line(format!("{}.marshal_zserio(writer);", field_name));
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
    } else if fund_type.name == "string" {
        // string types
        function.line(format!(
            "ztype::write_string(writer, {}.as_str());",
            field_name
        ));
    } else if fund_type.name == "bool" {
        // boolean
        function.line(format!(
            "let _ = writer.write_bool({}).unwrap();",
            field_name
        ));
    } else {
        // for "standard" fixed-width (unsigned) integer types, e.g. int32, uint64
        let rust_type_name =
            zserio_to_rust_type(&fund_type.name).expect("failed to determine native type");
        function.line(format!(
            "ztype::write_{}(writer, {});",
            rust_type_name, field_name
        ));
    }

    if field.is_optional {
        function.line("},");
        // write a "0" if the field is not set.
        function.line("None => writer.write_bool(false).unwrap(), ");
        function.line("};");
    }
}
