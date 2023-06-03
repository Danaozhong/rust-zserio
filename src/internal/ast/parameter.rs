use crate::internal::ast::type_reference::TypeReference;

pub struct Parameter {
    pub name: String,
    pub zserio_type: Option<Box<TypeReference>>,
}
