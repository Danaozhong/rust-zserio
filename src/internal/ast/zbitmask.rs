use crate::internal::ast::expression::Expression;

use super::type_reference::TypeReference;

pub struct ZBitmaskValue {
    pub name: String,
    pub value: Option<Box<Expression>>,
}

pub struct ZBitmaskType {
    pub name: String,
    pub zserio_type: Option<Box<TypeReference>>,
    pub values: Vec<ZBitmaskValue>,
}

impl ZBitmaskType {
    fn evaluate(&mut self) {
        // TODO
    }
}
