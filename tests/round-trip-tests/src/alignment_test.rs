use crate::reference_modules::alignment::alignment::alignment_struct::AlignmentStruct;

use rust_zserio::ztype::ZserioPackableOject;
use bitreader::BitReader;
use rust_bitwriter::BitWriter;

pub fn test_alignment() {
    let mut test_struct = AlignmentStruct::new();

    // Even though bo_value_4 is not set, the alignment of the field is still taken into
    // account, because the "is present" bit is written, and this bit takes the alignment
    // into account.
    assert!(test_struct.zserio_bitsize(0) == 17);

    test_struct.bo_value_4 = Some(true);
    assert!(test_struct.zserio_bitsize(0) == 18);

    // Set the condition to serialize bo_value_5 to true (bo_value_5 depends on bo_value_3).
    test_struct.bo_value_3 = true;
    assert!(test_struct.zserio_bitsize(0) == 21);
}


pub fn test_alignment_roundtrip() {
    let mut test_struct = AlignmentStruct::new();
    test_struct.bo_value_4 = Some(true);
    // Set the condition to serialize bo_value_5 to true (bo_value_5 depends on bo_value_3).
    test_struct.bo_value_3 = true;

    // serialize
    let mut bitwriter = BitWriter::new();
    test_struct.zserio_write(&mut bitwriter);
    assert!(test_struct.zserio_bitsize(0) == bitwriter.bit_count());
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // deserialize
    let mut other_test_struct = AlignmentStruct::new();
    let mut bitreader = BitReader::new(serialized_bytes);
    other_test_struct.zserio_read(&mut bitreader);

    // expect them to be identical.
    assert!(test_struct == other_test_struct);
}