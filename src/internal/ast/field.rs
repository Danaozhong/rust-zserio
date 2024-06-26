use crate::internal::ast::{expression::Expression, type_reference::TypeReference};
use crate::internal::compiler::symbol_scope::ModelScope;
use std::cell::RefCell;
use std::rc::Rc;
use std::string::String;

#[derive(Clone, Debug)]
/// This struct hold array-specific details of a field. This struct is only used
/// for fields that are of array type.
pub struct Array {
    /// Flag, specifying if the array is packed.
    pub is_packed: bool,

    /// Flag specifying if the array length is implicitly derived from the bit stream.
    /// If yes, the compiler will try to read array elements until the EOF is reached.
    /// Implicit arrays are not supported yet.
    pub is_implicit: bool,

    /// In case the zserio array length is set by an expression, this expression is specified
    /// here. In most cases, this will just be a numeric value (e.g. "3"), but zserio
    /// also supports more complex array length expressions.
    pub array_length_expression: Option<Rc<RefCell<Expression>>>,
}

#[derive(Clone, Debug)]
/// A field is a field in a zserio structure, union, or choice compound type.
pub struct Field {
    /// The field name.
    pub name: String,

    /// The comment placed above the field in the zserio code.
    pub comment: String,

    /// Flag, if the field can be optional.
    pub is_optional: bool,

    /// (Optional) bit alignment, if desired. If no bit alignment is set, 0 is used.
    pub alignment: u8,

    /// The type of the field.
    pub field_type: Box<TypeReference>,

    /// (Optional) An expression, providing a default value for the field.
    pub initializer: Option<Rc<RefCell<Expression>>>,

    /// (Optional) an expression
    pub offset: Option<Rc<RefCell<Expression>>>,

    /// (Optional) A constraint which restricts the possible values that can be assigned.
    pub constraint: Option<Rc<RefCell<Expression>>>,

    /// (Optional) A condition, which specifies if this field should be serialized or not.
    /// The field will only be written to the byte stream if this condition evaluates to true.
    pub optional_clause: Option<Rc<RefCell<Expression>>>,

    // (Optional) If this field is an array, this struct contains all properties of the array.
    pub array: Option<Array>,
}

impl Field {
    /// Evaluates all expressions within the field.
    pub fn evaluate(&mut self, scope: &mut ModelScope) {
        self.field_type.evaluate(scope);
        if let Some(array) = &self.array {
            if let Some(array_length_expression) = &array.array_length_expression {
                array_length_expression.borrow_mut().evaluate(scope);
            }
        }
        if let Some(optional_clause) = &self.optional_clause {
            optional_clause.borrow_mut().evaluate(scope);
        }
        if let Some(initializer) = &mut self.initializer {
            initializer.borrow_mut().evaluate(scope);
        }
    }
}
