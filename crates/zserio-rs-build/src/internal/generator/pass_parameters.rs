use crate::internal::ast::{
    expression::Expression, parameter::Parameter, type_reference::TypeReference,
};

use crate::internal::compiler::symbol_scope::{ModelScope, Symbol};
use crate::internal::parser::gen::zserioparser::INDEX;
use std::cell::RefCell;
use std::rc::Rc;

pub fn get_type_parameter(
    scope: &ModelScope,
    type_ref: &TypeReference,
) -> Vec<Rc<RefCell<Parameter>>> {
    let symbol = scope.get_symbol(type_ref);
    return match symbol.symbol {
        Symbol::Choice(zchoice) => zchoice.borrow().type_parameters.clone(),
        Symbol::Struct(zstruct) => zstruct.borrow().type_parameters.clone(),
        Symbol::Union(zunion) => zunion.borrow().type_parameters.clone(),
        _ => panic!("type cannot be parameterized"),
    };
}

/// Return the number of fields in the rust struct that is generated for this type.
pub fn number_of_fields(scope: &ModelScope, type_ref: &TypeReference) -> usize {
    let symbol = scope.get_symbol(type_ref);
    return match symbol.symbol {
        Symbol::Choice(zchoice) => {
            let zchoice = zchoice.borrow();
            zchoice.cases.len() + zchoice.type_parameters.len()
        }
        Symbol::Struct(zstruct) => {
            let zstruct = zstruct.borrow();
            zstruct.fields.len() + zstruct.type_parameters.len()
        }
        Symbol::Union(zunion) => {
            let zunion = zunion.borrow();
            zunion.fields.len() + zunion.type_parameters.len()
        }
        _ => 0,
    };
}

/// This function checks if an expression (or any sub-expression) contain an @index operator.
pub fn does_expression_contains_index_operator(expression: &Expression) -> bool {
    let mut expressions_to_check = vec![expression];

    while let Some(current_exp) = expressions_to_check.pop() {
        if current_exp.expression_type == INDEX {
            return true;
        }
        if let Some(op1) = &current_exp.operand1 {
            expressions_to_check.push(op1);
        }
        if let Some(op2) = &current_exp.operand2 {
            expressions_to_check.push(op2);
        }
        if let Some(op3) = &current_exp.operand3 {
            expressions_to_check.push(op3);
        }
    }
    // if none of the checked expressions contain the index operator, return false.
    false
}
