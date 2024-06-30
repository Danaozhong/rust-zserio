//! [zserio](http://zserio.org/) serialization bindings for rust.
//! `zserio` is a binary serialization language, similar to Protobuf. The key features are:
//! - It features a rich schema.
//! - Programming language agnostic.
//! - Compact and easy to use.
//! - Good and fast out-of the box compression.
//!
//! The syntax is similar to C/C++, which makes zserio code easy to read.
//!
//! Example:
//! ```ignore
//! struct DrinkOrder
//! {
//!     string customerName;
//!     string orderName;
//!     bool addSugar;
//!     bool addMilk;
//!     uint32 price;
//!     optional string extraWishes;
//!     uint32 extraCharges;
//!     
//!     function uint32 getTotal() {
//!         return price + extraCharges;
//!     };
//! };
//! ```
//! Above code is `zserio` code. Using the `zserio` serialization bindings, this
//! code can be used to generate C++/Python/Java code to read/write `zserio`-encoded
//! data.
//! With this crate, it is now possible to read/write `zserio`-encoded binary data
//! in rust.
//! To compile a `zs` file with rust-zserio, first install rust-zserio:
//!
//! ```sh
//! cargo install rust-zserio
//! ```
//!
//! Afterwards, you can run the code generator:
//!
//! ```sh
//! rust-zserio --root=<code_root_path> -o=<output_directory> <path_to_zserio_files>
//! ```
//! This will generate the interface files in rust, that allow reading/writing zserio-encoded
//! data.
//! [Optional] The `root` CLI flag is optional, ane enforces an overall crate prefix to the generated code.
pub mod doctest;
mod error;
pub mod internal;
pub mod ztype;

pub use self::error::*;
pub use ztype::ZserioPackableObject;

use bitreader::BitReader;
use std::fs::File;
use std::path::Path;

/// Deserialize an instance of `T` from data.
///
/// # Example
///
/// Read a smart layer tile from a byte slice.
///
/// ```
/// # use rust_zserio::doctest::DrinkOrder;
///
/// let data: &[u8] = b"binarydata";
/// let tile: DrinkOrder = rust_zserio::from_bytes(data).expect("can not parse data");
/// println!("{tile:?}");
/// ```
///
/// # Errors
///
/// This function will fail of there an error occurred if the data can not be decoded,
/// or there is not enough data in the slice.
pub fn from_bytes<T: ZserioPackableObject>(data: &[u8]) -> self::error::Result<T> {
    let mut bitreader = BitReader::new(data);
    let mut v = T::new();
    v.zserio_read(&mut bitreader)?;
    Ok(v)
}

/// Read an instance of `T` from a IO stream.
///
/// *The stream will be fully consumed by this function.* Any extra data is
/// ignored.
///
/// # Example
///
/// Read a smart layer tile from a file.
///
/// ```
/// # use rust_zserio::doctest::DrinkOrder;
/// use std::fs::File;
///
/// let file = File::open("tests/12345678.bin").unwrap();
/// let tile: DrinkOrder = rust_zserio::from_reader(file).unwrap();
/// println!("{tile:?}");
/// ```
///
/// # Errors
///
/// This function will fail of there an error occurred while reading data from
/// the stream, or if the data can not be decoded.
pub fn from_reader<R: std::io::Read, T: ZserioPackableObject>(
    mut reader: R,
) -> self::error::Result<T> {
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    from_bytes(&buffer)
}

/// Read an instance of `T` from a file.
///
/// # Example
///
/// Read a smart layer tile from a file.
///
/// ```
/// # use rust_zserio::doctest::DrinkOrder;
/// let order: DrinkOrder = rust_zserio::from_file("tests/12345678.bin").unwrap();
/// println!("{order:?}");
/// ```
///
/// # Errors
///
/// This function will fail of there an error occurred while reading data from
/// the stream, or if the data can not be decoded.
pub fn from_file<P: AsRef<Path>, T: ZserioPackableObject>(path: P) -> self::error::Result<T> {
    let mut file = File::open(path)?;
    from_reader(&mut file)
}
