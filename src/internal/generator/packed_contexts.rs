use crate::internal::ast::field::Field;

use crate::internal::ast::type_reference::TypeReference;
use crate::internal::compiler::fundamental_type::{
    get_fundamental_type, FundamentalZserioTypeReference,
};
use crate::internal::compiler::symbol_scope::ModelScope;
use crate::internal::generator::array::array_type_name;
use crate::internal::generator::encode::requires_borrowing;
use crate::internal::generator::{expression::generate_boolean_expression, types::TypeGenerator};
use codegen::Function;
use std::cell::RefCell;
use std::rc::Rc;

use super::array::initialize_array_trait;

pub struct FieldDetails {
    pub field: Rc<RefCell<Field>>,
    pub field_index: usize,
    pub field_name: String,
    pub native_type: FundamentalZserioTypeReference,
    pub rust_type: String,
    pub rust_array_type_name: String,
    pub field_context_node_name: String,
    pub is_packable: bool,
}

impl FieldDetails {
    pub fn from_field(
        field_rc: &Rc<RefCell<Field>>,
        field_index: usize,
        symbol_scope: &ModelScope,
        type_generator: &mut TypeGenerator,
    ) -> Self {
        let field = &field_rc.borrow();
        let field_name = type_generator.convert_field_name(&field.name);
        let field_context_node_name = format!("field_{}_node", &field_name);

        // confirm if the field can be packed. even in packed mode, some fields
        // may not use packing, for example if they are offset fields.
        // arrays can always be packed.
        let is_packable = is_delta_packable(
            symbol_scope,
            field.field_type.as_ref(),
            field.is_offset_field,
            field.array.is_some(),
        );
        FieldDetails {
            field: field_rc.clone(),
            field_index,
            field_name,
            native_type: *get_fundamental_type(&field.field_type, symbol_scope),
            rust_type: type_generator.ztype_to_rust_type(field.field_type.as_ref()),
            rust_array_type_name: array_type_name(&field.name),
            field_context_node_name,
            is_packable,
        }
    }
}

/// is_delta_packable identifies if a type is delta-packable or not.
pub fn is_delta_packable(
    model_scope: &ModelScope,
    zserio_type: &TypeReference,
    is_offset_field: bool,
    is_array: bool,
) -> bool {
    if is_offset_field {
        return false;
    }
    if is_array {
        // arrays can always be packed, unless overridden by being used as offsets.
        return true;
    }
    let fundamental_type = get_fundamental_type(zserio_type, model_scope);
    if fundamental_type.fundamental_type.is_builtin {
        return matches!(
            fundamental_type.fundamental_type.name.as_str(),
            "int8"
                | "int16"
                | "int32"
                | "int64"
                | "uint8"
                | "uint16"
                | "uint32"
                | "uint64"
                | "int"
                | "uint"
                | "bit"
                | "varint"
                | "varint16"
                | "varint32"
                | "varint64"
                | "varuint"
                | "varsize"
                | "varuint16"
                | "varuint32"
                | "varuint64"
        );
    }
    if fundamental_type.is_marshaler {
        return true;
    }
    false
}

pub fn generate_packed_context_for_field(fn_gen: &mut Function, field_details: &FieldDetails) {
    fn_gen.line(format!(
        "let mut {} = PackingContextNode::new();",
        &field_details.field_context_node_name
    ));
    // For array types, the packing context is created on-the-fly during array writing/reading.
    if field_details.field.borrow().array.is_none() {
        if field_details.native_type.is_marshaler {
            fn_gen.line(format!(
                "{}::zserio_create_packing_context(&mut {});",
                &field_details.rust_type, &field_details.field_context_node_name
            ));
        } else if field_details.is_packable {
            // The field can be delta-packed. Generate a delta context.
            fn_gen.line(format!(
                "{}.context = Some(DeltaContext::new());",
                field_details.field_context_node_name
            ));
        }
    }
    fn_gen.line(format!(
        "context_node.children.push({});",
        &field_details.field_context_node_name
    ));
}

pub fn generate_init_packed_context_for_field(
    model_scope: &ModelScope,
    type_generator: &mut TypeGenerator,
    fn_gen: &mut Function,
    field_details: &FieldDetails,
) {
    if field_details.field.borrow().array.is_some() {
        return;
    }

    if let Some(optional_clause) = &field_details.field.borrow().optional_clause {
        fn_gen.line(format!(
            "if {} {{",
            generate_boolean_expression(&optional_clause.borrow(), type_generator, model_scope)
        ));
    }

    let mut field_name = format!("self.{}", field_details.field_name.clone());

    if field_details.field.borrow().is_optional {
        // If the type is a marshaller, take it by reference.
        let mut borrow_symbol = String::from("");
        if requires_borrowing(&field_details.field.borrow(), &field_details.native_type) {
            borrow_symbol = "&".into();
        }

        fn_gen.line(format!(
            "if let Some(x) = {}{} {{",
            borrow_symbol, field_name
        ));
        field_name = "x".into();
    }

    if field_details.native_type.is_marshaler {
        fn_gen.line(format!(
            "{}.zserio_init_packing_context(&mut context_node.children[{}]);",
            &field_name, field_details.field_index
        ));
    } else if field_details.is_packable {
        // Initialize the delta context with the array traits
        fn_gen.line(format!(
            "let mut {}_delta_context = context_node.children[{}].context.as_mut().unwrap();",
            field_details.field_context_node_name, field_details.field_index
        ));
        fn_gen.line(format!(
            "{}_delta_context.init(&{}, &{});",
            &field_details.field_context_node_name,
            initialize_array_trait(
                model_scope,
                type_generator,
                &field_details.native_type.fundamental_type
            ),
            &field_name
        ));
    }
    if field_details.field.borrow().is_optional {
        fn_gen.line("}");
    }
    if field_details.field.borrow().optional_clause.is_some() {
        fn_gen.line("}");
    }
}
