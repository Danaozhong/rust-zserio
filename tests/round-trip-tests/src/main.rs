pub mod reference_modules {
    pub mod core {
        pub mod types;
    }
}

use crate::reference_modules::core::types::{
    basic_choice::BasicChoice, color::Color, some_enum::SomeEnum, value_wrapper,
};

use bitreader::BitReader;
use rust_bitwriter::BitWriter;
use rust_zserio::ztype::ZserioPackableOject;

fn main() {
    test_structure();
    test_choice();
}

fn test_structure() {
    // This test generates a test structure, serializes it, deserializes it, and ensures
    // that the data is still the same.

    // Instantiate the data
    let mut value_wrapper = value_wrapper::ValueWrapper::new();
    value_wrapper.value = 18;
    value_wrapper.enum_value = Color::Red;
    value_wrapper.description = "test".into();
    value_wrapper.fixed_array = vec![100, 101, 102, 103];
    value_wrapper.packed_array = vec![200, 201, 202, 203, 205, 204];

    // serialize
    let mut bitwriter = BitWriter::new();
    value_wrapper.zserio_write(&mut bitwriter);
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // deserialize
    let mut other_value_wrapper = value_wrapper::ValueWrapper::new();
    let mut bitreader = BitReader::new(serialized_bytes);
    other_value_wrapper.zserio_read(&mut bitreader);

    assert!(other_value_wrapper.value == value_wrapper.value);
    assert!(other_value_wrapper.enum_value == value_wrapper.enum_value);
    assert!(other_value_wrapper.other_value == value_wrapper.other_value);
    assert!(other_value_wrapper.description == value_wrapper.description);
    assert!(other_value_wrapper.fixed_array == value_wrapper.fixed_array);
    assert!(other_value_wrapper.packed_array == value_wrapper.packed_array);

    // serialize the new structure again, and ensure it is binary identical
    let mut other_bitwriter = BitWriter::new();
    other_value_wrapper.zserio_write(&mut other_bitwriter);
    other_bitwriter.close().expect("failed to close bit stream");
    let other_serialized_bytes = other_bitwriter.data();
    assert!(other_serialized_bytes == serialized_bytes);
}

fn test_choice() {
    // This test generates a test zserio choice, serializes it, deserializes it, and ensures
    // that the data is identical to what was serialized.
    let choice_param = SomeEnum::AttrC;

    let mut basic_choice = BasicChoice::new();
    basic_choice.z_type = choice_param;
    basic_choice.field_c = 42;

    // Serialize to binary.
    let mut bitwriter = BitWriter::new();
    basic_choice.zserio_write(&mut bitwriter);
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // Deserialize the binary stream.
    let mut other_basic_choice = BasicChoice::new();
    other_basic_choice.z_type = choice_param;
    let mut bitreader = BitReader::new(serialized_bytes);
    other_basic_choice.zserio_read(&mut bitreader);

    assert!(other_basic_choice.field_c == basic_choice.field_c);

    // serialize the new structure again, and ensure it is binary identical
    let mut other_bitwriter = BitWriter::new();
    other_basic_choice.zserio_write(&mut other_bitwriter);
    other_bitwriter.close().expect("failed to close bit stream");
    let other_serialized_bytes = other_bitwriter.data();
    assert!(other_serialized_bytes == serialized_bytes);
}
