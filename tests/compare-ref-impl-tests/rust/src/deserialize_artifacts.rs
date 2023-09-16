use bitreader::BitReader;
use rust_bitwriter::BitWriter;
use rust_zserio::ztype::ZserioPackableObject;
use std::env;
use std::path::PathBuf;

pub fn get_test_directory() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.join("..").join("artifacts")
}

/// Reads a zserio-enocoded object a binary file generated with Python, and ensures
/// 1) That the file encoded with the python lib can be read,
/// 2) That the object is binary identical to the Python code after serializing it again in rust,
/// 3) Ensures the bit positions and bit counts are identical.
pub fn read_from_python_and_compare(name: &str, zserio_object: &mut impl ZserioPackableObject) {
    // Deserialize the binary file generated with Python
    let bin_path = get_test_directory().join(format!("{}_from_python.bin", name));
    let data_bytes = std::fs::read(bin_path).expect("failed to read test data");
    let mut bitreader = BitReader::new(&data_bytes);
    zserio_object.zserio_read(&mut bitreader);

    // Ensure than when serializing the file again, it is still the same.
    let mut bitwriter = BitWriter::new();
    zserio_object.zserio_write(&mut bitwriter);
    bitwriter
        .close()
        .expect("failed to closed bitwriter bit stream");
    assert_eq!(
        bitwriter.data(),
        &*data_bytes,
        "binary data is diffferent to the Python reference implementation"
    );
}
