use std::error;
use std::fmt;
use std::io::{self, Read, Write};
use std::mem::MaybeUninit;
use std::net::TcpStream;
use std::num::ParseIntError;
use std::str::Split;

use crate::{Block, Coordinate};

#[derive(Debug)]
pub enum Error {
    IORead(io::Error),
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
        Self::IORead(error)
    }
}
impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Self::ParseInt(error)
    }
}

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

    pub fn parse_u32(&self) -> Result<(u32, Terminator), Error> {
        let value = self.as_str().parse().map_err(Error::from)?;
        Ok((value, self.terminator))
    }
}

impl Terminator {
    pub fn expect(self, expected: Terminator) -> Result<(), Error> {
        if self != expected {
            return Err(Error::UnexpectedTerminator {
                expected,
                actual: self,
            });
        }
        Ok(())
    }
}

impl<'a> ResponseStream<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Self {
        Self { stream }
    }

    fn next(&mut self) -> Result<ItemBuffer, Error> {
        ItemBuffer::read_from(&mut self.stream)
    }

    pub fn next_block(&mut self) -> Result<(Block, Terminator), Error> {
        let (id, terminator) = self.next()?.parse_u32()?;
        terminator.expect(Terminator::Comma)?;
        let (modifier, terminator) = self.next()?.parse_u32()?;
        Ok((Block { id, modifier }, terminator))
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

    pub fn as_integer(&self) -> Option<i32> {
        self.response.trim().parse().ok()
    }

    pub fn as_coordinate(&self) -> Option<Coordinate> {
        parse_coord(&self.response)
    }

    pub fn as_block(&self) -> Option<Block> {
        parse_block(&self.response)
    }

    pub fn as_integer_list(&self) -> Vec<i32> {
        IntegerList::from(&self.response).collect()
    }

    pub fn as_block_list(&self) -> Option<Vec<Block>> {
        let mut list = Vec::new();
        for item in self.response.split(';') {
            let block = parse_block(item)?;
            list.push(block);
        }
        Some(list)
    }
}

fn parse_coord(item: &str) -> Option<Coordinate> {
    let mut iter = IntegerList::from(item);
    let x = iter.next()?;
    let y = iter.next()?;
    let z = iter.next()?;
    if iter.next().is_some() {
        return None;
    }
    Some(Coordinate { x, y, z })
}

fn parse_block(item: &str) -> Option<Block> {
    let mut iter = IntegerList::from(item);
    let id = iter.next()? as u32;
    let modifier = iter.next()? as u32;
    if iter.next().is_some() {
        return None;
    }
    Some(Block { id, modifier })
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
