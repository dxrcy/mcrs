use std::fmt;

use crate::{Block, Coordinate};

/// A request argument which is lazily serialized with [`fmt::Display`].
pub enum Argument<'a> {
    Coordinate(Coordinate),
    Block(Block),
    Integer(i32),
    String(&'a str),
}

impl fmt::Display for Argument<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Coordinate(coordinate) => {
                write!(f, "{},{},{}", coordinate.x, coordinate.y, coordinate.z)?;
            }
            Self::Block(block) => {
                write!(f, "{},{}", block.id, block.modifier)?;
            }
            Self::Integer(integer) => {
                write!(f, "{}", integer)?;
            }
            Self::String(string) => {
                for ch in string.chars() {
                    match ch {
                        '\n' => write!(f, " ")?,
                        '\t' | '\x20'..='\x7e' => write!(f, "{}", ch)?,
                        _ => (),
                    }
                }
            }
        }
        Ok(())
    }
}
