use bitreader::BitReader;
use reference_module_lib::reference_modules::offsets::offsets::offsets::Offsets;
use rust_bitwriter::BitWriter;

use rust_zserio::ztype::ZserioPackableObject;

pub fn test_offsets() {
    let mut test_struct = Offsets {
        u_32_offset: 1,
        vi_32_array: vec![10, 11, 13],
        vi_16_offset_array: vec![1, 3, 2, 7, -12],
        u_32_array_offset: vec![1, 3, 2, 4, 2],
        vi_64_index_offset_array: vec![100, 102, -2435425, -32543, 1510001],
        u_8_check: 127,
        u_16_final_check: 5523,
        has_flag: false,
        u_16_offset: 4,
        u_32_value: 242, // this value should not be stored, because it depends on `has_flag`
        u_16_yet_final_check: 42,
    };

    // serialize
    let mut bitwriter = BitWriter::new();
    test_struct
        .zserio_write(&mut bitwriter)
        .expect("can not write zserio data");
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // deserialize
    let mut other_test_struct = Offsets::default();
    let mut bitreader = BitReader::new(serialized_bytes);
    other_test_struct
        .zserio_read(&mut bitreader)
        .expect("can read write zserio data");

    // expect them to be identical.
    test_struct.u_32_value = 0;
    assert!(test_struct == other_test_struct);

    // try again with the optional field enabled.
    test_struct.u_32_value = 5252;
    test_struct.has_flag = true;

    // serialize
    let mut bitwriter = BitWriter::new();
    test_struct
        .zserio_write(&mut bitwriter)
        .expect("can not write zserio data");
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // deserialize
    let mut other_test_struct = Offsets::default();
    let mut bitreader = BitReader::new(serialized_bytes);
    other_test_struct
        .zserio_read(&mut bitreader)
        .expect("can not read zserio data");

    // expect them to be identical.
    assert!(test_struct == other_test_struct);
}
