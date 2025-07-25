use std::{fmt, ops};

use crate::{Coordinate2D, Size};

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
    pub const fn index_to_offset(self, index: usize) -> Coordinate2D {
        let z = (index % self.z as usize) as i32;
        let x = (index / self.z as usize) as i32;
        Coordinate2D { x, z }
    }

    /// Convert an **offset** [`Coordinate2D`] to a [`Heights`] index.
    ///
    /// [`Heights`]: crate::Heights
    pub fn offset_to_index(self, coordinate: impl Into<Coordinate2D>) -> usize {
        let coordinate = coordinate.into();
        coordinate.z as usize + coordinate.x as usize * self.z as usize
    }

    // Returns the amount of blocks in the flat area.
    pub fn area(self) -> usize {
        self.x as usize * self.z as usize
    }

    pub const fn with_height(self, height: u32) -> Size {
        Size {
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

impl fmt::Display for Size2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.x, self.z)
    }
}
impl fmt::Debug for Size2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Size2D({}, {})", self.x, self.z)
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

impl ops::Mul<u32> for Size2D {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x * rhs,
            z: self.z * rhs,
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

impl From<[u32; 2]> for Size2D {
    fn from(value: [u32; 2]) -> Self {
        Self {
            x: value[0],
            z: value[1],
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
