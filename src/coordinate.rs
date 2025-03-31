use std::{fmt, ops};

use crate::{Size, Size2D};

/// An absolute or relative coordinate in the Minecraft world
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// An absolute or relative coordinate in the Minecraft world, with no y-value
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Coordinate2D {
    pub x: i32,
    pub z: i32,
}

impl Coordinate {
    /// Create a new coordinate
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    // TODO(rename): Possibly a misleading method name?
    pub fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    pub fn size_between(self, other: Self) -> Size {
        Size {
            x: (self.x - other.x).unsigned_abs() + 1,
            y: (self.y - other.y).unsigned_abs() + 1,
            z: (self.z - other.z).unsigned_abs() + 1,
        }
    }
}

impl Coordinate2D {
    /// Create a new 2D coordinate
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    pub fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            z: self.z.min(other.z),
        }
    }

    pub fn size_between(self, other: Self) -> Size2D {
        Size2D {
            x: (self.x - other.x).unsigned_abs() + 1,
            z: (self.z - other.z).unsigned_abs() + 1,
        }
    }

    pub fn with_height(self, height: i32) -> Coordinate {
        Coordinate {
            x: self.x,
            y: height,
            z: self.z,
        }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl fmt::Display for Coordinate2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.z)
    }
}
impl fmt::Debug for Coordinate2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.z)
    }
}

impl<T> ops::Add<T> for Coordinate
where
    T: Into<Self>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl<T> ops::Sub<T> for Coordinate
where
    T: Into<Self>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> ops::Add<T> for Coordinate2D
where
    T: Into<Self>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self {
            x: self.x + rhs.x,
            z: self.z + rhs.z,
        }
    }
}
impl<T> ops::Sub<T> for Coordinate2D
where
    T: Into<Self>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self {
            x: self.x - rhs.x,
            z: self.z - rhs.z,
        }
    }
}

impl From<[i32; 3]> for Coordinate {
    fn from(value: [i32; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}
impl From<(i32, i32, i32)> for Coordinate {
    fn from(value: (i32, i32, i32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl From<[i32; 2]> for Coordinate2D {
    fn from(value: [i32; 2]) -> Self {
        Self {
            x: value[0],
            z: value[1],
        }
    }
}
impl From<(i32, i32)> for Coordinate2D {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0,
            z: value.1,
        }
    }
}

impl From<Coordinate> for Coordinate2D {
    fn from(coord: Coordinate) -> Self {
        Self {
            x: coord.x,
            z: coord.z,
        }
    }
}
