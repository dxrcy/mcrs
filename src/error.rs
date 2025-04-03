use std::error;
use std::fmt;
use std::io;
use std::num::ParseIntError;

use crate::response::Terminator;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    UnexpectedEOF,
    ParseInt(ParseIntError),
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
impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Self::ParseInt(error)
    }
}
