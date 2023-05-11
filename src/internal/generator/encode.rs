use codegen::Function;

use crate::internal::ast::field::Field;
use crate::internal::generator::native_type::get_fundamental_type;
use crate::internal::generator::{
    types::zserio_to_rust_type,
    types::convert_name,
};

use super::types::zserio_type_bit_size;

pub fn encode_field(function: &mut Function, field: &Field) {
    let native_type = get_fundamental_type(&*field.field_type);
    let fund_type = native_type.fundamental_type;
    let mut field_name = format!("self.{}", convert_name(&field.name));
    
    if field.is_optional {
        function.line(format!("match self.{} {{", field.name));
        function.line("None => writer.write_bool(false), ");
        function.line("Some(x) => {");
        function.line("writer.write_bool(true);");

        field_name = "x".into();

    }
    
    if native_type.is_marshaler {
        function.line(format!("{}.marshal_zserio(writer);", field_name));
    } else {
        if fund_type.bits != 0 {
            if fund_type.name == "int" {
                function.line(format!("writer.write_signed_bits({}, {});", field_name, fund_type.bits));
            } else {
                function.line(format!("writer.write_unsigned_bits({}, {});", field_name, fund_type.bits));
            }
        } else if fund_type.name == "string" {
            // string types
            function.line(format!("writer.write_string({});", field_name));
        } else if fund_type.name == "bool" {
            // boolean
            function.line(format!("writer.write_bool({});", field_name));
        } else {
            // for "standard" fixed-width (unsigned) integer types, e.g. int32, uint64 
            let rust_type_name = zserio_to_rust_type(&fund_type.name).expect("failed to determine native type");
            let zserio_type_bit_size = zserio_type_bit_size(&fund_type.name).expect("failed to identify bit length");
            function.line(
                format!("writer.write_{}({}, {});", 
                rust_type_name, 
                field_name, 
                zserio_type_bit_size));
        }
    }

    if field.is_optional {
        function.line("},");
        function.line("};");
    }

}