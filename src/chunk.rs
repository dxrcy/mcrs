use std::fmt;

use crate::{Block, Coordinate, Size};

/// Stores a 3D cuboid of [`Block`]s while preserving their location relative to
/// the base point they were gathered.
#[derive(Clone)]
pub struct Chunk {
    list: Vec<Block>,
    origin: Coordinate,
    size: Size,
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

    /// Get the [`Block`] at the **offset** [`Coordinate`].
    pub fn get_offset(&self, coordinate: impl Into<Coordinate>) -> Option<Block> {
        let coordinate = coordinate.into();
        if !self.size.contains(coordinate) {
            return None;
        }
        let index = self.size.offset_to_index(coordinate);
        assert!(
            index < self.list.len(),
            "calculated index should be less than internal list length"
        );
        Some(self.list[index])
    }

    /// Get the [`Block`] at the **worldspace** [`Coordinate`]
    pub fn get_worldspace(&self, coordinate: impl Into<Coordinate>) -> Option<Block> {
        self.get_offset(coordinate.into() - self.origin)
    }

    /// Get the origin [`Coordinate`].
    pub fn origin(&self) -> Coordinate {
        self.origin
    }

    /// Get the 3D size of the chunk.
    pub fn size(&self) -> Size {
        self.size
    }

    /// Create an iterator over the blocks in the chunk.
    pub fn iter(&self) -> Iter {
        Iter::from(self)
    }
}

impl fmt::Debug for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Chunk {:?}>", self.size)
    }
}

impl<'a> IntoIterator for &'a Chunk {
    type Item = IterItem<'a>;
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Iter::from(self)
    }
}

/// An iterator over the blocks in a [`Chunk`].
///
/// Holds a shared reference to the original [`Chunk`].
pub struct Iter<'a> {
    chunk: &'a Chunk,
    index: usize,
}

/// An iterated item in a [`Chunk`].
///
/// Holds a reference to the original [`Chunk`].
pub struct IterItem<'a> {
    chunk: &'a Chunk,
    index: usize,
}

impl<'a> Iter<'a> {
    // TODO(refactor): Remove and inline in `Chunk::iter`
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
    /// Get a shared reference to the entire [`Chunk`].
    pub fn chunk(&self) -> &'a Chunk {
        self.chunk
    }

    /// Get the [`Block`] corresponding to the [`Chunk`] item.
    pub fn block(&self) -> Block {
        *self
            .chunk
            .list
            .get(self.index)
            .expect("should be valid index in chunk")
    }

    /// Get the **offset** [`Coordinate`] corresponding to the [`Chunk`] item.
    pub fn position_offset(&self) -> Coordinate {
        self.chunk.size.index_to_offset(self.index)
    }

    /// Get the **worldspace** [`Coordinate`] corresponding to the [`Chunk`] item.
    pub fn position_worldspace(&self) -> Coordinate {
        self.position_offset() + self.chunk.origin
    }
}

impl fmt::Debug for IterItem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<Chunk item {:?} {:?}>",
            self.position_offset(),
            self.block(),
        )
    }
}
