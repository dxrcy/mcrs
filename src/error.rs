use std::error;
use std::fmt;
use std::io;
use std::num::ParseIntError;

use crate::response::Terminator;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    UnexpectedEof,
    ParseInt(ParseIntError),
    ParseInt2(IntegerError),
    ItemTooLarge {
        max_length: usize,
    },
    NonAscii {
        byte: u8,
    },
    UnexpectedTerminator {
        expected: Terminator,
        actual: Terminator,
    },
}

#[derive(Debug)]
pub enum IntegerError {
    Empty,
    InvalidDigit,
    Overflow,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO(feat): Implement proper error message
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::IO(error)
    }
}
// TODO: Remove
impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Self::ParseInt(error)
    }
}
impl From<IntegerError> for Error {
    fn from(error: IntegerError) -> Self {
        Self::ParseInt2(error)
    }
}
