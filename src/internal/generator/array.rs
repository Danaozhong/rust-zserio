use crate::internal::ast::field::Field;
use crate::internal::ast::type_reference::TypeReference;
use crate::internal::generator::native_type::get_fundamental_type;
use crate::internal::generator::types::{ztype_to_rust_type};
use codegen::Function;

pub fn array_type_name(name: &String) -> String {
    String::from("zs_array_") + name
}

pub fn get_array_trait_for_type(zserio_type: &TypeReference) -> String {
    if !zserio_type.is_builtin {
        "ObjectArrayTrait".into()
    } else {
        match zserio_type.name.as_str() {
            "int8" => return "BitFieldArrayTrait".into(),
            "int16" => return "BitFieldArrayTrait".into(),
            "int32" => return "BitFieldArrayTrait".into(),
            "int64" => return "BitFieldArrayTrait".into(),
            "varint32" => return "VarUintArrayTrait".into(),
            "uint8" => return "UnsignedBitFieldArrayTrait".into(),
            "uint16" => return "UnsignedBitFieldArrayTrait".into(),
            "uint32" => return "UnsignedBitFieldArrayTrait".into(),
            "varuint32" => return "VarUint32ArrayTrait".into(),
            "string" => return "StringArrayTrait".into(),
            "float16" => return "F16ArrayTrait".into(),
            "float32" => return "F32ArrayTrait".into(),
            "float64" => return "F64ArrayTrait".into(),
            "bool" => return "BooleanArrayTrait".into(),
            "bit" => return "UnsignedBitFieldArrayTrait".into(),
            "int" => return "BitFieldArrayTrait".into(),
            "extern" => return "ObjectArrayTrait".into(),
            _ => panic!("failed to identify array trait"),
        }
    }
}

pub fn initialize_array_trait(zserio_type: &TypeReference) -> String {
    let mut code_str = format!(
        "ztype::array_traits::{}{{\n",
        get_array_trait_for_type(zserio_type)
    )
    .into();

    // check if the array traits need initialization
    if zserio_type.is_builtin {
        // Initialize the signed/unsigned bitfield array traits
        let mut bits = zserio_type.bits;
        if bits == 0 {
            match zserio_type.name.as_str() {
                "int8" => bits = 8,
                "int16" => bits = 16,
                "int32" => bits = 32,
                "int64" => bits = 64,
                "uint8" => bits = 8,
                "uint16" => bits = 16,
                "uint32" => bits = 32,
                "uint64" => bits = 64,
                _ => bits = 0,
            }
        }
        if bits != 0 {
            code_str += format!("num_bits: {},\n", zserio_type.bits).as_str();
        }
    }

    code_str += "}";
    print!("{}", code_str);
    code_str
}

pub fn instantiate_zserio_array(function: &mut Function, field: &Field) {
    let native_type = get_fundamental_type(&*field.field_type);
    let fund_type = native_type.fundamental_type;
    let rust_type = ztype_to_rust_type(field.field_type.as_ref());
    // also initialize the array part
    function.line(format!(
        "let mut {} = ztype::Array::<{}>{{",
        array_type_name(&field.name),
        rust_type
    ));
    function.line(format!(
        "array_trait: Box::new({}),",
        initialize_array_trait(fund_type.as_ref())
    ));
    // TODO function.line(format!("fixed_size: {},", array.array_length_expression.is_some()));
    function.line("fixed_size: None,");
    function.line(format!("is_aligned: {},", field.alignment != 0));
    //TODO function.line(format!("is_packed: {},", field.array.unwrap().is_packed));
    function.line(format!("is_packed: {},", false));

    function.line("packing_context_node: None,");

    function.line("};");
}

pub fn instantiate_zserio_arrays(function: &mut Function, fields: &Vec<Field>) {
    for field in fields {
        if field.array.is_some() {
            instantiate_zserio_array(function, field);
        }
    }
}
