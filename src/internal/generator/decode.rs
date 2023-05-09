use codegen::Function;

use crate::internal::ast::field::Field;
use crate::internal::generator::native_type::get_fundamental_type;
use crate::internal::generator::{
    types::zserio_to_rust_type,
    types::convert_name,
};

pub fn decode_field(function: &mut Function, field: &Field) {
    let native_type = get_fundamental_type(&*field.field_type);
    let fund_type = native_type.fundamental_type;
    let mut field_name = format!("self.{}", convert_name(&field.name));
    

    // TODO optional clause

    // TODO alignment

    if field.is_optional {
        function.line("let present = reader.read_bool();");
        function.line("if present {");
    }
    
    if native_type.is_marshaler {
        function.line(format!("{}.unmarshal_zserio(reader);", field_name));
    } else {
        if fund_type.bits != 0 {
            if fund_type.name == "int" {
                function.line(format!("reader.read_signed_bits({}, {});", field_name, fund_type.bits));
            } else {
                function.line(format!("reader.reader_unsigned_bits({}, {});", field_name, fund_type.bits));
            }
        } else {
            let rust_type_name = zserio_to_rust_type(&fund_type.name).expect("failed to determine native type");
            function.line(format!("reader.reader_{}({});", rust_type_name, field_name));
        }
    }

    if field.is_optional {
        function.line("}"); // close the "if present {"
    }

}