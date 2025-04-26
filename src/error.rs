use std::error;
use std::fmt;
use std::io;

use crate::response::Terminator;

#[derive(Debug)]
pub enum Error {
    // TODO(feat): Remove generic IO error kind
    IO(io::Error),
    ParseInt(IntegerError),
    UnexpectedTerminator {
        expected: Terminator,
        actual: Terminator,
    },
    UnexpectedEof,
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
impl From<IntegerError> for Error {
    fn from(error: IntegerError) -> Self {
        Self::ParseInt(error)
    }
}
