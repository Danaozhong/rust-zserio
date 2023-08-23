use codegen::Function;

use crate::internal::ast::{field::Array, field::Field, parameter::Parameter};

use crate::internal::ast::type_reference::TypeReference;
use crate::internal::compiler::fundamental_type::get_fundamental_type;
use crate::internal::compiler::symbol_scope::ModelScope;
use crate::internal::generator::types::{convert_field_name, ztype_to_rust_type};

pub fn new_field(scope: &ModelScope, function: &mut Function, field: &Field) {
    new_type(
        scope,
        function,
        &field.name,
        &field.field_type,
        field.is_optional,
        &field.array,
    );
}

pub fn new_param(scope: &ModelScope, function: &mut Function, param: &Parameter) {
    new_type(
        scope,
        function,
        &param.name,
        &param.zserio_type,
        false,
        &None,
    );
}

pub fn new_type(
    scope: &ModelScope,
    function: &mut Function,
    name: &String,
    type_reference: &TypeReference,
    is_optional: bool,
    array: &Option<Array>,
) {
    let native_type = get_fundamental_type(type_reference, scope);
    let fund_type = native_type.fundamental_type;
    let field_name = convert_field_name(name);
    let rust_type = ztype_to_rust_type(type_reference);

    if is_optional {
        function.line(format!("{}: None,", field_name));
    } else if let Some(_array) = array {
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
        } else if type_reference.name == "extern" {
            function.line(format!(
                "{}: ztype::ExternType{{ bit_size: 0, data_blob: vec![] }},",
                field_name
            ));
        } else if type_reference.name == "bytes" {
            function.line(format!(
                "{}: ztype::BytesType{{ byte_size: 0, data_blob: vec![] }},",
                field_name
            ));
        } else {
            // must be an integer type
            function.line(format!("{}: 0,", field_name));
        }
    }
}
