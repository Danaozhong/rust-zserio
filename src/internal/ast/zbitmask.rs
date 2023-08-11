use super::type_reference::TypeReference;
use crate::internal::ast::expression::Expression;
use crate::internal::compiler::symbol_scope::{ModelScope, PackageScope, Symbol};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]

pub struct ZBitmaskValue {
    pub name: String,
    pub value: Option<Rc<RefCell<Expression>>>,
}

#[derive(Debug)]

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

pub fn add_bitmask_to_scope(
    z_bitmask: &Rc<RefCell<ZBitmaskType>>,
    package_scope: &mut PackageScope,
) {
    // Add the bitmask name to the package scope.
    package_scope.file_symbols.insert(
        z_bitmask.borrow().name.clone(),
        Symbol::Bitmask(z_bitmask.clone()),
    );
}
