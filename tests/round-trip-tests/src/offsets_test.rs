use bitreader::BitReader;
use reference_module_lib::reference_modules::offsets::offsets::offsets::Offsets;
use rust_bitwriter::BitWriter;

use rust_zserio::ztype::ZserioPackableObject;

pub fn test_offsets() {
    let mut test_struct = Offsets::new();

    test_struct.u_32_offset = 1;
    test_struct.vi_32_array = vec![10, 11, 13];
    test_struct.vi_16_offset_array = vec![1, 3, 2, 7, -12];
    test_struct.u_32_array_offset = vec![1, 3, 2, 4, 2];
    test_struct.vi_64_index_offset_array = vec![100, 102, -2435425, -32543, 1510001];
    test_struct.u_8_check = 127;
    test_struct.u_16_final_check = 5523;
    test_struct.has_flag = false;
    test_struct.u_16_offset = 4;
    test_struct.u_32_value = 242; // this value should not be stored, because it depends on `has_flag`.
    test_struct.u_16_yet_final_check = 42;

    // serialize
    let mut bitwriter = BitWriter::new();
    test_struct.zserio_write(&mut bitwriter);
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // deserialize
    let mut other_test_struct = Offsets::new();
    let mut bitreader = BitReader::new(serialized_bytes);
    other_test_struct.zserio_read(&mut bitreader);

    // expect them to be identical.
    test_struct.u_32_value = 0;
    assert!(test_struct == other_test_struct);

    // try again with the optional field enabled.
    test_struct.u_32_value = 5252;
    test_struct.has_flag = true;

    // serialize
    let mut bitwriter = BitWriter::new();
    test_struct.zserio_write(&mut bitwriter);
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // deserialize
    let mut other_test_struct = Offsets::new();
    let mut bitreader = BitReader::new(serialized_bytes);
    other_test_struct.zserio_read(&mut bitreader);

    // expect them to be identical.
    assert!(test_struct == other_test_struct);
}
