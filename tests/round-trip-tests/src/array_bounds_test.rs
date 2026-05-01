use bitreader::BitReader;
use reference_module_lib::reference_modules::packed_arrays::packed_arrays::DataStruct;
use rust_bitwriter::BitWriter;
use zserio::ZserioPackableObject;

/// Feed a byte stream that declares 0 packed elements then 1 000 000 plain u32 elements, with
/// no actual element data following.  The decoder must return an error (stream exhausted) rather
/// than attempting to allocate and read 4 MB of garbage.
pub fn test_array_bounds_check_rejects_oversized_length() {
    // varsize(0)         → u_32_packed_array length = 0
    // varsize(1_000_000) → u_32_array length = 1 000 000, but no data follows
    let bytes: &[u8] = &[0x00, 0xBD, 0x84, 0x40];
    let mut reader = BitReader::new(bytes);
    let mut ds = DataStruct::default();
    let result = ds.zserio_read(&mut reader);
    assert!(
        result.is_err(),
        "expected zserio_read to return Err for an oversized array length, got Ok"
    );
}

/// Serialize a real DataStruct with two elements in each array and round-trip it.
/// The decoder must accept the valid stream and reproduce the original data exactly.
pub fn test_array_bounds_check_allows_valid_stream() {
    let original = DataStruct {
        u_32_packed_array: vec![10, 20],
        u_32_array: vec![100, 200],
        ..Default::default()
    };

    // Serialize
    let mut writer = BitWriter::new();
    original
        .zserio_write(&mut writer)
        .expect("serialization failed");
    writer.close().expect("close failed");
    let bytes = writer.data().clone();

    // Deserialize
    let mut reader = BitReader::new(&bytes);
    let mut decoded = DataStruct::default();
    decoded
        .zserio_read(&mut reader)
        .expect("deserialization of valid stream failed");

    assert_eq!(
        original.u_32_packed_array, decoded.u_32_packed_array,
        "u_32_packed_array mismatch after round-trip"
    );
    assert_eq!(
        original.u_32_array, decoded.u_32_array,
        "u_32_array mismatch after round-trip"
    );

    // Re-serialize and confirm bit-identical output
    let mut writer2 = BitWriter::new();
    decoded
        .zserio_write(&mut writer2)
        .expect("re-serialization failed");
    writer2.close().expect("close failed");
    assert_eq!(
        bytes,
        writer2.data().clone(),
        "re-serialized bytes differ from original"
    );
}
