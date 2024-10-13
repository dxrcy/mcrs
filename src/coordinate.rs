use std::{fmt, ops};

use crate::chunk::ChunkSize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Coordinate {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    // TODO(rename): Possibly a misleading method name?
    pub(crate) fn min(self, other: Self) -> Self {
        Coordinate {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    pub(crate) fn size_between(self, other: Self) -> ChunkSize {
        ChunkSize {
            x: (self.x - other.x).abs() as u32 + 1,
            y: (self.y - other.y).abs() as u32 + 1,
            z: (self.z - other.z).abs() as u32 + 1,
        }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ops::Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}