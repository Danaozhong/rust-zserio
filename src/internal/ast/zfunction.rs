use crate::internal::ast::expression::Expression;
use crate::internal::ast::type_reference::TypeReference;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ZFunction {
    pub name: String,
    pub result: Option<Rc<RefCell<Expression>>>,
    pub return_type: Option<Box<TypeReference>>,
}
