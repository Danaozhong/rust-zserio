use codegen::Function;

use crate::internal::ast::field::Field;

use crate::internal::generator::native_type::get_fundamental_type;
use crate::internal::generator::types::{convert_name, ztype_to_rust_type};

pub fn new_field(function: &mut Function, field: &Field) {
    let native_type = get_fundamental_type(&field.field_type);
    let fund_type = native_type.fundamental_type;
    let field_name = convert_name(&field.name);
    let rust_type = ztype_to_rust_type(field.field_type.as_ref());

    if field.is_optional {
        function.line(format!("{}: None,", field_name));
    } else if let Some(_array) = &field.array {
        function.line(format!("{}: vec![],", field_name));
    } else if native_type.is_marshaler {
        function.line(format!("{}: {}::new(),", field_name, rust_type));
    } else {
        // fundamental types
        if fund_type.name == "string" {
            // string types
            function.line(format!("{}: \"\".into(),", field_name));
        } else if fund_type.name == "bool" {
            // boolean
            function.line(format!("{}: false,", field_name));
        } else {
            // must be an integer type
            function.line(format!("{}: 0,", field_name));
        }
    }
}
