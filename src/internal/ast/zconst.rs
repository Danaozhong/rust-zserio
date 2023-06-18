use super::type_reference::TypeReference;
use crate::internal::ast::expression::Expression;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ZConst {
    pub name: String,
    pub comment: String,
    pub zserio_type: Option<Box<TypeReference>>,
    pub value_expression: Option<Rc<RefCell<Expression>>>,
}
