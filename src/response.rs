use std::str::Split;

use crate::{Block, Coordinate};

pub struct Response {
    response: String,
}

impl Response {
    pub fn new(response: String) -> Self {
        Self { response }
    }

    pub fn as_integer(self) -> Option<i32> {
        self.response.trim().parse().ok()
    }

    pub fn as_coordinate(self) -> Option<Coordinate> {
        let mut iter = IntegerList::from(&self.response);
        let x = iter.next()?;
        let y = iter.next()?;
        let z = iter.next()?;
        Some(Coordinate { x, y, z })
    }

    pub fn as_block(self) -> Option<Block> {
        let mut iter = IntegerList::from(&self.response);
        let id = iter.next()?;
        let modifier = iter.next()?;
        Some(Block { id, modifier })
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
        Some(self.inner.next()?.trim().parse::<f32>().ok()?.floor() as i32)
    }
}
