use std::{fmt, ops};

use crate::{Coordinate2D, Size, Size2D};

/// A worldspace or offset coordinate in the Minecraft world
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
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

    pub const fn size_between(self, other: Self) -> Size {
        Size {
            x: (self.x - other.x).unsigned_abs() + 1,
            y: (self.y - other.y).unsigned_abs() + 1,
            z: (self.z - other.z).unsigned_abs() + 1,
        }
    }

    pub const fn magnitude(self) -> Size {
        Size {
            x: self.x.unsigned_abs(),
            y: self.y.unsigned_abs(),
            z: self.z.unsigned_abs(),
        }
    }

    // TODO(doc)
    pub fn flat(self) -> Coordinate2D {
        self.into()
    }

    // TODO(doc)
    pub const fn with_height(self, height: i32) -> Coordinate {
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
        write!(f, "Coordinate({}, {}, {})", self.x, self.y, self.z)
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

impl ops::Add<Coordinate2D> for Coordinate {
    type Output = Self;

    fn add(self, rhs: Coordinate2D) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y,
            z: self.z + rhs.z,
        }
    }
}
impl ops::Sub<Coordinate2D> for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Coordinate2D) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Add<Size2D> for Coordinate {
    type Output = Self;

    fn add(self, rhs: Size2D) -> Self::Output {
        Self {
            x: self.x + rhs.x as i32,
            y: self.y,
            z: self.z + rhs.z as i32,
        }
    }
}

impl ops::Sub<Size2D> for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Size2D) -> Self::Output {
        Self {
            x: self.x - rhs.x as i32,
            y: self.y,
            z: self.z - rhs.z as i32,
        }
    }
}

impl From<[i32; 3]> for Coordinate {
    fn from([x, y, z]: [i32; 3]) -> Self {
        Self { x, y, z }
    }
}
impl From<(i32, i32, i32)> for Coordinate {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Self { x, y, z }
    }
}

impl<T> From<T> for Coordinate
where
    T: Into<Size>,
{
    fn from(value: T) -> Self {
        let size = value.into();
        Self {
            x: size.x as i32,
            y: size.y as i32,
            z: size.z as i32,
        }
    }
}
