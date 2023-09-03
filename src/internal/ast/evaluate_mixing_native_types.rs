use crate::internal::ast::type_reference::TypeReference;

fn compare_native_types(type1: &TypeReference, type2: &TypeReference) -> bool {
    type1.name == type2.name && type1.bits == type2.bits
}

pub fn evaluate_mixing_float_types(
    type1: &Option<TypeReference>,
    type2: &Option<TypeReference>,
) -> Option<TypeReference> {
    match (type1, type2) {
        (Some(t1), Some(t2)) => {
            // If both types are set, check if they are the same type.
            if compare_native_types(t1, t2) {
                return type1.clone();
            }
            // Choose the float type with the highest rank.
            for float_type in ["float64", "float32", "float16"] {
                if t1.name == float_type || t2.name == float_type {
                    return Some(TypeReference::new_native_type(float_type, false));
                }
            }
            panic!("unknown float types");
        }
        (Some(_), None) => type1.clone(),
        (None, Some(_)) => type2.clone(),
        (None, None) => None,
    }
}

fn get_integer_rank(integer_type: &TypeReference) -> u8 {
    // if the bit length is dynamic, always assume 64bits
    if integer_type.length_expression.is_some() {
        return 64;
    }
    if integer_type.bits != 0 {
        return integer_type.bits;
    }
    return match integer_type.name.as_str() {
        "int8" => 8,
        "int16" => 16,
        "int32" => 32,
        "int64" => 64,
        "varint16" => 15,
        "varint32" => 29,
        "varint64" => 57,
        "varint" => 64,
        "uint8" => 8,
        "uint16" => 16,
        "uint32" => 32,
        "varuint16" => 15,
        "varuint32" => 29,
        "varuint64" => 57,
        "varsize" => 31,
        "varuint" => 64,
        _ => panic!("unexpected integer type {:?}", integer_type),
    };
}

pub fn is_signed(int_type_name: &str) -> bool {
    match int_type_name {
        "int8" => true,
        "int16" => true,
        "int32" => true,
        "int64" => true,
        "varint16" => true,
        "varint32" => true,
        "varint64" => true,
        "varint" => true,
        "uint8" => false,
        "uint16" => false,
        "uint32" => false,
        "varuint16" => false,
        "varuint32" => false,
        "varuint64" => false,
        "varsize" => false,
        "varuint" => false,
        _ => panic!("unexpected integer type {:?}", int_type_name),
    }
}

pub fn convert_to_unsigned(int_type_name: &str) -> String {
    match int_type_name {
        "int8" => "uint8".into(),
        "int16" => "uint16".into(),
        "int32" => "uint32".into(),
        "int64" => "uint64".into(),
        "varint16" => "varuint16".into(),
        "varint32" => "varuint32".into(),
        "varint64" => "varuint64".into(),
        "varint" => "varuint".into(),
        _ => panic!("unexpected signed integer type {:?}", int_type_name),
    }
}

pub fn evaluate_mixing_integer_types(
    type1: &Option<TypeReference>,
    type2: &Option<TypeReference>,
) -> Option<TypeReference> {
    // For now, we use the same logic as C++ when mixing integer types:
    // Mixing signed/unsigned values of the same type (in the sequence of int16, int32, int64),
    // the unsigned type will win.
    // Mixing different ranks result in the higher rank being used.
    match (type1, type2) {
        (Some(t1), Some(t2)) => {
            // If both types are set, check if they are the same type.
            if compare_native_types(t1, t2) {
                return type1.clone();
            }

            // Check if there is a mix in signed/unsigned. If yes, the final
            // type will be a unsigned type (like C++, unsigned wins).
            let t1_is_signed = is_signed(t1.name.as_str());
            let t2_is_signed = is_signed(t2.name.as_str());
            let result_is_signed = t1_is_signed && t2_is_signed;

            // Pick the higher rank.
            let t1_rank = get_integer_rank(t1);
            let t2_rank = get_integer_rank(t2);

            if t1_rank > t2_rank {
                if t1_is_signed == result_is_signed {
                    // type 1 wins
                    return Some(t1.clone());
                }
                // t1 rank wins, but t2 sign wins
                let mut result_type = t1.clone();
                result_type.name = convert_to_unsigned(result_type.name.as_str());
                return Some(result_type);
            }
            if t2_is_signed == result_is_signed {
                // type 2 wins
                return Some(t2.clone());
            }
            // t2 rank wins, but t1 sign wins
            let mut result_type = t2.clone();
            result_type.name = convert_to_unsigned(result_type.name.as_str());
            Some(result_type)
        }
        // Handle cases such as "u16Var + 12"
        (Some(_), None) => type1.clone(),
        (None, Some(_)) => type2.clone(),
        (None, None) => None,
    }
}
