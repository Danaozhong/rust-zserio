use codegen::Function;

use crate::internal::ast::field::Field;
use crate::internal::generator::native_type::get_fundamental_type;
use crate::internal::generator::types::{
    array_type_name, convert_name, zserio_to_rust_type, get_array_trait_for_type, field_to_rust_type,
};

pub fn new_field(function: &mut Function, field: &Field) {
    let native_type = get_fundamental_type(&*field.field_type);
    let fund_type = native_type.fundamental_type;
    let field_name = convert_name(&field.name);
    let rust_type = field_to_rust_type(&field);

    if field.is_optional {
        function.line(format!("{}: None,", field_name));
    } else if let Some(array) = &field.array {
        function.line(format!("{}: vec![],", field_name));

        // also initialize the array part
        function.line(format!("{}: ztype::Array::<{}>{{", array_type_name(&field.name), rust_type));
        function.line(format!("array_trait: ztype::{}{{}},", get_array_trait_for_type(fund_type.as_ref())));
        // TODO function.line(format!("fixed_size: {},", array.array_length_expression.is_some()));
        function.line("fixed_size: None,");
        function.line(format!("is_aligned: {},", field.alignment != 0));
        function.line(format!("is_packed: {},", array.is_packed));
        function.line("packing_context_node: None,");

        function.line("},");

    } else if native_type.is_marshaler {
        function.line(format!("{}: {}::new(),", field_name, fund_type.name));
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
