#![allow(dead_code)]
#![allow(unused_imports)]
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::mem;
use std::convert::TryInto;

use crate::RWDError;

// TODO: Rename
/// The first 26 bytes of a RWD container header
pub static MAGIC_HEADER_BYTES: [u8; 26] = [
    0x54, 0x47, 0x43, 0x4B, 0x02,
    0x00, 0x00, 0x00, 0x03, 0x00,
    0x00, 0x00, 0x02, 0x00, 0x00,
    0x00, 0x02, 0x00, 0x41, 0x00,
    0x41, 0x00, 0x00, 0x00, 0x00,
    0x00
];

/// The header is the first 30 bytes of an RWD container, it is unclear exactly what data
/// is stored within the header.
pub struct Header {
    /// These are the first 26 bytes of the header, I believe them to be a magic number for RWD containers
    pub constant_bytes: [u8; 26],
    /// These are the last 4 bytes of the header, it is unclear what their purpose is
    pub unknown_bytes: [u8; 4],
}

impl Header {
    /// Load a header from a file
    pub fn load(rwd_file: &mut impl Read) -> Result<Header, RWDError> {
        let mut bytes: [u8; 30] = [0; 30];
        rwd_file.read_exact(&mut bytes)?;

        // check that the header is not incorrect/corrupt
        for byte in 0..26 {
            if bytes[byte] != MAGIC_HEADER_BYTES[byte] {
                return Err(RWDError::IncorrectHeader);
            }
        }

        Ok(Header {
            constant_bytes: bytes[0..26].try_into()?,
            unknown_bytes: bytes[26..30].try_into()?,
        })
    }

    /// Create a HeaderBuilder
    pub fn builder() -> HeaderBuilder {
        HeaderBuilder::default()
    }
}

#[derive(Default)]
pub struct HeaderBuilder {
    pub constant_bytes: [u8; 26],
    pub unknown_bytes: [u8; 4],
}

impl HeaderBuilder {
    pub fn new() -> Self {
        Self {
            constant_bytes: MAGIC_HEADER_BYTES,
            unknown_bytes: [0; 4],
        }
    }

    pub fn set_trailing_bytes(self, bytes: [u8; 4]) -> Self {
        Self {
            constant_bytes: self.constant_bytes,
            unknown_bytes: bytes
        }
    }

    pub fn build(self) -> Header {
        Header {
            constant_bytes: MAGIC_HEADER_BYTES,
            unknown_bytes: self.unknown_bytes,
        }
    }
}