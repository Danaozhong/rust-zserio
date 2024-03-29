use super::type_reference::TypeReference;
use crate::internal::ast::expression::Expression;
use crate::internal::compiler::symbol_scope::{ModelScope, PackageScope, Symbol};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
/// This class stores a possible value of a zserio bitmask type.
pub struct ZBitmaskValue {
    /// The name of the possible value.
    pub name: String,

    /// An expression, specifying which value this bitmask value has.
    /// This field is optional - if it is not set, zserio will automatically
    /// assign the next free bit in the bitmask for this value.
    pub value: Option<Rc<RefCell<Expression>>>,
}

#[derive(Debug)]

/// A zserio bitmask type.
pub struct ZBitmaskType {
    /// The name of the bitmask type.
    pub name: String,

    /// The zserio integer type used to store this bitmask.
    pub zserio_type: Box<TypeReference>,

    /// A list of possible bitmask values supported by this type.
    pub values: Vec<ZBitmaskValue>,
}

impl ZBitmaskType {
    /// Evaluates all expressions within this bitmask type.
    pub fn evaluate(&mut self, scope: &mut ModelScope) {
        for value in &mut self.values {
            if let Some(value_expression) = &mut value.value {
                value_expression.as_ref().borrow_mut().evaluate(scope);
            }
        }
        self.zserio_type.evaluate(scope);
    }
}

/// Adds the bitmask and all possible values to the evaluation scope.
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
