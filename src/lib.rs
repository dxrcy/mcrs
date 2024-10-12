mod blocks;
mod command;
mod connection;
mod response;

pub use command::Arg;
pub use connection::Connection;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block {
    pub id: i32,
    pub modifier: i32,
}

impl Coordinate {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl Block {
    pub const fn new(id: i32, modifier: i32) -> Self {
        Self { id, modifier }
    }
}
