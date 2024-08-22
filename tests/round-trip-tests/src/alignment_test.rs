use reference_module_lib::reference_modules::alignment::alignment::AlignmentStruct;

use bitreader::BitReader;
use rust_bitwriter::BitWriter;
use zserio::{Result, ZserioPackableObject};

pub fn test_alignment() -> Result<()> {
    let mut test_struct = AlignmentStruct::default();

    // Even though bo_value_4 is not set, the alignment of the field is still taken into
    // account, because the "is present" bit is written, and this bit takes the alignment
    // into account.
    assert_eq!(test_struct.zserio_bitsize(0)?, 17);

    test_struct.bo_value_4 = Some(true);
    assert_eq!(test_struct.zserio_bitsize(0)?, 18);

    // Set the condition to serialize bo_value_5 to true (bo_value_5 depends on bo_value_3).
    test_struct.bo_value_3 = true;
    assert_eq!(test_struct.zserio_bitsize(0)?, 21);
    Ok(())
}

pub fn test_alignment_roundtrip() -> Result<()> {
    let mut test_struct = AlignmentStruct {
        bo_value_4: Some(true),
        ..Default::default()
    };
    // Set the condition to serialize bo_value_5 to true (bo_value_5 depends on bo_value_3).
    test_struct.bo_value_3 = true;

    // serialize
    let mut bitwriter = BitWriter::new();
    test_struct.zserio_write(&mut bitwriter)?;
    assert_eq!(test_struct.zserio_bitsize(0)?, bitwriter.bit_count());
    bitwriter.close().expect("failed to close bit stream");
    let serialized_bytes = bitwriter.data();

    // deserialize
    let mut other_test_struct = AlignmentStruct::default();
    let mut bitreader = BitReader::new(serialized_bytes);
    other_test_struct.zserio_read(&mut bitreader)?;

    // expect them to be identical.
    assert_eq!(test_struct, other_test_struct);
    Ok(())
}
