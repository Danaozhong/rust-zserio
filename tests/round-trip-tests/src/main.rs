pub mod reference_modules {
    pub mod core {
       pub mod types;
    }
}
   
use crate::reference_modules::core::types::{
    ValueWrapper,
    Color,   
};

use rust_bitwriter::BitWriter;
use bitreader::BitReader;

fn main() {
    // This test generates a test structure, serializes it, deserializes it, and ensures
    // that the data is still the same.

    // Instantiate the data
    let value_wrapper = ValueWrapper::ValueWrapper{
        value: 1,
        other_value: 3,
        enum_value: Color::Color::BLACK,
        description: String::from("te2222st"),
        opt_int_32: Option::from(12),
    };
    
    // serialize
    let mut bitwriter = BitWriter::new();
    value_wrapper.marshal_zserio(&mut bitwriter);
    bitwriter.close().expect("failed to close bit stream");
    let serialized_byes = bitwriter.data();

    // deserialize
    let mut other_value_wrapper = ValueWrapper::ValueWrapper{
        value: 0,
        other_value: 0,
        enum_value: Color::Color::BLACK,
        description: String::from(""),
        opt_int_32: Option::None,
    };
    let mut bitreader = BitReader::new(&serialized_byes);
    other_value_wrapper.unmarshal_zserio(&mut bitreader);

    assert!(other_value_wrapper.value == value_wrapper.value);
    assert!(other_value_wrapper.other_value == value_wrapper.other_value);
    assert!(other_value_wrapper.description == value_wrapper.description);
    
    // serialize the new structure again, and ensure it is binary identical
    let mut other_bitwriter = BitWriter::new();
    other_value_wrapper.marshal_zserio(&mut other_bitwriter);
    other_bitwriter.close().expect("failed to close bit stream");
    let other_serialized_bytes = other_bitwriter.data();
    assert!(other_serialized_bytes == serialized_byes);
}
