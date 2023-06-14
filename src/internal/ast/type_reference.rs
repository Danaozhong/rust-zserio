use crate::internal::ast::expression::Expression;
use crate::internal::compiler::symbol_scope::ModelScope;
use std::string::String;

#[derive(Clone)]
pub struct TypeReference {
    pub is_builtin: bool,
    pub package: String,
    pub name: String,
    pub bits: i8,
    pub template_arguments: Vec<Box<TypeReference>>,
    pub type_arguments: Vec<Box<Expression>>,
    pub length_expression: Option<Box<Expression>>,
}

impl TypeReference {
    pub fn evaluate(&mut self, scope: &ModelScope) {
        for type_argument in &self.type_arguments {
            // TODO wait for expression PR to be merged
            //type_argument.evaluate();
        }

        if let Some(length_expression) = &self.length_expression {
            //length_expression.evaluate();
        }
    }
}

pub struct InstantiateType {
    pub name: String,
    pub zserio_type: Option<Box<TypeReference>>,
}
