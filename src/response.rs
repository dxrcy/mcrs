use std::io::{self, BufRead as _, BufReader, Read as _};
use std::net::TcpStream;
use std::str::Split;

use crate::{Block, Coordinate};

#[derive(Debug)]
pub(crate) struct ResponseStream<'a> {
    stream: &'a mut TcpStream,
}

impl<'a> ResponseStream<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Self {
        Self { stream }
    }

    // TODO(feat): Use custom error type
    // TODO(feat): Handle `,` vs `;` vs '\n' separators (currently treating the same)
    pub fn next_u32(&mut self) -> io::Result<Option<u32>> {
        let mut item = String::new();
        loop {
            let mut buf = [0u8; 1];
            let bytes_read = self.stream.read(&mut buf).unwrap();
            if bytes_read == 0 {
                break;
            }
            let ch = buf[0] as char;
            if matches!(ch, ',' | ';' | '\n') {
                break;
            }
            item.push(ch);
        }
        Ok(Some(item.parse().unwrap()))
    }

    pub fn next_block(&mut self) -> io::Result<Option<Block>> {
        // TODO: Handle unexpected EOF
        let id = self.next_u32().unwrap().expect("unexpected eof");
        let modifier = self.next_u32().unwrap().expect("unexpected eof");
        Ok(Some(Block { id, modifier }))
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
