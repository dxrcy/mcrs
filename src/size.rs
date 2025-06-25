use std::{fmt, ops};

use crate::{Coordinate, Coordinate2D};

/// 3D size in blocks.
///
/// Used by [`Chunk`].
///
/// [`Chunk`]: crate::Chunk
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Size {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

/// 2D size in blocks.
///
/// Used by [`Heights`].
///
/// [`Heights`]: crate::Heights
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Size2D {
    pub x: u32,
    pub z: u32,
}

impl Size {
    pub const fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }

    /// Returns `true` if the **offset** [`Coordinate`] is within the size.
    pub fn contains(&self, coordinate: impl Into<Coordinate>) -> bool {
        let coordinate = coordinate.into();
        (0..self.x as i32).contains(&coordinate.x)
            && (0..self.y as i32).contains(&coordinate.y)
            && (0..self.z as i32).contains(&coordinate.z)
    }

    /// Convert a [`Chunk`] index to an **offset** [`Coordinate`].
    ///
    /// [`Chunk`]: crate::Chunk
    pub const fn index_to_offset(&self, index: usize) -> Coordinate {
        let z = (index % self.z as usize) as i32;
        let xy = index / self.z as usize;
        let x = (xy % self.x as usize) as i32;
        let y = (xy / self.x as usize) as i32;
        Coordinate { x, y, z }
    }

    /// Convert an **offset** [`Coordinate`] to a [`Chunk`] index.
    ///
    /// [`Chunk`]: crate::Chunk
    pub fn offset_to_index(&self, coordinate: impl Into<Coordinate>) -> usize {
        let coordinate = coordinate.into();
        let [x, y, z] = [
            coordinate.x as usize,
            coordinate.y as usize,
            coordinate.z as usize,
        ];
        z + (x + y * self.x as usize) * self.z as usize
    }

    /// Returns the amount of blocks in the cuboid volume.
    pub fn volume(&self) -> usize {
        self.x as usize * self.y as usize * self.z as usize
    }

    // TODO(doc)
    pub fn flat(&self) -> Size2D {
        self.clone().into()
    }
}

impl Size2D {
    pub const fn new(x: u32, z: u32) -> Self {
        Self { x, z }
    }

    /// Returns `true` if the **offset** [`Coordinate2D`] is within the size.
    pub fn contains(self, coordinate: impl Into<Coordinate2D>) -> bool {
        let coordinate = coordinate.into();
        (0..self.x as i32).contains(&coordinate.x) && (0..self.z as i32).contains(&coordinate.z)
    }

    /// Convert a [`Heights`] index to an **offset** [`Coordinate2D`].
    ///
    /// [`Heights`]: crate::Heights
    pub const fn index_to_offset(&self, index: usize) -> Coordinate2D {
        let z = (index % self.z as usize) as i32;
        let x = (index / self.z as usize) as i32;
        Coordinate2D { x, z }
    }

    /// Convert an **offset** [`Coordinate2D`] to a [`Heights`] index.
    ///
    /// [`Heights`]: crate::Heights
    pub fn offset_to_index(&self, coordinate: impl Into<Coordinate2D>) -> usize {
        let coordinate = coordinate.into();
        coordinate.z as usize + coordinate.x as usize * self.z as usize
    }

    // Returns the amount of blocks in the flat area.
    pub fn area(&self) -> usize {
        self.x as usize * self.z as usize
    }

    pub const fn with_height(self, height: u32) -> Size {
        Size {
            x: self.x,
            y: height,
            z: self.z,
        }
    }
}

impl fmt::Debug for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}x{}", self.x, self.y, self.z)
    }
}

impl fmt::Debug for Size2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.x, self.z)
    }
}

impl<T> ops::Add<T> for Size
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
impl<T> ops::Sub<T> for Size
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

impl<T> ops::Add<T> for Size2D
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
impl<T> ops::Sub<T> for Size2D
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

impl<T> ops::Mul<T> for Size
where
    T: Into<Self>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T> ops::Mul<T> for Size2D
where
    T: Into<Self>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self {
            x: self.x * rhs.x,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Div<u32> for Size {
    type Output = Self;

    fn div(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Div<u32> for Size2D {
    type Output = Self;

    fn div(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x / rhs,
            z: self.z / rhs,
        }
    }
}

impl From<[u32; 3]> for Size {
    fn from(value: [u32; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}
impl From<[u32; 2]> for Size2D {
    fn from(value: [u32; 2]) -> Self {
        Self {
            x: value[0],
            z: value[1],
        }
    }
}
impl From<(u32, u32, u32)> for Size {
    fn from(value: (u32, u32, u32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}
impl From<(u32, u32)> for Size2D {
    fn from(value: (u32, u32)) -> Self {
        Self {
            x: value.0,
            z: value.1,
        }
    }
}

impl From<Size> for Size2D {
    fn from(size: Size) -> Self {
        Self {
            x: size.x,
            z: size.z,
        }
    }
}
