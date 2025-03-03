//! This module contains re-exports from dependencies that are needed by
//! the generated code. This allows generated code to only depend on the
//!  zserio crate.
pub use bitmask_enum::bitmask;
pub use bitreader::BitReader;
pub use rust_bitwriter::BitWriter;
