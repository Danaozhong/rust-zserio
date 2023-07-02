use crate::internal::ast::expression::Expression;
use crate::internal::ast::field::Field;
use crate::internal::ast::parameter::Parameter;
use crate::internal::ast::zfunction::ZFunction;
use std::cell::RefCell;
use std::rc::Rc;
pub struct ZChoiceCase {
    pub conditions: Vec<Rc<RefCell<Expression>>>,
    pub field: Option<Box<Field>>,
}

pub struct ZChoice {
    pub name: String,
    pub template_parameters: Vec<String>,
    pub type_parameters: Vec<Parameter>,
    pub selector_expression: Option<Rc<RefCell<Expression>>>,
    pub cases: Vec<ZChoiceCase>,
    pub default_case: Option<ZChoiceCase>,
    pub functions: Vec<ZFunction>,
}
