use crate::internal::ast::{expression::Expression, type_reference::TypeReference};
use crate::internal::compiler::symbol_scope::ModelScope;
use std::cell::RefCell;
use std::rc::Rc;
use std::string::String;

#[derive(Clone, Debug)]
pub struct Array {
    pub is_packed: bool,
    pub is_implicit: bool,
    pub array_length_expression: Option<Rc<RefCell<Expression>>>,
}

#[derive(Clone, Debug)]
pub struct Field {
    pub name: String,
    pub zserio_name: String,
    pub comment: String,
    pub is_optional: bool,
    pub alignment: u8,
    pub field_type: Box<TypeReference>,
    pub initializer: Option<Rc<RefCell<Expression>>>,
    pub constraint: Option<Rc<RefCell<Expression>>>,
    pub optional_clause: Option<Rc<RefCell<Expression>>>,

    // Specifies if the field is an array
    pub array: Option<Array>,
}

impl Field {
    pub fn evaluate(&mut self, scope: &mut ModelScope) {
        self.field_type.evaluate(scope);
        if let Some(array) = &self.array {
            if let Some(array_length_expression) = &array.array_length_expression {
                array_length_expression.borrow_mut().evaluate(scope);
            }
        }
        if let Some(optional_clause) = &self.optional_clause {
            optional_clause.borrow_mut().evaluate(scope);
        }
        if let Some(initializer) = &mut self.initializer {
            initializer.borrow_mut().evaluate(scope);
        }
    }
}
