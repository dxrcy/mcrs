use std::io::Read;
use std::mem::MaybeUninit;
use std::net::TcpStream;
use std::str::Split;

use crate::{Block, Coordinate, Error};

// TODO(opt): Use custom buffered reader wrapper
#[derive(Debug)]
pub(crate) struct ResponseStream<'a> {
    stream: &'a mut TcpStream,
}

const ITEM_BUFFER_SIZE: usize = 48;

#[derive(Debug)]
struct ItemBuffer {
    bytes: MaybeUninit<[u8; ITEM_BUFFER_SIZE]>,
    length: usize,
    terminator: Terminator,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Terminator {
    Comma,
    Semicolon,
    Newline,
}

impl ItemBuffer {
    pub fn read_from<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut bytes = MaybeUninit::<[u8; ITEM_BUFFER_SIZE]>::uninit();
        let mut length = 0;

        let terminator = loop {
            let mut buf = [0u8; 1];
            let bytes_read = reader.read(&mut buf).map_err(Error::from)?;
            // All responses must end with '\n', therefore all response ITEMS must end in *some*
            // terminator.
            if bytes_read == 0 {
                return Err(Error::UnexpectedEOF);
            }

            let byte = buf[0];
            match byte {
                b',' => break Terminator::Comma,
                b';' => break Terminator::Semicolon,
                b'\n' => break Terminator::Newline,
                0x80.. => return Err(Error::NonAscii { byte }),
                _ => (),
            }

            if length >= ITEM_BUFFER_SIZE {
                return Err(Error::ItemTooLarge {
                    max_length: ITEM_BUFFER_SIZE,
                });
            }

            // SAFETY: We are not reading from this reference, so it is fine to access
            // uninitialized.
            let bytes = unsafe { bytes.assume_init_mut() };
            bytes[length] = byte;
            length += 1;
        };

        Ok(ItemBuffer {
            bytes,
            length,
            terminator,
        })
    }

    fn as_slice(&self) -> &[u8] {
        // SAFETY: When the buffer was constructed, all bytes in `0..self.length` were initialized.
        unsafe { &self.bytes.assume_init_ref()[0..self.length] }
    }

    fn as_str(&self) -> &str {
        // SAFETY: The buffer was only constructed from ASCII bytes, and could not not have been
        // mutated after construction.
        unsafe { std::str::from_utf8_unchecked(self.as_slice()) }
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
    pub fn new(stream: &'a mut TcpStream) -> Self {
        Self { stream }
    }

    fn next(&mut self) -> Result<ItemBuffer, Error> {
        ItemBuffer::read_from(&mut self.stream)
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
