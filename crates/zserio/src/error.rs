//! Error and Result types for zserio
//!
use std::string::FromUtf8Error;
use std::{io, result};
use thiserror::Error;

/// Result type for zserio errors.
pub type Result<T, E = ZserioError> = result::Result<T, E>;

/// Error enumeration of zserio errors.
#[derive(Debug, Error)]
pub enum ZserioError {
    #[error("unexpected end of data")]
    Eof,
    #[error("requested data does not fit in variable")]
    DataDoesNotFit,
    #[error("invalid UTF-8 data")]
    InvalidUtf8(#[from] FromUtf8Error),
    #[error("IO error")]
    IoError(#[from] io::Error),
    #[error("data error: {0}")]
    DataError(String),
}

impl From<bitreader::BitReaderError> for ZserioError {
    fn from(e: bitreader::BitReaderError) -> ZserioError {
        match e {
            bitreader::BitReaderError::NotEnoughData {
                position: _,
                length: _,
                requested: _,
            } => ZserioError::Eof,
            bitreader::BitReaderError::TooManyBitsForType {
                position: _,
                requested: _,
                allowed: _,
            } => ZserioError::DataDoesNotFit,
        }
    }
}
