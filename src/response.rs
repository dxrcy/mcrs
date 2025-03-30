use std::str::Split;

use crate::{Block, Coordinate};

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
