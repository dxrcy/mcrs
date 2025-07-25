use crate::error::OutOfBoundsError;
use crate::response::ResponseStream;
use crate::{Block, Coordinate, Error, Size};

/// Stores a 3D cuboid of [`Block`]s while preserving their location relative to
/// the base point they were gathered.
#[derive(Clone, Debug)]
pub struct Chunk {
    list: Vec<Block>,
    origin: Coordinate,
    size: Size,
}

impl Chunk {
    /// Get the [`Block`] at the **offset** [`Coordinate`].
    pub fn get_offset(&self, coordinate: impl Into<Coordinate>) -> Result<Block, OutOfBoundsError> {
        let coordinate = coordinate.into();
        if !self.size.contains(coordinate) {
            return Err(OutOfBoundsError);
        }
        let index = self.size.offset_to_index(coordinate);
        assert!(
            index < self.list.len(),
            "calculated index should be less than internal list length"
        );
        Ok(self.list[index])
    }

    /// Get the [`Block`] at the **worldspace** [`Coordinate`]
    pub fn get_worldspace(
        &self,
        coordinate: impl Into<Coordinate>,
    ) -> Result<Block, OutOfBoundsError> {
        self.get_offset(coordinate.into() - self.origin)
    }

    /// Get the origin [`Coordinate`].
    pub const fn origin(&self) -> Coordinate {
        self.origin
    }

    /// Get the 3D size of the chunk.
    pub const fn size(&self) -> Size {
        self.size
    }
}

impl<'a> IntoIterator for &'a Chunk {
    type Item = IterItem<'a>;
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            chunk: self,
            index: 0,
        }
    }
}

#[derive(Debug)]
pub struct ChunkStream<'a> {
    response: ResponseStream<'a>,
    index: usize,
    origin: Coordinate,
    size: Size,
}

#[derive(Debug)]
pub struct ChunkStreamItem<'a> {
    chunk: &'a ChunkStream<'a>,
    index: usize,
    block: Block,
}

impl<'a> ChunkStream<'a> {
    pub(crate) fn new(
        a: impl Into<Coordinate>,
        b: impl Into<Coordinate>,
        response: ResponseStream<'a>,
    ) -> Self {
        let a = a.into();
        let b = b.into();
        Self {
            response,
            index: 0,
            origin: a.min(b),
            size: a.size_between(b),
        }
    }

    // Cannot be an iterator, due to lifetime problems
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Result<Option<ChunkStreamItem>, Error> {
        if self.is_at_end() {
            return Ok(None);
        }

        self.index += 1;
        let block = if self.is_at_end() {
            self.response.final_block()?
        } else {
            self.response.next_block()?
        };

        Ok(Some(ChunkStreamItem {
            chunk: self,
            block,
            index: self.index,
        }))
    }

    pub fn collect(mut self) -> Result<Chunk, Error> {
        assert!(self.index == 0, "cannot collect partially-consumed stream");
        let mut list = Vec::with_capacity(self.size().volume());
        while let Some(item) = self.next()? {
            list.push(item.block);
        }
        Ok(Chunk {
            list,
            origin: self.origin,
            size: self.size,
        })
    }

    /// Get the origin [`Coordinate`].
    pub const fn origin(&self) -> Coordinate {
        self.origin
    }

    /// Get the 3D size of the chunk.
    pub const fn size(&self) -> Size {
        self.size
    }

    fn is_at_end(&self) -> bool {
        self.index >= self.size.volume()
    }
}

impl ChunkStreamItem<'_> {
    pub fn block(&self) -> Block {
        self.block
    }

    pub const fn position_offset(&self) -> Coordinate {
        self.chunk.size.index_to_offset(self.index)
    }

    pub fn position_worldspace(&self) -> Coordinate {
        self.position_offset() + self.chunk.origin
    }
}

/// An iterator over the blocks in a [`Chunk`].
///
/// Holds a shared reference to the original [`Chunk`].
#[derive(Debug)]
pub struct Iter<'a> {
    chunk: &'a Chunk,
    index: usize,
}

/// An iterated item in a [`Chunk`].
///
/// Holds a reference to the original [`Chunk`].
#[derive(Debug)]
pub struct IterItem<'a> {
    chunk: &'a Chunk,
    index: usize,
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
    pub const fn chunk(&self) -> &'a Chunk {
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
    pub const fn position_offset(&self) -> Coordinate {
        self.chunk.size.index_to_offset(self.index)
    }

    /// Get the **worldspace** [`Coordinate`] corresponding to the [`Chunk`] item.
    pub fn position_worldspace(&self) -> Coordinate {
        self.position_offset() + self.chunk.origin
    }
}
