use crate::internal::ast::type_reference::TypeReference;
use crate::internal::ast::zchoice::ZChoice;
use crate::internal::ast::zstruct::ZStruct;
use crate::internal::ast::zsubtype::Subtype;
use crate::internal::ast::zunion::ZUnion;
use crate::internal::compiler::symbol_scope::ModelScope;

/// Resolves all type references within a struct, and assigns the full package path
/// to the type reference.
pub fn resolve_struct_types(z_struct: &mut ZStruct, scope: &mut ModelScope) {
    if !z_struct.template_parameters.is_empty() {
        // Ignore templates
        return;
    }
    for field in &z_struct.fields {
        resolve_type_reference(field.borrow_mut().field_type.as_mut(), scope);
    }
    for type_param in &z_struct.type_parameters {
        resolve_type_reference(type_param.borrow_mut().zserio_type.as_mut(), scope);
    }
    for function in &z_struct.functions {
        resolve_type_reference(&mut function.borrow_mut().return_type, scope);
    }
}

/// Resolves all type references within a struct, and assigns the full package path
/// to the type reference.
pub fn resolve_choice_types(z_choice: &mut ZChoice, scope: &mut ModelScope) {
    if !z_choice.template_parameters.is_empty() {
        // Ignore templates
        return;
    }
    for case in &z_choice.cases {
        if let Some(field) = &case.field {
            if field.borrow().name == "IsoDetails" {
                print!("test");
            }
            resolve_type_reference(field.borrow_mut().field_type.as_mut(), scope);
        }
    }
    if let Some(default_case) = &z_choice.default_case {
        if let Some(field) = &default_case.field {
            resolve_type_reference(field.borrow_mut().field_type.as_mut(), scope);
        }
    }
    for type_param in &z_choice.type_parameters {
        resolve_type_reference(type_param.borrow_mut().zserio_type.as_mut(), scope);
    }
    for function in &z_choice.functions {
        resolve_type_reference(&mut function.borrow_mut().return_type, scope);
    }
}

/// Resolves all type references within an union, and assigns the full package path
/// to the type reference.
pub fn resolve_union_types(z_union: &mut ZUnion, scope: &mut ModelScope) {
    if !z_union.template_parameters.is_empty() {
        // Ignore templates
        return;
    }
    for field in &z_union.fields {
        resolve_type_reference(field.borrow_mut().field_type.as_mut(), scope);
    }
    for type_param in &z_union.type_parameters {
        resolve_type_reference(type_param.borrow_mut().zserio_type.as_mut(), scope);
    }
    for function in &z_union.functions {
        resolve_type_reference(&mut function.borrow_mut().return_type, scope);
    }
}

/// Resolves all type references within a subtype, and assigns the full package path
/// to the type reference.
pub fn resolve_subtype(z_subtype: &mut Subtype, scope: &mut ModelScope) {
    resolve_type_reference(&mut z_subtype.zserio_type, scope);
}

pub fn resolve_type_reference(type_reference: &mut TypeReference, scope: &mut ModelScope) {
    if type_reference.is_builtin {
        return;
    }
    let symbol_ref = scope.resolve_symbol(&type_reference.name, true);
    if type_reference.package.is_empty() {
        type_reference.package = symbol_ref.package.clone();
    }
}
