// Error types for GodPotato
// SPDX-License-Identifier: Apache-2.0

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GodPotatoError {
    #[error("Invalid OBJREF signature: expected 0x574F454D, got {0:#x}")]
    InvalidObjRefSignature(u32),

    #[error("Invalid OBJREF type flags: {0:#x}")]
    InvalidObjRefType(u32),

    #[error("Binary parsing error: {0}")]
    BinaryParse(#[from] binrw::Error),

    #[error("UTF-16 decoding error")]
    Utf16Decode,

    #[error("Unexpected end of data")]
    UnexpectedEof,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, GodPotatoError>;
