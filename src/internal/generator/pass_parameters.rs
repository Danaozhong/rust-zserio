use codegen::Function;
use crate::internal::ast::{
    field::Field,
    parameter::Parameter,
    expression::Expression,
    
    
};


pub fn pass_field_parameters(code_gen_fn: &mut Function, fields: &Vec<Field>) {
    for field in fields {
        if field.field_type.type_arguments.is_empty() {
            continue;
        }
        // TODO need to resolve the type, and retrieve the parameters
        pass_parameters(code_gen_fn, &field.name, &field.field_type.type_arguments, &vec![]);

    }
}

pub fn pass_parameters(code_gen_fn: &mut Function, field_name: &str, arguments: &Vec<Box<Expression>>, params: &Vec<Parameter>) {
    assert!(arguments.len() == params.len(), "argument count does not match the number of parameters");

    for param in params {
        code_gen_fn.line(format!("{} = {}", field_name, param.name));
    }
}