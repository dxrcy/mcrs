use std::fmt;

use crate::Coordinate;

/// 3D size of a [`Chunk`]
#[derive(Clone, Copy)]
pub struct Size {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

/// 2D size of a [`HeightMap`]
#[derive(Clone, Copy)]
pub struct Size2D {
    pub x: u32,
    pub z: u32,
}

impl Size {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }

    /// Convert a [`Chunk`] index to a **relative** [`Coordinate`]
    pub fn index_to_coordinate(&self, index: usize) -> Coordinate {
        let z = (index % self.z as usize) as i32;
        let xy = index / self.z as usize;
        let x = (xy % self.x as usize) as i32;
        let y = (xy / self.x as usize) as i32;
        Coordinate { x, y, z }
    }

    /// Convert a **relative** [`Coordinate`] to a [`Chunk`] index
    pub fn coordinate_to_index(&self, coordinate: impl Into<Coordinate>) -> usize {
        let coordinate = coordinate.into();
        let [x, y, z] = [
            coordinate.x as usize,
            coordinate.y as usize,
            coordinate.z as usize,
        ];
        z + (x + y * self.x as usize) * self.z as usize
    }

    /// Returns `true` if the **relative** [`Coordinate`] is within the
    /// [`Chunk`] size
    pub fn contains(&self, coordinate: impl Into<Coordinate>) -> bool {
        let coordinate = coordinate.into();
        (0..self.x as i32).contains(&coordinate.x)
            && (0..self.y as i32).contains(&coordinate.y)
            && (0..self.z as i32).contains(&coordinate.z)
    }
}

impl Size2D {
    pub fn new(x: u32, z: u32) -> Self {
        Self { x, z }
    }

    pub(crate) fn from(size: crate::Size) -> Self {
        Self {
            x: size.x,
            z: size.z,
        }
    }

    /// Convert a [`HeightMap`] index to a **relative** `y`-agnostic
    /// [`Coordinate`]
    pub fn index_to_coordinate(&self, index: usize) -> Coordinate {
        let z = (index % self.z as usize) as i32;
        let x = (index / self.z as usize) as i32;
        Coordinate { x, y: 0, z }
    }

    /// Convert a **relative** `y`-agnostic [`Coordinate`] to a [`HeightMap`]
    /// index
    pub fn coordinate_to_index(&self, coordinate: impl Into<Coordinate>) -> usize {
        let coordinate = coordinate.into();
        coordinate.z as usize + coordinate.x as usize * self.z as usize
    }

    /// Returns `true` if the **relative** `y`-agnostic [`Coordinate`] is within
    /// the [`HeightMap`] size
    pub fn contains(self, coordinate: impl Into<Coordinate>) -> bool {
        let coordinate = coordinate.into();
        (0..self.x as i32).contains(&coordinate.x) && (0..self.z as i32).contains(&coordinate.z)
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
