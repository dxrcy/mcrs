use std::fmt;
use std::io::Read;
use std::net::TcpStream;

use crate::error::IntegerError;
use crate::{Block, Coordinate, Error, Result};

const BUFFER_SIZE: usize = 0x2000;

#[derive(Debug)]
pub struct ResponseStream<'a> {
    reader: IntegerStream<'a, TcpStream>,
}

impl<'a> ResponseStream<'a> {
    pub fn new(reader: &'a mut BufReader<TcpStream>) -> Result<Self> {
        let reader = IntegerStream::new(reader);
        Ok(Self { reader })
    }

    pub fn next_i32(&mut self) -> Result<i32> {
        self.reader
            .read::<i32>()?
            .expect_terminator(Terminator::Comma)
    }

    pub fn final_i32(&mut self) -> Result<i32> {
        self.reader
            .read::<i32>()?
            .expect_terminator(Terminator::Newline)
    }

    pub fn next_block(&mut self) -> Result<Block> {
        let id = self
            .reader
            .read::<u32>()?
            .expect_terminator(Terminator::Comma)?;
        let modifier = self
            .reader
            .read::<u32>()?
            .expect_terminator(Terminator::Semicolon)?;
        Ok(Block { id, modifier })
    }

    pub fn final_block(&mut self) -> Result<Block> {
        let id = self
            .reader
            .read::<u32>()?
            .expect_terminator(Terminator::Comma)?;
        let modifier = self
            .reader
            .read::<u32>()?
            .expect_terminator(Terminator::Newline)?;
        Ok(Block { id, modifier })
    }

    pub fn final_coordinate(&mut self) -> Result<Coordinate> {
        let x = self
            .reader
            .read::<i32>()?
            .expect_terminator(Terminator::Comma)?;
        let y = self
            .reader
            .read::<i32>()?
            .expect_terminator(Terminator::Comma)?;
        let z = self
            .reader
            .read::<i32>()?
            .expect_terminator(Terminator::Newline)?;
        Ok(Coordinate { x, y, z })
    }
}

#[derive(Debug)]
pub struct BufReader<R> {
    inner: R,
    buffer: [u8; BUFFER_SIZE],
    index: usize,
    length: usize,
}

impl<R> BufReader<R>
where
    R: Read,
{
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            buffer: [0u8; BUFFER_SIZE],
            index: usize::MAX,
            length: 0,
        }
    }

    pub fn next(&mut self) -> Result<u8> {
        let byte = self.peek()?;
        self.index += 1;
        Ok(byte)
    }

    pub fn peek(&mut self) -> Result<u8> {
        if self.index >= self.length {
            let bytes_read = self.inner.read(&mut self.buffer)?;
            if bytes_read == 0 {
                return Err(Error::UnexpectedEof);
            }
            self.index = 0;
            self.length = bytes_read;
        }
        Ok(self.buffer[self.index])
    }
}

#[derive(Debug)]
struct IntegerStream<'a, R> {
    inner: &'a mut BufReader<R>,
}

impl<'a, R> IntegerStream<'a, R>
where
    R: Read,
{
    pub fn new(inner: &'a mut BufReader<R>) -> Self {
        Self { inner }
    }

    pub fn read<T>(&mut self) -> Result<WithTerminator<T>>
    where
        T: TryFrom<i32>,
    {
        let sign = match self.inner.peek()? {
            b'-' => {
                self.inner.next()?;
                -1
            }
            b'+' => {
                self.inner.next()?;
                1
            }
            _ => 1,
        };

        let mut integer: i32 = 0;
        let mut len = 0;

        // Take digits until any non-digit character is peeked
        loop {
            let byte = self.inner.peek()?;
            let digit = match byte {
                b'0'..=b'9' => (byte - b'0') as i32,
                _ => break,
            };
            self.inner.next()?;

            integer *= 10;
            integer += digit;
            len += 1;
        }

        if len == 0 {
            // `^[-+]?$`
            return Err(IntegerError::Empty.into());
        }

        integer *= sign;

        // Decimal point and following digits
        if self.inner.peek()? == b'.' {
            self.inner.next()?;

            let mut is_integer = true; // Whether all decimal digits are '0'
            loop {
                let byte = self.inner.peek()?;
                match byte {
                    b'0' => (),
                    b'1'..=b'9' => is_integer = false,
                    _ => break,
                }
                self.inner.next()?;
            }
            // Ensure number is always rounded down, NOT truncated
            // Without this, `-1.3` would become `-1` (instead of `-2`)
            if !is_integer && sign < 0 {
                integer -= 1;
            }
        }

        // Check and consume byte following integer
        let Ok(terminator) = self.inner.next()?.try_into() else {
            return Err(IntegerError::InvalidDigit.into());
        };

        let Ok(integer) = integer.try_into() else {
            return Err(IntegerError::Overflow.into());
        };

        Ok(WithTerminator {
            value: integer,
            terminator,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Terminator {
    Comma,
    Semicolon,
    Newline,
}

impl fmt::Display for Terminator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Comma => write!(f, "comma (,)"),
            Self::Semicolon => write!(f, "semicolon (;)"),
            Self::Newline => write!(f, "newline (\\n)"),
        }
    }
}

impl TryFrom<u8> for Terminator {
    type Error = ();
    fn try_from(byte: u8) -> std::result::Result<Self, Self::Error> {
        match byte {
            b',' => Ok(Terminator::Comma),
            b';' => Ok(Terminator::Semicolon),
            b'\n' => Ok(Terminator::Newline),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct WithTerminator<T> {
    value: T,
    terminator: Terminator,
}

impl<T> WithTerminator<T> {
    pub fn expect_terminator(self, expected: Terminator) -> Result<T> {
        if self.terminator != expected {
            return Err(Error::UnexpectedTerminator {
                expected,
                actual: self.terminator,
            });
        }
        Ok(self.value)
    }
}
