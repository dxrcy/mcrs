use std::fmt::{self, Write as _};

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
                Sanitizer { inner: f }.write_fmt(*arguments)?;
            }
        }
        Ok(())
    }
}

struct Sanitizer<W> {
    inner: W,
}
impl<W> fmt::Write for Sanitizer<W>
where
    W: fmt::Write,
{
    fn write_str(&mut self, string: &str) -> fmt::Result {
        for ch in string.chars() {
            match ch {
                '\n' => write!(self.inner, " ")?,
                '\t' | '\x20'..='\x7e' => write!(self.inner, "{}", ch)?,
                _ => (),
            }
        }
        Ok(())
    }
}
