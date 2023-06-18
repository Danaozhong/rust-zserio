use super::expression::Expression;
use super::type_reference::TypeReference;
use crate::internal::compiler::symbol_scope::ModelScope;
use crate::internal::compiler::symbol_scope::{PackageScope, Symbol};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::string::String;
pub struct ZEnumItem {
    pub name: String,
    pub comment: String,
    pub expression: Option<Rc<RefCell<Expression>>>,
}

pub struct ZEnum {
    pub name: String,
    pub comment: String,
    pub items: Vec<ZEnumItem>,
    pub enum_type: Box<TypeReference>,
}

impl ZEnum {
    pub fn evaluate(&mut self, scope: &mut ModelScope) {
        for item in &mut self.items {
            if let Some(item_expression) = &mut item.expression {
                item_expression.as_ref().borrow_mut().evaluate(scope);
            }
        }
        self.enum_type.evaluate(scope);
    }
}

pub fn add_enum_to_scope(z_enum: &Rc<RefCell<ZEnum>>, package_scope: &mut PackageScope) {
    // Create a local scope, which contains all symbols within this structure
    let mut local_symbols = HashMap::new();
    for (i, enum_item) in z_enum.borrow().items.iter().enumerate() {
        local_symbols.insert(enum_item.name.clone(), Symbol::EnumItem(z_enum.clone(), i));
    }
    package_scope
        .local_symbols
        .insert(z_enum.borrow().name.clone(), local_symbols);

    // Add the struct name itself to the package scope.
    package_scope
        .file_symbols
        .insert(z_enum.borrow().name.clone(), Symbol::Enum(z_enum.clone()));
}
