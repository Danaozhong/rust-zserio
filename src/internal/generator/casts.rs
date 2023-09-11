use crate::internal::ast::expression::{Expression, ExpressionType};
use crate::internal::ast::type_reference::TypeReference;
use crate::internal::generator::types::TypeGenerator;

pub fn requires_cast(
    target_type: &TypeReference,
    other_type: &TypeReference,
    type_generator: &TypeGenerator,
) -> bool {
    let other_rust_type = type_generator.ztype_to_rust_type(other_type);
    let target_rust_type = type_generator.ztype_to_rust_type(target_type);
    other_rust_type != target_rust_type
}

pub fn expression_requires_cast(
    target_type: &TypeReference,
    type_generator: &TypeGenerator,
    expression: &Expression,
) -> bool {
    match &expression.result_type {
        ExpressionType::Integer(_) | ExpressionType::BitMask(_) | ExpressionType::Float(_) => {
            match &expression.native_type {
                Some(native_type) => requires_cast(target_type, native_type, type_generator),
                None => false,
            }
        }
        _ => false,
    }
}
