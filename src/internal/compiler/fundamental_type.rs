use crate::internal::compiler::symbol_scope::{ModelScope, Symbol, SymbolReference};

pub fn _get_fundamental_type(symbol_ref: &SymbolReference, scope: &ModelScope) -> SymbolReference {
    let mut current_symbol_ref = symbol_ref.clone();

    loop {
        match current_symbol_ref.symbol {
            Symbol::Bitmask(_)
            | Symbol::Choice(_)
            | Symbol::Enum(_)
            | Symbol::Const(_)
            | Symbol::Struct(_) => {
                return current_symbol_ref;
            }
            Symbol::Subtype(subtype) => {
                current_symbol_ref = scope.resolve_symbol(&subtype.borrow().zserio_type.name);
            }
            _ => panic!("unexpected type {:?}", symbol_ref.symbol),
        }
    }
}
