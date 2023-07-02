use crate::internal::ast::expression::Expression;
use crate::internal::compiler::symbol_scope::ModelScope;
use std::cell::RefCell;
use std::rc::Rc;
use std::string::String;

#[derive(Clone)]
pub struct TypeReference {
    pub is_builtin: bool,
    pub package: String,
    pub name: String,
    pub bits: i8,
    pub template_arguments: Vec<Box<TypeReference>>,
    pub type_arguments: Vec<Rc<RefCell<Expression>>>,
    pub length_expression: Option<Rc<RefCell<Expression>>>,
}

impl TypeReference {
    pub fn evaluate(&self, scope: &mut ModelScope) {
        for type_argument in &self.type_arguments {
            type_argument.as_ref().borrow_mut().evaluate(scope);
        }

        if let Some(length_expression) = &self.length_expression {
            length_expression.as_ref().borrow_mut().evaluate(scope);
        }
    }
}

pub struct InstantiateType {
    pub name: String,
    pub zserio_type: Box<TypeReference>,
}
