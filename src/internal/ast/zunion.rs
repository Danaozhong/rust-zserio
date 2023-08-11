use crate::internal::ast::{field::Field, parameter::Parameter, zfunction::ZFunction};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct ZUnion {
    pub name: String,
    pub comment: String,
    pub template_parameters: Vec<String>,
    pub type_parameters: Vec<Rc<RefCell<Parameter>>>,
    pub fields: Vec<Field>,
    pub functions: Vec<ZFunction>,
}
