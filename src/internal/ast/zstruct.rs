use std::collections::HashMap;

use crate::internal::ast::{field::Field, parameter::Parameter, zfunction::ZFunction};

use crate::internal::compiler::symbol_scope::{ModelScope, PackageScope, Symbol};
use std::cell::RefCell;
use std::rc::Rc;

pub struct ZStruct {
    pub name: String,
    pub comment: String,
    pub template_parameters: Vec<String>,
    pub type_parameters: Vec<Rc<RefCell<Parameter>>>,
    pub fields: Vec<Field>,
    pub functions: Vec<ZFunction>,
}

impl ZStruct {
    pub fn evaluate(&self, scope: &mut ModelScope) {
        // Ignore packages that are templated. They cannot be evaluated. Only their templated
        // instances will be evaluated.
        if !self.template_parameters.is_empty() {
            return;
        }

        for param in &self.type_parameters {
            param.as_ref().borrow().zserio_type.evaluate(scope);
        }
        for field in &self.fields {
            field.evaluate(scope);
        }
    }
}

pub fn add_struct_to_scope(z_struct: &Rc<RefCell<ZStruct>>, package_scope: &mut PackageScope) {
    // Create a local scope, which contains all symbols within this structure
    let mut local_symbols = HashMap::new();
    for rc_param in z_struct.borrow().type_parameters.iter() {
        let param = rc_param.as_ref().borrow();
        local_symbols.insert(param.name.clone(), Symbol::Parameter(rc_param.clone()));
    }
    for (i, field) in z_struct.borrow().fields.iter().enumerate() {
        local_symbols.insert(field.name.clone(), Symbol::Field(z_struct.clone(), i));
    }
    package_scope
        .local_symbols
        .insert(z_struct.borrow().name.clone(), local_symbols);

    // Add the struct name itself to the package scope.
    package_scope.file_symbols.insert(
        z_struct.borrow().name.clone(),
        Symbol::Struct(z_struct.clone()),
    );
}
