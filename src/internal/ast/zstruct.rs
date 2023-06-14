use crate::internal::ast::{field::Field, parameter::Parameter, zfunction::ZFunction};

use crate::internal::compiler::symbol_scope::ModelScope;

pub struct ZStruct {
    pub name: String,
    pub comment: String,
    pub template_parameters: Vec<String>,
    pub type_parameters: Vec<Parameter>,
    pub fields: Vec<Field>,
    pub functions: Vec<ZFunction>,
}

impl ZStruct {
    pub fn evaluate(&mut self, scope: &ModelScope) {
        for param in &mut self.type_parameters {
            param.zserio_type.evaluate(scope);
        }
        for field in &mut self.fields {}
    }
}
