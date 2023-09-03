use crate::internal::ast::type_reference::TypeReference;
use crate::internal::compiler::symbol_scope::{ModelScope, Symbol, SymbolReference};

pub struct FundamentalZserioTypeReference {
    pub fundamental_type: Box<TypeReference>,
    pub requires_cast: bool,
    pub is_marshaler: bool,
}

pub fn get_fundamental_type(
    type_ref: &TypeReference,
    scope: &ModelScope,
) -> Box<FundamentalZserioTypeReference> {
    let mut requires_cast = false;
    let mut is_marshaler = false;

    let mut current_type_ref = type_ref.clone();

    loop {
        if current_type_ref.is_builtin {
            return Box::new(FundamentalZserioTypeReference {
                fundamental_type: Box::new(current_type_ref),
                requires_cast,
                is_marshaler,
            });
        }
        // We call get_symbol() instead of resolve_symbol(), because we
        // expect the type reference to already be fully resolved.
        let current_symbol_ref = scope.get_symbol(&current_type_ref);
        /*
        scope.scope_stack.push(
            ScopeLocation {
                package: current_symbol_ref.package.clone(),
                import_symbol: None,
                symbol_name: current_symbol_ref,
            }
        ); */
        let mut new_type_ref = match current_symbol_ref.symbol {
            Symbol::Bitmask(_)
            | Symbol::Choice(_)
            | Symbol::Enum(_)
            | Symbol::Const(_)
            | Symbol::Union(_)
            | Symbol::Struct(_) => {
                is_marshaler = true;
                get_symbol_type(&current_symbol_ref)
            }
            Symbol::Subtype(subtype) => *subtype.borrow().zserio_type.clone(),
            _ => panic!("unexpected type {:?}", current_symbol_ref.symbol),
        };

        // Make sure that the type arguments are passed to the
        // fundamental type.
        new_type_ref.type_arguments = current_type_ref.type_arguments.clone();
        current_type_ref = new_type_ref;

        if is_marshaler {
            // If the type is a marshaller type, return the type reference
            return Box::new(FundamentalZserioTypeReference {
                fundamental_type: Box::from(current_type_ref),
                requires_cast,
                is_marshaler,
            });
        }

        requires_cast = true;
    }
}

pub fn get_symbol_type(symbol_ref: &SymbolReference) -> TypeReference {
    let type_ref_name = match &symbol_ref.symbol {
        Symbol::Bitmask(bitmask) => bitmask.borrow().name.clone(),
        Symbol::Choice(zchoice) => zchoice.borrow().name.clone(),
        Symbol::Enum(zenum) => zenum.borrow().name.clone(),
        Symbol::Const(zconst) => zconst.borrow().name.clone(),
        Symbol::Struct(zstruct) => zstruct.borrow().name.clone(),
        Symbol::Subtype(subtype) => subtype.borrow().name.clone(),
        Symbol::Union(zunion) => zunion.borrow().name.clone(),
        _ => panic!("unexpected type {:?}", symbol_ref.symbol),
    };

    TypeReference {
        is_builtin: false,
        is_const: false,
        package: symbol_ref.package.clone(),
        name: type_ref_name,
        bits: 0,
        template_arguments: vec![],
        type_arguments: vec![],
        length_expression: None,
    }
}
