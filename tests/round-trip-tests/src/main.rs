pub mod reference_modules {
    pub mod core {
        pub mod types;
    }
}

use crate::reference_modules::core::types::value_wrapper;

use bitreader::BitReader;
use rust_bitwriter::BitWriter;
use rust_zserio::ztype::ZserioPackableOject;

fn main() {
    // This test generates a test structure, serializes it, deserializes it, and ensures
    // that the data is still the same.

    // Instantiate the data
    let mut value_wrapper = value_wrapper::ValueWrapper::new();
    value_wrapper.value = 18;
    value_wrapper.description = "test".into();
    value_wrapper.fixed_array = vec![100, 101, 102, 103];

    // serialize
    let mut bitwriter = BitWriter::new();
    value_wrapper.zserio_write(&mut bitwriter);
    bitwriter.close().expect("failed to close bit stream");
    let serialized_byes = bitwriter.data();

    // deserialize
    let mut other_value_wrapper = value_wrapper::ValueWrapper::new();
    let mut bitreader = BitReader::new(serialized_byes);
    other_value_wrapper.zserio_read(&mut bitreader);

    assert!(other_value_wrapper.value == value_wrapper.value);
    assert!(other_value_wrapper.other_value == value_wrapper.other_value);
    assert!(other_value_wrapper.description == value_wrapper.description);
    assert!(other_value_wrapper.fixed_array == value_wrapper.fixed_array);

    // serialize the new structure again, and ensure it is binary identical
    let mut other_bitwriter = BitWriter::new();
    other_value_wrapper.zserio_write(&mut other_bitwriter);
    other_bitwriter.close().expect("failed to close bit stream");
    let other_serialized_bytes = other_bitwriter.data();
    assert!(other_serialized_bytes == serialized_byes);
}
