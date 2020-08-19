#![allow(dead_code)]
#![allow(unused_imports)]
use thiserror::Error;

// TODO: Split into distinct sub-errors
#[derive(Error, Debug)]
pub enum RWDError {
    /// Represents an empty file error. For example, an empty text file being given
    /// as input to some function `count_words()`.
    #[error("RWD contains no data")]
    EmptyFile,

    /// This ocurrs when the first 26 bytes of the header are incorrect
    #[error("The RWD header does not contain the correct byte sequence")]
    IncorrectHeader,

    #[error("RWD is missing trailer metadata")]
    MissingTrailer,

    #[error("RWD is missing footer")]
    MissingFooter,

    /// Below is just a slew of transparent error bindings.
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    Error(#[from] std::array::TryFromSliceError),
}

pub mod header;
//pub mod footer;

pub use crate::header::*;
//use crate::footer::*;