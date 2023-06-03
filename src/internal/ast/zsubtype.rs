use crate::internal::ast::type_reference::TypeReference;

pub struct Subtype {
    pub name: String,
    pub zserio_type: Option<Box<TypeReference>>,
}
