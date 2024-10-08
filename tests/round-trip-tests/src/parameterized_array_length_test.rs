use reference_module_lib::reference_modules::parameterized_array_length::parameterized_array_length::ParameterizedArrayLength;

use bitreader::BitReader;
use rust_bitwriter::BitWriter;
use zserio::{Result, ZserioPackableObject};

pub fn test_parameterized_array_length() -> Result<()> {
    let test_struct = ParameterizedArrayLength {
        num_elements: 2,
        data: vec![10, 15], // the array length should be specified by num_elements
    };

    // The bit length should be 16 (for num_elements) + 2 * 32 (for data).
    // there should no array length be encoded, as the array length is given
    // using num_elements.
    assert_eq!(
        test_struct.zserio_bitsize(0)?,
        16 + 2 * 32,
        "bit length of parameterized array length doesn't match"
    );
    // serialize
    let mut bitwriter = BitWriter::new();
    test_struct
        .zserio_write(&mut bitwriter)
        .expect("can not write zserio data");
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // deserialize
    let mut other_test_struct = ParameterizedArrayLength::default();
    let mut bitreader = BitReader::new(serialized_bytes);
    other_test_struct
        .zserio_read(&mut bitreader)
        .expect("can not read zserio data");

    // expect them to be identical.
    assert_eq!(test_struct, other_test_struct);
    Ok(())
}
