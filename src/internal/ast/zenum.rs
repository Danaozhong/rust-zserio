use std::string::String;

use super::expression::Expression;
use super::type_reference::TypeReference;

pub struct ZEnumItem {
    pub name: String,
    pub comment: String,
    pub expression: Option<Box<Expression>>,
}

pub struct ZEnum {
    pub name: String,
    pub comment: String,
    pub items: Vec<ZEnumItem>,
    pub enum_type: Box<TypeReference>,
}

impl ZEnum {
    pub fn evaluate(&mut self, scope: &ModelScope) {
        for item in &mut self.items {
            if let Some(item_expression) = item.expression {
                item_expression.evaluate();
            }
        }
    }
}
