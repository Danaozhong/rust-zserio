use crate::internal::ast::type_reference::TypeReference;

#[derive(Clone)]
pub struct Parameter {
    pub name: String,
    pub zserio_type: Box<TypeReference>,
}
