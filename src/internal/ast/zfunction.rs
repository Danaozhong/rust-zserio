use crate::internal::ast::expression::Expression;
use crate::internal::ast::type_reference::TypeReference;
use crate::internal::compiler::symbol_scope::{ModelScope, PackageScope, Symbol};
use std::cell::RefCell;
use std::rc::Rc;

pub struct ZFunction {
    pub name: String,
    pub result: Rc<RefCell<Expression>>,
    pub return_type: Box<TypeReference>,
}

impl ZFunction {
    pub fn evaluate(&mut self, scope: &mut ModelScope) {
        self.result.as_ref().borrow_mut().evaluate(scope);
        if let Some(length_expression) = &self.return_type.length_expression {
            length_expression.as_ref().borrow_mut().evaluate(scope);
        }
    }
}
