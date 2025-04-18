use bitreader::BitReader;
use rust_bitwriter::BitWriter;
use serde::Deserialize;
use serde_json;
use std::env;
use std::path::PathBuf;
use zserio::{Result, ZserioPackableObject};

pub fn get_test_directory() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.join("..").join("artifacts")
}

/// Reads a zserio-encoded object a binary file generated with Python, and ensures
/// 1) That the file encoded with the python lib can be read,
/// 2) That the object is binary identical to the Python code after serializing it again in rust,
/// 3) Ensures the bit positions and bit counts are identical.
pub fn read_from_python_and_compare(
    name: &str,
    zserio_object: &mut impl ZserioPackableObject,
) -> Result<()> {
    // Deserialize the binary file generated with Python
    let bin_path = get_test_directory().join(format!("{}_from_python.bin", name));
    let data_bytes = std::fs::read(bin_path).expect("failed to read test data");
    let mut bitreader = BitReader::new(&data_bytes);
    zserio_object.zserio_read(&mut bitreader)?;

    // Ensure than when serializing the file again, it is still the same.
    let mut bitwriter = BitWriter::new();
    zserio_object.zserio_write(&mut bitwriter)?;
    bitwriter
        .close()
        .expect("failed to closed bitwriter bit stream");
    assert_eq!(
        bitwriter.data(),
        &*data_bytes,
        "binary data is different to the Python reference implementation"
    );

    compare_bitlength_calculations_with_python_reference(name, zserio_object);
    Ok(())
}

#[derive(Debug, Deserialize)]
struct PythonReferenceJson {
    pub bitsize: u64,
}

/// For a zserio-packable object, this function checks if the calculated bit
/// length values match to what the python reference implementation calculates.
pub fn compare_bitlength_calculations_with_python_reference<T: ZserioPackableObject>(
    name: &str,
    zserio_object: &T,
) {
    // Load the reference numbers
    let json_path = get_test_directory().join(format!("{}_stats.json", name));
    let json_file_content =
        String::from_utf8(std::fs::read(json_path).expect("failed to read test data"))
            .expect("failed to parse file as utf-8");

    let python_reference_json: PythonReferenceJson =
        serde_json::from_str(&json_file_content).expect("failed to parse file content as json");

    // Compare bitsize
    let actual_bitsize = zserio_object
        .zserio_bitsize(0)
        .expect("can not determine bitsize");
    assert_eq!(
        python_reference_json.bitsize, actual_bitsize,
        "bitsize calculations don't match"
    );
}
