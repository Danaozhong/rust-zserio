use crate::internal::ast::field::Field;
use crate::internal::ast::package::ZPackage;
use crate::internal::ast::parameter::Parameter;
use crate::internal::ast::type_reference::TypeReference;
use crate::internal::ast::zchoice::{add_choice_to_scope, ZChoice, ZChoiceCase};
use crate::internal::ast::zfunction::ZFunction;
use crate::internal::ast::zstruct::{add_struct_to_scope, ZStruct};
use crate::internal::compiler::symbol_scope::{ModelScope, Symbol};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn instantiate_type(
    pkg: &mut ZPackage,
    scope: &mut ModelScope,
    zserio_type: &TypeReference,
    name: &str,
) -> TypeReference {
    // A template instantiation is only possible if the type accepts template arguments.
    // For types that are no templates, return the type itself.
    if zserio_type.template_arguments.len() == 0 {
        return zserio_type.clone();
    }

    let mut new_type_name = String::from(name);
    if new_type_name == "" {
        // in case no name is provided, a new name needs to be generated.
        new_type_name = generate_instantiated_name(zserio_type);
    }

    let symbol = scope.resolve_symbol(&zserio_type.name);
    match symbol.symbol {
        Symbol::Struct(s) => {
            return instantiate_struct(
                pkg,
                scope,
                &s.as_ref().borrow(),
                &zserio_type.template_arguments,
                &new_type_name,
            );
        }
        Symbol::Choice(c) => {
            return instantiate_choice(
                pkg,
                scope,
                &c.as_ref().borrow(),
                &zserio_type.template_arguments,
                &new_type_name,
            );
        }
        _ => panic!("unsupported type for template instantiation"),
    }
}

fn generate_instantiated_name(type_reference: &TypeReference) -> String {
    let mut new_name = type_reference.name.clone();
    for template_arg in &type_reference.template_arguments {
        new_name += template_arg.name.as_str();
    }
    new_name
}

fn instantiate_struct(
    pkg: &mut ZPackage,
    scope: &mut ModelScope,
    z_struct: &ZStruct,
    template_arguments: &Vec<TypeReference>,
    instantiated_name: &String,
) -> TypeReference {
    assert!(z_struct.template_parameters.len() > 0);
    assert!(z_struct.template_parameters.len() == template_arguments.len());

    let new_type_ref = TypeReference {
        is_builtin: false,
        package: pkg.name.clone(),
        name: instantiated_name.clone(),
        bits: 0,
        template_arguments: vec![],
        type_arguments: vec![],
        length_expression: None,
    };

    if pkg.structs.contains_key(instantiated_name) {
        // The structure was already instantiated, there is no need to instantiate it again.
        // Return a new reference to the already existing type.
        return new_type_ref;
    }

    // The instantiated struct doesn't exist yet. Start by instantiating the
    // template parameter itself.
    let mut instantiated_types: HashMap<String, TypeReference> = HashMap::new();
    for (index, template_arg) in template_arguments.iter().enumerate() {
        // In case the type instantiation is a template itself, it must be instantiated
        // before mapping it. In case the template argument itself is not a template,
        // instantiate_type() will just pass the type through.
        let instantiated_type = instantiate_type(pkg, scope, template_arg, "");
        instantiated_types.insert(
            z_struct.template_parameters[index].clone(),
            instantiated_type,
        );
    }

    let mut instantiated_struct = ZStruct {
        name: instantiated_name.clone(),
        comment: z_struct.comment.clone(),
        template_parameters: vec![],
        type_parameters: vec![],
        fields: vec![],
        functions: vec![],
    };

    // Instantiate the fields
    for field in &z_struct.fields {
        instantiated_struct
            .fields
            .push(Rc::from(RefCell::from(instantiate_field(
                pkg,
                scope,
                &field.borrow(),
                &instantiated_types,
            ))));
    }
    // Instantiate the parameters
    for rc_param in &z_struct.type_parameters {
        // Check if the parameter is a templated type
        let param = rc_param.as_ref().borrow();
        if instantiated_types.contains_key(&param.zserio_type.name) {
            // The parameter is a templated type, so replace the parameter
            // type by a reference to the newly instantiated type.
            instantiated_struct
                .type_parameters
                .push(Rc::new(RefCell::new(Parameter {
                    name: param.name.clone(),
                    zserio_type: Box::new(instantiated_types[&param.zserio_type.name].clone()),
                })));
        } else {
            // The parameter is not templated, and is not affected by template instantiation.
            instantiated_struct
                .type_parameters
                .push(Rc::new(RefCell::new(param.clone())));
        }
    }

    for rc_function in &z_struct.functions {
        let function = rc_function.as_ref().borrow();
        if instantiated_types.contains_key(&function.return_type.name) {
            // The parameter is a templated type, so replace the parameter
            // type by a reference to the newly instantiated type.
            instantiated_struct
                .functions
                .push(Rc::new(RefCell::new(ZFunction {
                    name: function.name.clone(),
                    result: function.result.clone(),
                    return_type: Box::new(instantiated_types[&function.return_type.name].clone()),
                })));
        } else {
            instantiated_struct
                .functions
                .push(Rc::new(RefCell::new(function.clone())));
        }
    }

    // Add the newly added structure to the package.
    pkg.structs.insert(
        instantiated_name.clone(),
        Rc::from(RefCell::from(instantiated_struct)),
    );

    // Update the scope to contain the newly added structure.
    add_struct_to_scope(&pkg.structs[instantiated_name], scope.get_package_scope());
    new_type_ref
}

