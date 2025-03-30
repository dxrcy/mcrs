use std::fmt;

use crate::{Block, Coordinate};

// Stores a 3D cuboid of [`Block`]s while preserving their location relative to
// the base point they were gathered
//
/// [`Block`]: crate::Block
#[derive(Clone)]
pub struct Chunk {
    list: Vec<Block>,
    origin: Coordinate,
    size: Size,
}

/// 3D size of a [`Chunk`]
#[derive(Clone, Copy, Debug)]
pub struct Size {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Chunk {
    pub(crate) fn new(
        a: impl Into<Coordinate>,
        b: impl Into<Coordinate>,
        list: Vec<Block>,
    ) -> Self {
        let a = a.into();
        let b = b.into();
        Self {
            list,
            origin: a.min(b),
            size: a.size_between(b),
        }
    }

    /// Get the [`Block`] at the **relative** [`Coordinate`]
    pub fn get(&self, coordinate: impl Into<Coordinate>) -> Option<Block> {
        let coordinate = coordinate.into();
        if !self.size.contains(coordinate) {
            return None;
        }
        let index = self.size.coordinate_to_index(coordinate);
        assert!(
            index < self.list.len(),
            "calculated index should be less than internal list length"
        );
        Some(self.list[index])
    }

    /// Get the origin [`Coordinate`]
    pub fn origin(&self) -> Coordinate {
        self.origin
    }

    /// Get the 3D size of the chunk
    pub fn size(&self) -> Size {
        self.size
    }

    /// Create an iterator over the blocks in the chunk
    pub fn iter(&self) -> Iter {
        Iter::from(self)
    }
}

impl Size {
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

impl fmt::Debug for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Chunk {}x{}x{}>", self.size.x, self.size.y, self.size.z)
    }
}

impl<'a> IntoIterator for &'a Chunk {
    type Item = IterItem<'a>;
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Iter::from(self)
    }
}

/// An iterator over the blocks in a [`Chunk`]
pub struct Iter<'a> {
    chunk: &'a Chunk,
    index: usize,
}

/// An iterated item in a [`Chunk`]
pub struct IterItem<'a> {
    chunk: &'a Chunk,
    index: usize,
}

impl<'a> Iter<'a> {
    /// Create an iterator over the blocks in a [`Chunk`]
    pub fn from(chunk: &'a Chunk) -> Self {
        Self { chunk, index: 0 }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = IterItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.chunk.list.len() {
            return None;
        }
        let index = self.index;
        self.index += 1;
        let item = IterItem {
            chunk: self.chunk,
            index,
        };
        Some(item)
    }
}

impl<'a> IterItem<'a> {
    /// Get a reference to the entire [`Chunk`]
    pub fn chunk(&self) -> &'a Chunk {
        self.chunk
    }

    /// Get the [`Block`] corresponding to the [`Chunk`] item
    pub fn block(&self) -> Block {
        *self
            .chunk
            .list
            .get(self.index)
            .expect("should be valid index in chunk")
    }

    /// Get the **relative** [`Coordinate`] corresponding to the [`Chunk`] item
    pub fn position_relative(&self) -> Coordinate {
        self.chunk.size.index_to_coordinate(self.index)
    }

    /// Get the **absolute** [`Coordinate`] corresponding to the [`Chunk`] item
    pub fn position_absolute(&self) -> Coordinate {
        self.position_relative() + self.chunk.origin
    }
}

impl<'a> fmt::Debug for IterItem<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<Chunk item {} {}>",
            self.position_relative(),
            self.block(),
        )
    }
}
