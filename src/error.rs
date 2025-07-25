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

#[derive(Debug)]
pub struct OutOfBoundsError;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IO(error) => write!(f, "Input/output error: {}", error)?,
            Self::ParseInt(error) => write!(f, "Parsing integer: {}", error)?,
            Self::UnexpectedTerminator { expected, actual } => write!(
                f,
                "Unexpected response terminator: expected {}, found {}",
                expected, actual,
            )?,
            Self::UnexpectedEof => write!(f, "Unexpected end of stream")?,
        }
        Ok(())
    }
}

impl fmt::Display for IntegerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "Empty value")?,
            Self::InvalidDigit => write!(f, "Invalid digit")?,
            Self::Overflow => write!(f, "Value would overflow")?,
        }
        Ok(())
    }
}

impl fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Position out of bounds")
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