fn instantiate_choice(
    pkg: &mut ZPackage,
    scope: &mut ModelScope,
    z_choice: &ZChoice,
    template_arguments: &Vec<TypeReference>,
    instantiated_name: &String,
) -> TypeReference {
    assert!(z_choice.template_parameters.len() > 0);
    assert!(z_choice.template_parameters.len() == template_arguments.len());

    let new_type_ref = TypeReference {
        is_builtin: false,
        package: pkg.name.clone(),
        name: instantiated_name.clone(),
        bits: 0,
        template_arguments: vec![],
        type_arguments: vec![],
        length_expression: None,
    };

    if pkg.zchoices.contains_key(instantiated_name) {
        // The structure was already instantiated, there is no need to instantiate it again.
        // Return a new reference to the already existing type.
        return new_type_ref;
    }

    // The instantiated struct doesn't exist yet. Start by instantiating the
    // template parameter itself.
    let mut instantiated_types: HashMap<String, TypeReference> = HashMap::new();
    for (index, template_arg) in template_arguments.iter().enumerate() {
        // In case the type instantiation is a template itself, it must be instantiated
        // before mapping it. In case the template argument itself is not a template,
        // instantiate_type() will just pass the type through.
        let instantiated_type = instantiate_type(pkg, scope, template_arg, "");
        instantiated_types.insert(
            z_choice.template_parameters[index].clone(),
            instantiated_type,
        );
    }

    let mut instantiated_choice = ZChoice {
        name: instantiated_name.clone(),
        template_parameters: vec![],
        type_parameters: vec![],
        selector_expression: z_choice.selector_expression.clone(),
        cases: vec![],
        default_case: None,
        functions: vec![],
    };

    // Instantiate the choice fields
    for case in &z_choice.cases {
        instantiated_choice.cases.push(ZChoiceCase {
            conditions: case.conditions.clone(),
            field: {
                if let Some(field) = &case.field {
                    Option::from(Rc::from(RefCell::from(instantiate_field(
                        pkg,
                        scope,
                        &field.borrow(),
                        &instantiated_types,
                    ))))
                } else {
                    None
                }
            },
        });
    }
    // Instantiate the default field, if present
    if let Some(default_case) = &z_choice.default_case {
        instantiated_choice.default_case = Option::from(ZChoiceCase {
            conditions: default_case.conditions.clone(),
            field: {
                if let Some(field) = &default_case.field {
                    Option::from(Rc::from(RefCell::from(instantiate_field(
                        pkg,
                        scope,
                        &field.borrow(),
                        &instantiated_types,
                    ))))
                } else {
                    None
                }
            },
        });
    }

    // Instantiate the parameters
    for rc_param in &z_choice.type_parameters {
        // Check if the parameter is a templated type
        let param = rc_param.as_ref().borrow();
        if instantiated_types.contains_key(&param.name) {
            // The parameter is a templated type, so replace the parameter
            // type by a reference to the newly instantiated type.
            instantiated_choice
                .type_parameters
                .push(Rc::new(RefCell::new(Parameter {
                    name: param.name.clone(),
                    zserio_type: Box::new(instantiated_types[&param.name].clone()),
                })));
        } else {
            // The parameter is not templated, and is not affected by template instantiation.
            instantiated_choice
                .type_parameters
                .push(Rc::new(RefCell::new(param.clone())));
        }
    }

    for rc_function in &z_choice.functions {
        let function = rc_function.as_ref().borrow();
        if instantiated_types.contains_key(&function.return_type.name) {
            // The parameter is a templated type, so replace the parameter
            // type by a reference to the newly instantiated type.
            instantiated_choice
                .functions
                .push(Rc::new(RefCell::new(ZFunction {
                    name: function.name.clone(),
                    result: function.result.clone(),
                    return_type: Box::new(instantiated_types[&function.return_type.name].clone()),
                })));
        } else {
            instantiated_choice
                .functions
                .push(Rc::new(RefCell::new(function.clone())));
        }
    }
    // Add the newly added choice to the package.
    pkg.zchoices.insert(
        instantiated_name.clone(),
        Rc::from(RefCell::from(instantiated_choice)),
    );

    // Update the scope to contain the newly added structure.
    add_choice_to_scope(&pkg.zchoices[instantiated_name], scope.get_package_scope());
    new_type_ref
}

pub fn instantiate_field(
    pkg: &mut ZPackage,
    scope: &mut ModelScope,
    field: &Field,
    instantiated_types: &HashMap<String, TypeReference>,
) -> Field {
    let mut new_field = field.clone();
    if new_field.field_type.template_arguments.len() > 0 {
        // The field type is not a template type itself, but by itself is a template,
        // which is using the template type in its template parameters.
        // For example:
        // SomeOtherTemplate<TEMPLATE_TYPE> field;

        // Iterate over the teamplate parameters and instantiate them.
        let mut new_template_arguments = vec![];

        for (index, template_parameter) in
            new_field.field_type.template_arguments.iter().enumerate()
        {
            if instantiated_types.contains_key(&template_parameter.name) {
                new_template_arguments.push(instantiated_types[&template_parameter.name].clone());
            } else {
                new_template_arguments.push(template_parameter.clone());
            }
        }
        new_field.field_type.template_arguments = new_template_arguments;

        new_field.field_type = Box::from(instantiate_type(pkg, scope, &*new_field.field_type, ""));
        return new_field;
    }

    // Check if the field type is directly a templated type. For example:
    // TEMPLATE_TYPE field;
    if instantiated_types.contains_key(&field.field_type.name) {
        new_field.field_type = Box::from(instantiated_types[&field.field_type.name].clone());
    }
    return new_field;
}