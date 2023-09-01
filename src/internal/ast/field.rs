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

    // Specifies if the field is an array
    pub array: Option<Array>,
}

impl Field {
    pub fn evaluate(&self, scope: &mut ModelScope) {
        self.field_type.evaluate(scope);
        if let Some(array) = &self.array {
            if let Some(array_length_expression) = &array.array_length_expression {
                array_length_expression.borrow_mut().evaluate(scope);
            }
        }
    }
}
