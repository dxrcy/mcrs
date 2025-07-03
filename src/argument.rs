use std::fmt;

use crate::{Block, Coordinate, Coordinate2D};

/// A request argument which is lazily serialized with [`fmt::Display`].
pub enum Argument<'a> {
    Coordinate(Coordinate),
    Coordinate2D(Coordinate2D),
    Block(Block),
    Format(fmt::Arguments<'a>),
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
            Self::Format(arguments) => {
                // TODO(fix!): Sanitize
                write!(f, "{}", arguments)?;
            }
        }
        Ok(())
    }
}
