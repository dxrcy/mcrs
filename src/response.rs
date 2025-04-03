use std::io::{BufRead as _, BufReader};
use std::net::TcpStream;
use std::str::Split;

use crate::{Block, Coordinate, Error};

// TODO(doc)
pub const MAX_ITEM_LENGTH: usize = "12345.123456890,".len();
// TODO(doc)
pub const MAX_SCALAR_LENGTH: usize = MAX_ITEM_LENGTH * 3;

#[derive(Debug)]
pub(crate) struct ResponseStream<'a> {
    buffer: &'a str,
    index: usize,
}

#[derive(Debug)]
struct ItemBuffer<'a> {
    string: &'a str,
    terminator: Terminator,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Terminator {
    Comma,
    Semicolon,
    Newline,
}

impl<'a> ItemBuffer<'a> {
    pub fn read_from(reader: &mut ResponseStream<'a>) -> Result<Self, Error> {
        let index = reader.index;

        let terminator = loop {
            // All responses must end with '\n', therefore all response ITEMS must end in *some*
            // terminator.
            let Some(byte) = reader.read_byte() else {
                return Err(Error::UnexpectedEOF);
            };

            match byte {
                b',' => break Terminator::Comma,
                b';' => break Terminator::Semicolon,
                b'\n' => break Terminator::Newline,
                0x80.. => return Err(Error::NonAscii { byte }),
                _ => (),
            }
        };

        let string = &reader.buffer[index..reader.index - 1];
        Ok(ItemBuffer { string, terminator })
    }

    fn as_str(&self) -> &str {
        self.string
    }

    pub fn parse_u32(&self) -> Result<WithTerminator<u32>, Error> {
        let string = trim_decimals(self.as_str());
        let value = string.parse().map_err(Error::from)?;
        Ok(WithTerminator {
            value,
            terminator: self.terminator,
        })
    }

    pub fn parse_i32(&self) -> Result<WithTerminator<i32>, Error> {
        let string = trim_decimals(self.as_str());
        let value = string.parse().map_err(Error::from)?;
        Ok(WithTerminator {
            value,
            terminator: self.terminator,
        })
    }
}

fn trim_decimals(string: &str) -> &str {
    match string.find('.') {
        Some(index) => string.split_at(index).0,
        None => string,
    }
}

#[derive(Debug)]
struct WithTerminator<T> {
    value: T,
    terminator: Terminator,
}

impl<T> WithTerminator<T> {
    pub fn expect_terminator(self, expected: Terminator) -> Result<T, Error> {
        if self.terminator != expected {
            return Err(Error::UnexpectedTerminator {
                expected,
                actual: self.terminator,
            });
        }
        Ok(self.value)
    }
}

impl<'a> ResponseStream<'a> {
    pub fn new(
        stream: &mut TcpStream,
        capacity: usize,
        buffer: &'a mut String,
    ) -> Result<Self, Error> {
        buffer.clear();
        BufReader::with_capacity(capacity, stream).read_line(buffer)?;
        Ok(Self { buffer, index: 0 })
    }

    /// This method could easily be changed to instead read by *character*, however there is
    /// currently no need.
    pub fn read_byte(&mut self) -> Option<u8> {
        if self.index >= self.buffer.len() {
            return None;
        }
        let byte = self.buffer.as_bytes()[self.index];
        self.index += 1;
        Some(byte)
    }

    fn next(&mut self) -> Result<ItemBuffer, Error> {
        ItemBuffer::read_from(self)
    }

    pub fn final_i32(&mut self) -> Result<i32, Error> {
        self.next()?
            .parse_i32()?
            .expect_terminator(Terminator::Newline)
    }

    pub fn next_block(&mut self) -> Result<Block, Error> {
        let id = self
            .next()?
            .parse_u32()?
            .expect_terminator(Terminator::Comma)?;
        let modifier = self
            .next()?
            .parse_u32()?
            .expect_terminator(Terminator::Semicolon)?;
        Ok(Block { id, modifier })
    }

    pub fn final_block(&mut self) -> Result<Block, Error> {
        let id = self
            .next()?
            .parse_u32()?
            .expect_terminator(Terminator::Comma)?;
        let modifier = self
            .next()?
            .parse_u32()?
            .expect_terminator(Terminator::Newline)?;
        Ok(Block { id, modifier })
    }

    pub fn final_coordinate(&mut self) -> Result<Coordinate, Error> {
        let x = self
            .next()?
            .parse_i32()?
            .expect_terminator(Terminator::Comma)?;
        let y = self
            .next()?
            .parse_i32()?
            .expect_terminator(Terminator::Comma)?;
        let z = self
            .next()?
            .parse_i32()?
            .expect_terminator(Terminator::Newline)?;
        Ok(Coordinate { x, y, z })
    }
}

#[derive(Debug)]
pub struct Response {
    response: String,
}

impl Response {
    pub fn new(response: String) -> Self {
        Self { response }
    }

    pub fn as_integer_list(&self) -> Vec<i32> {
        IntegerList::from(&self.response).collect()
    }
}

struct IntegerList<'a> {
    inner: Split<'a, char>,
}

impl<'a> IntegerList<'a> {
    pub fn from(line: &'a str) -> Self {
        Self {
            inner: line.split(','),
        }
    }
}

impl Iterator for IntegerList<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.next()?;
        let float: f32 = item.trim().parse().ok()?;
        Some(float.floor() as i32)
    }
}
