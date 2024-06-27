//!
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
mod error;
pub mod internal;
pub mod ztype;

pub use self::error::*;
