use crate::internal::ast::type_reference::TypeReference;

pub struct FundamentalZserioTypeReference {
    pub fundamental_type: Box<TypeReference>,
    pub requires_cast: bool,
    pub is_marshaler: bool,
}

pub fn get_fundamental_type(type_ref: &TypeReference) -> Box<FundamentalZserioTypeReference> {
    loop {
        if type_ref.is_builtin {
            return Box::new(FundamentalZserioTypeReference {
                fundamental_type: Box::new(type_ref.clone()),
                requires_cast: false,
                is_marshaler: false,
            });
        }
        break;
    }

    return Box::new(FundamentalZserioTypeReference {
        fundamental_type: Box::new(type_ref.clone()),
        requires_cast: false,
        is_marshaler: true,
    });
}
