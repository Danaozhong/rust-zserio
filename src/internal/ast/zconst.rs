use super::type_reference::TypeReference;
use crate::internal::ast::expression::Expression;
use crate::internal::compiler::symbol_scope::{ModelScope, PackageScope, Symbol};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]

pub struct ZConst {
    pub name: String,
    pub comment: String,
    pub zserio_type: Box<TypeReference>,
    pub value_expression: Rc<RefCell<Expression>>,
}

impl ZConst {
    pub fn evaluate(&mut self, scope: &mut ModelScope) {
        self.value_expression.as_ref().borrow_mut().evaluate(scope);
    }
}

pub fn add_const_to_scope(z_const: &Rc<RefCell<ZConst>>, package_scope: &mut PackageScope) {
    // Add the const name to the package scope.
    package_scope.file_symbols.insert(
        z_const.borrow().name.clone(),
        Symbol::Const(z_const.clone()),
    );
}
