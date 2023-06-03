use crate::internal::ast::{field::Field, parameter::Parameter, zfunction::ZFunction};

pub struct ZUnion {
    pub name: String,
    pub comment: String,
    pub template_parameters: Vec<String>,
    pub type_parameters: Vec<Parameter>,
    pub fields: Vec<Field>,
    pub functions: Vec<ZFunction>,
}
