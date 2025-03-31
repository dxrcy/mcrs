use std::fmt;

use crate::{Coordinate, Coordinate2D};

/// 3D size in blocks.
///
/// Used by [`Chunk`].
#[derive(Clone, Copy)]
pub struct Size {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

/// 2D size in blocks.
///
/// Used by [`Heights`].
#[derive(Clone, Copy)]
pub struct Size2D {
    pub x: u32,
    pub z: u32,
}

impl Size {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
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
    pub(crate) fn index_to_coordinate(&self, index: usize) -> Coordinate {
        let z = (index % self.z as usize) as i32;
        let xy = index / self.z as usize;
        let x = (xy % self.x as usize) as i32;
        let y = (xy / self.x as usize) as i32;
        Coordinate { x, y, z }
    }

    /// Convert an **offset** [`Coordinate`] to a [`Chunk`] index.
    ///
    /// [`Chunk`]: crate::Chunk
    pub(crate) fn coordinate_to_index(&self, coordinate: impl Into<Coordinate>) -> usize {
        let coordinate = coordinate.into();
        let [x, y, z] = [
            coordinate.x as usize,
            coordinate.y as usize,
            coordinate.z as usize,
        ];
        z + (x + y * self.x as usize) * self.z as usize
    }
}

impl Size2D {
    pub fn new(x: u32, z: u32) -> Self {
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
    pub(crate) fn index_to_coordinate(&self, index: usize) -> Coordinate2D {
        let z = (index % self.z as usize) as i32;
        let x = (index / self.z as usize) as i32;
        Coordinate2D { x, z }
    }

    /// Convert an **offset** [`Coordinate2D`] to a [`Heights`] index.
    ///
    /// [`Heights`]: crate::Heights
    pub(crate) fn coordinate_to_index(&self, coordinate: impl Into<Coordinate2D>) -> usize {
        let coordinate = coordinate.into();
        coordinate.z as usize + coordinate.x as usize * self.z as usize
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

impl From<Size> for Size2D {
    fn from(size: Size) -> Self {
        Self {
            x: size.x,
            z: size.z,
        }
    }
}
