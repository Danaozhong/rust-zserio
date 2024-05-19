use super::type_reference::TypeReference;
use crate::internal::ast::expression::{Expression, ExpressionType};
use crate::internal::compiler::symbol_scope::{ModelScope, PackageScope, Symbol};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
/// This class stores a possible value of a zserio bitmask type.
pub struct ZBitmaskValue {
    /// The name of the possible value.
    pub name: String,

    /// The integer value of the bitmask.
    pub value: u64,

    /// An expression, specifying which value this bitmask value has.
    /// This field is optional - if it is not set, zserio will automatically
    /// assign the next free bit in the bitmask for this value.
    pub value_expression: Option<Rc<RefCell<Expression>>>,
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
        let mut bitmask_value = 1u64;
        for value in &mut self.values {
            // Check if the bitmask value has a value hard-coded.
            // If yes, use the value from the expression.
            if let Some(value_expression_rc) = &mut value.value_expression {
                let mut value_expression = value_expression_rc.as_ref().borrow_mut();

                value_expression.evaluate(scope);
                let expr_bitmask_value = match value_expression.result_type {
                    ExpressionType::Integer(v) => v as u64,
                    _ => panic!("only integer bitmask values are supported"),
                };
                if expr_bitmask_value < bitmask_value {
                    panic!("expression redefines bitmask value");
                }
            }
            value.value = bitmask_value;

            // Pick the next bit for the next bitmask value.
            bitmask_value = bitmask_value << 1;
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
