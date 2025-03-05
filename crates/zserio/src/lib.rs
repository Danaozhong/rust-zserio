//! [zserio](http://zserio.org/) serialization bindings for rust. `zserio` is a binary
//! serialization language, similar to Protobuf. The key features are:
//!
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
//! code can be used to generate C++/Python/Java code to read/write
//! `zserio`-encoded data. With this crate, it is now possible to read/write
//! `zserio`-encoded binary data in rust. To generate Rust code for a zserio schema
//! you must first install `zserio-rs-build`:
//!
//! ```sh
//! cargo install zserio-rs-build
//! ```
//!
//! Afterwards, you can run the code generator:
//!
//! ```sh
//! rust-rs-build --root=<code_root_path> -o=<output_directory> <path_to_zserio_files>
//! ```
//!
//! This will generate the interface files in rust, that allow reading/writing
//! zserio-encoded data.
//!
//! The `--root` CLI flag is optional, and enforces an overall crate prefix to
//! the generated code.
//!
//! The generated code is formatted, but does not match the exact standards from
//! rustfmt. If you are using rustfmt from the nightly channel you can set
//! [`format_generated_files =
//! false`](https://rust-lang.github.io/rustfmt/?version=v1.6.0&search=#format_generated_files)
//! in `rustfmt.toml` (see
//! [rust-lang/rustfmt#5080](https://github.com/rust-lang/rustfmt/issues/5080).

mod error;
mod numbits;
pub mod ztype;

pub use error::*;
pub use numbits::*;
pub use ztype::ZserioPackableObject;

pub mod doctest;

use bitreader::BitReader;
use rust_bitwriter::BitWriter;
use std::fs::File;
use std::path::Path;

/// Serialize an instance of `T` to a a byte vector
///
/// # Example
///
/// ```
/// # use zserio::doctest::DrinkOrder;
/// let order = DrinkOrder{ customer_name: "Jane Doe".into() };
/// let data: Vec<u8> = zserio::to_bytes(&order).expect("can not write data");
/// ```
///
/// # Errors
///
/// This function will fail if the data can not be encoded.
pub fn to_bytes<T: ZserioPackableObject>(v: &T) -> self::error::Result<Vec<u8>> {
    let mut writer = BitWriter::new();
    v.zserio_write(&mut writer)?;
    writer.close()?;
    Ok(writer.data().clone())
}

/// Serialize an instance of `T` to a writer instance
///
/// # Example
///
/// ```
/// # use zserio::doctest::DrinkOrder;
/// use std::fs::File;
///
/// let order = DrinkOrder{ customer_name: "Jane Doe".into() };
/// let mut out = File::create("tests/12345678.bin").expect("can not create file");
/// zserio::to_writer(&mut out, &order).expect("can not write data");
/// ```
///
/// # Errors
///
/// This function will fail if an IO error occurs or if the data can not be
/// encoded.
pub fn to_writer<W: std::io::Write, T: ZserioPackableObject>(
    mut out: W,
    v: &T,
) -> self::error::Result<()> {
    let mut writer = BitWriter::new();
    v.zserio_write(&mut writer)?;
    writer.close()?;
    Ok(out.write_all(writer.data())?)
}

/// Deserialize an instance of `T` from data.
///
/// This function does not support [parametrized
/// types](https://zserio.org/doc/ZserioLanguageOverview.html#parameterized-types).
///
/// # Example
///
/// Read a DrinkOrder from a byte slice.
///
/// ```
/// # use zserio::doctest::DrinkOrder;
/// let data: &[u8] = b"binarydata";
/// let tile: DrinkOrder = zserio::from_bytes(data).expect("can not parse data");
/// println!("{tile:?}");
/// ```
///
/// # Errors
///
/// This function will fail if the data can not be decoded, or there is not
/// enough data in the slice.
pub fn from_bytes<T: ZserioPackableObject>(data: &[u8]) -> self::error::Result<T> {
    let mut bitreader = BitReader::new(data);
    let mut v: T = Default::default();
    v.zserio_read(&mut bitreader)?;
    Ok(v)
}

/// Read an instance of `T` from a IO stream.
///
/// *The stream will be fully consumed by this function.* Any extra data is
/// ignored.
///
/// This function does not support [parametrized
/// types](https://zserio.org/doc/ZserioLanguageOverview.html#parameterized-types).
///
/// # Example
///
/// Read a DrinkOrder from a file.
///
/// ```
/// # use zserio::doctest::DrinkOrder;
/// use std::fs::File;
///
/// let file = File::open("tests/12345678.bin").unwrap();
/// let tile: DrinkOrder = zserio::from_reader(file).unwrap();
/// println!("{tile:?}");
/// ```
///
/// # Errors
///
/// This function will fail if there an IO error occurrs, if the data can not be
/// decoded, or there is not enough data in the slice.
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
/// Read a zserio object from a file.
///
/// This function does not support [parametrized
/// types](https://zserio.org/doc/ZserioLanguageOverview.html#parameterized-types).
///
/// ```
/// # use zserio::doctest::DrinkOrder;
/// let order: DrinkOrder = zserio::from_file("tests/12345678.bin").unwrap();
/// println!("{order:?}");
/// ```
///
/// # Errors
///
/// This function will fail if there an IO error occurred,if the data can not be
/// decoded, or there is not enough data in the slice.
pub fn from_file<P: AsRef<Path>, T: ZserioPackableObject>(path: P) -> self::error::Result<T> {
    let mut file = File::open(path)?;
    from_reader(&mut file)
}
