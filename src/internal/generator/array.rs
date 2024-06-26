use crate::internal::ast::field::Field;
use crate::internal::ast::type_reference::TypeReference;
use crate::internal::compiler::fundamental_type::get_fundamental_type;
use crate::internal::compiler::symbol_scope::ModelScope;
use crate::internal::generator::expression::generate_expression;
use crate::internal::generator::types::TypeGenerator;
use codegen::Function;
use convert_case::{Case, Casing};

pub fn array_type_name(name: &String) -> String {
    String::from("zs_array_") + &name.to_case(Case::Snake)
}

pub fn get_array_trait_for_type(zserio_type: &TypeReference) -> String {
    if !zserio_type.is_builtin {
        "ObjectArrayTrait".into()
    } else {
        match zserio_type.name.as_str() {
            "int8" => "BitFieldArrayTrait".into(),
            "int16" => "BitFieldArrayTrait".into(),
            "int32" => "BitFieldArrayTrait".into(),
            "int64" => "BitFieldArrayTrait".into(),
            "varint16" => "VarInt16ArrayTrait".into(),
            "varint32" => "VarInt32ArrayTrait".into(),
            "varint64" => "VarInt64ArrayTrait".into(),
            "varint" => "VarIntArrayTrait".into(),
            "uint8" => "UnsignedBitFieldArrayTrait".into(),
            "uint16" => "UnsignedBitFieldArrayTrait".into(),
            "uint32" => "UnsignedBitFieldArrayTrait".into(),
            "uint64" => "UnsignedBitFieldArrayTrait".into(),
            "varuint16" => "VarUint16ArrayTrait".into(),
            "varuint32" => "VarUint32ArrayTrait".into(),
            "varuint64" => "VarUint64ArrayTrait".into(),
            "varuint" => "VarUintArrayTrait".into(),
            "varsize" => "VarSizeArrayTrait".into(),
            "string" => "StringArrayTrait".into(),
            "float16" => "Float16ArrayTrait".into(),
            "float32" => "Float32ArrayTrait".into(),
            "float64" => "Float64ArrayTrait".into(),
            "bool" => "BooleanArrayTrait".into(),
            "bit" => "UnsignedBitFieldArrayTrait".into(),
            "int" => "BitFieldArrayTrait".into(),
            "extern" => "BitBufferArrayTrait".into(),
            "bytes" => "ByteBufferArrayTrait".into(),
            _ => panic!("failed to identify array trait {:?}", &zserio_type.name),
        }
    }
}

pub fn initialize_array_trait(
    scope: &ModelScope,
    type_generator: &mut TypeGenerator,
    zserio_type: &TypeReference,
) -> String {
    let mut code_str = format!(
        "ztype::array_traits::{}{{\n",
        get_array_trait_for_type(zserio_type)
    );

    // check if the array traits need initialization
    if zserio_type.is_builtin {
        // Initialize the signed/unsigned bitfield array traits
        if let Some(length_expression) = &zserio_type.length_expression {
            code_str += format!(
                "num_bits: ({}) as u8,\n",
                generate_expression(&length_expression.borrow(), type_generator, scope)
            )
            .as_str();
        } else {
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
                code_str += format!("num_bits: {},\n", bits).as_str();
            }
        }
    }

    code_str += "}";
    code_str
}

pub fn instantiate_zserio_array(
    scope: &ModelScope,
    type_generator: &mut TypeGenerator,
    function: &mut Function,
    field: &Field,
    force_packed: bool,
) {
    if field.array.is_none() {
        return;
    }
    let native_type = get_fundamental_type(&field.field_type, scope);
    let fund_type = native_type.fundamental_type;
    let rust_type = type_generator.ztype_to_rust_type(field.field_type.as_ref());
    let is_packed = field.array.as_ref().unwrap().is_packed || force_packed;

    // also initialize the array part
    function.line(format!(
        "let mut {} = ztype::Array::<{}>{{",
        array_type_name(&field.name),
        rust_type
    ));
    function.line(format!(
        "array_trait: Box::new({}),",
        initialize_array_trait(scope, type_generator, fund_type.as_ref())
    ));
    let array_length_str = match &field.array.as_ref().unwrap().array_length_expression {
        Some(array_length_expression) => format!(
            "Some(({}) as usize)",
            generate_expression(&array_length_expression.borrow(), type_generator, scope)
        ),
        None => "None".to_owned(),
    };
    function.line(format!("fixed_size: {},", array_length_str));

    function.line(format!("is_aligned: {},", use_indexed_offset));

    function.line(format!("is_packed: {},", is_packed));
    function.line("};");
}
