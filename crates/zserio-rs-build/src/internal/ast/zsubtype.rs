use crate::internal::ast::type_reference::TypeReference;
use crate::internal::compiler::symbol_scope::{PackageScope, Symbol};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Subtype {
    pub name: String,
    pub zserio_type: Box<TypeReference>,
}

pub fn add_subtype_to_scope(subtype: &Rc<RefCell<Subtype>>, package_scope: &mut PackageScope) {
    package_scope.file_symbols.insert(
        subtype.borrow().name.clone(),
        Symbol::Subtype(subtype.clone()),
    );
}
