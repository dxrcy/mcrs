use std::fmt;

use crate::{Block, Coordinate, Coordinate2D};

/// A request argument which is lazily serialized with [`fmt::Display`].
pub enum Argument<'a> {
    Coordinate(Coordinate),
    Coordinate2D(Coordinate2D),
    Block(Block),
    String(&'a str),
}

impl fmt::Display for Argument<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Coordinate(coordinate) => {
                write!(f, "{},{},{}", coordinate.x, coordinate.y, coordinate.z)?;
            }
            Self::Coordinate2D(coordinate) => {
                write!(f, "{},{}", coordinate.x, coordinate.z)?;
            }
            Self::Block(block) => {
                write!(f, "{},{}", block.id, block.modifier)?;
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
