use super::type_reference::TypeReference;
use crate::internal::ast::expression::Expression;
use crate::internal::compiler::symbol_scope::ModelScope;
use std::cell::RefCell;
use std::rc::Rc;
pub struct ZBitmaskValue {
    pub name: String,
    pub value: Option<Rc<RefCell<Expression>>>,
}

pub struct ZBitmaskType {
    pub name: String,
    pub zserio_type: Box<TypeReference>,
    pub values: Vec<ZBitmaskValue>,
}

impl ZBitmaskType {
    pub fn evaluate(&mut self, scope: &mut ModelScope) {
        for value in &mut self.values {
            if let Some(value_expression) = &mut value.value {
                value_expression.as_ref().borrow_mut().evaluate(scope);
            }
        }
        self.zserio_type.evaluate(scope);
    }
}
