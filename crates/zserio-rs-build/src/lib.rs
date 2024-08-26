//! This package provides a compiler to Generate rust code for the [zserio](http://zserio.org/)
//! serialization formst. `zserio` is a binary serialization language, similar to Protobuf. The key
//! features are:
//!
//! - It features a rich schema.
//! - Programming language agnostic.
//! - Compact and easy to use.
//! - Good and fast out-of the box compression.
//!
//! The syntax is similar to C/C++, which makes zserio code easy to read.
//!
//! For more information on zserio in Rust please see [zserio](https://crates.io/crates/zserio).
mod internal;

pub use internal::generator::model::generate_model;
pub use internal::model::Model;
