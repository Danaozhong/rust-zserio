use std::collections::HashMap;

use crate::internal::ast::{field::Field, parameter::Parameter, zfunction::ZFunction};

use crate::internal::compiler::symbol_scope::{ModelScope, PackageScope, Symbol};
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug)]
pub struct ZUnion {
    pub name: String,
    pub comment: String,
    pub template_parameters: Vec<String>,
    pub type_parameters: Vec<Rc<RefCell<Parameter>>>,
    pub fields: Vec<Rc<RefCell<Field>>>,
    pub functions: Vec<Rc<RefCell<ZFunction>>>,
}

impl ZUnion {
    pub fn evaluate(&self, scope: &mut ModelScope) {
        // Ignore unions that are templated. They cannot be evaluated. Only their templated
        // instances will be evaluated.
        if !self.template_parameters.is_empty() {
            return;
        }

        for param in &self.type_parameters {
            param.as_ref().borrow().zserio_type.evaluate(scope);
        }
        for field in &self.fields {
            field.as_ref().borrow_mut().evaluate(scope);
        }
        for function in &self.functions {
            function.as_ref().borrow_mut().evaluate(scope);
        }
    }
}

pub fn add_zunion_to_scope(z_union: &Rc<RefCell<ZUnion>>, package_scope: &mut PackageScope) {
    // Create a local scope, which contains all symbols within this union type
    let mut local_symbols = HashMap::new();
    for rc_param in z_union.borrow().type_parameters.iter() {
        let param = rc_param.as_ref().borrow();
        local_symbols.insert(param.name.clone(), Symbol::Parameter(rc_param.clone()));
    }

    // add the fields to the local scope
    for field in &z_union.borrow().fields {
        local_symbols.insert(field.borrow().name.clone(), Symbol::Field(field.clone()));
    }

    // add the functions to the local scope
    for function in &z_union.borrow().functions {
        local_symbols.insert(
            function.as_ref().borrow().name.clone(),
            Symbol::Function(function.clone()),
        );
    }

    package_scope
        .local_symbols
        .insert(z_union.borrow().name.clone(), local_symbols);

    // Add the union name itself to the package scope.
    package_scope.file_symbols.insert(
        z_union.borrow().name.clone(),
        Symbol::Union(z_union.clone()),
    );
}
