use codegen::Function;

use crate::internal::ast::{field::Array, field::Field, parameter::Parameter};

use crate::internal::ast::type_reference::TypeReference;
use crate::internal::compiler::fundamental_type::get_fundamental_type;
use crate::internal::compiler::symbol_scope::ModelScope;
use crate::internal::generator::types::{convert_field_name, TypeGenerator};

pub fn new_field(
    scope: &ModelScope,
    type_generator: &TypeGenerator,
    function: &mut Function,
    field: &Field,
) {
    new_type(
        scope,
        type_generator,
        function,
        &field.name,
        &field.field_type,
        field.is_optional,
        &field.array,
    );
}

pub fn new_param(
    scope: &ModelScope,
    type_generator: &TypeGenerator,
    function: &mut Function,
    param: &Parameter,
) {
    new_type(
        scope,
        type_generator,
        function,
        &param.name,
        &param.zserio_type,
        false,
        &None,
    );
}

pub fn get_default_initializer(
    is_optional: bool,
    is_array: bool,
    is_marshaler: bool,
    fund_type_name: &String,
    rust_type: &String,
) -> String {
    if is_optional {
        return "None".into();
    } else if is_array {
        return "vec![]".into();
    } else if is_marshaler {
        return format!("{}::new()", rust_type);
    } else if fund_type_name == "string" {
        // string types
        return "\"\".into()".into();
    } else if fund_type_name == "bool" {
        // boolean
        return "false".into();
    } else if fund_type_name == "extern" {
        return "ztype::ExternType{ bit_size: 0, data_blob: vec![] }".into();
    } else if fund_type_name == "bytes" {
        return "ztype::BytesType{ byte_size: 0, data_blob: vec![] }".into();
    } else if fund_type_name == "float16"
        || fund_type_name == "float32"
        || fund_type_name == "float64"
    {
        return "0.0".into();
    }
    // must be an integer type
    "0".into()
}
pub fn new_type(
    scope: &ModelScope,
    type_generator: &TypeGenerator,
    function: &mut Function,
    name: &str,
    type_reference: &TypeReference,
    is_optional: bool,
    array: &Option<Array>,
) {
    let native_type = get_fundamental_type(type_reference, scope);
    let fund_type = native_type.fundamental_type;
    let field_name = convert_field_name(name);
    let rust_type = type_generator.ztype_to_rust_type(type_reference);

    function.line(format!(
        "{}: {},",
        field_name,
        get_default_initializer(
            is_optional,
            array.is_some(),
            native_type.is_marshaler,
            &fund_type.name,
            &rust_type
        ),
    ));
}
