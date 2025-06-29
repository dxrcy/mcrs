use std::{fmt, ops};

use crate::{Coordinate, Size2D};

/// A worldspace or offset coordinate in the Minecraft world, with no y-value
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coordinate2D {
    pub x: i32,
    pub z: i32,
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

    pub const fn size_between(self, other: Self) -> Size2D {
        Size2D {
            x: (self.x - other.x).unsigned_abs() + 1,
            z: (self.z - other.z).unsigned_abs() + 1,
        }
    }

    pub const fn magnitude(self) -> Size2D {
        Size2D {
            x: self.x.abs() as u32,
            z: self.z.abs() as u32,
        }
    }

    pub const fn with_height(self, height: i32) -> Coordinate {
        Coordinate {
            x: self.x,
            y: height,
            z: self.z,
        }
    }

    pub const fn flip_if(self, condition: bool) -> Self {
        if condition {
            Self::new(self.z, self.x)
        } else {
            self
        }
    }
}

impl fmt::Display for Coordinate2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.z)
    }
}
impl fmt::Debug for Coordinate2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Coordinate2D({}, {})", self.x, self.z)
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

impl From<[i32; 2]> for Coordinate2D {
    fn from([x, z]: [i32; 2]) -> Self {
        Self { x, z }
    }
}
impl From<(i32, i32)> for Coordinate2D {
    fn from((x, z): (i32, i32)) -> Self {
        Self { x, z }
    }
}

impl<T> From<T> for Coordinate2D
where
    T: Into<Size2D>,
{
    fn from(value: T) -> Self {
        let size = value.into();
        Self {
            x: size.x as i32,
            z: size.z as i32,
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
