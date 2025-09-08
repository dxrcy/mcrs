use std::cmp::Ordering;

use crate::error::OutOfBoundsError;
use crate::response::ResponseStream;
use crate::{Coordinate2D, Error, Size2D};

/// Stores a 2D area of the world with the `y`-values of the highest solid block
/// in each column (`x`, `z` coordinate).
#[derive(Clone, Debug)]
pub struct Heights {
    list: Vec<i32>,
    origin: Coordinate2D,
    size: Size2D,
}

impl Heights {
    /// Get the height value at the **offset** [`Coordinate2D`].
    pub fn get_offset(&self, coordinate: impl Into<Coordinate2D>) -> Result<i32, OutOfBoundsError> {
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

    /// Get the height value at the **worldspace** [`Coordinate2D`].
    pub fn get_worldspace(
        &self,
        coordinate: impl Into<Coordinate2D>,
    ) -> Result<i32, OutOfBoundsError> {
        self.get_offset(coordinate.into() - self.origin)
    }

    /// Get the origin [`Coordinate2D`].
    pub const fn origin(&self) -> Coordinate2D {
        self.origin
    }

    /// Get the 2D size of the area.
    pub const fn size(&self) -> Size2D {
        self.size
    }

    // TODO(doc)
    pub fn min(&self) -> i32 {
        self.into_iter()
            .map(|item| item.height())
            .min()
            .expect("height map should not be empty")
    }

    // TODO(doc)
    pub fn max(&self) -> i32 {
        self.into_iter()
            .map(|item| item.height())
            .max()
            .expect("height map should not be empty")
    }
}

#[derive(Debug)]
pub struct HeightsStream<'a> {
    response: ResponseStream<'a>,
    index: usize,
    origin: Coordinate2D,
    size: Size2D,
}

#[derive(Debug)]
pub struct HeightsStreamItem<'a> {
    chunk: &'a HeightsStream<'a>,
    index: usize,
    height: i32,
}

impl<'a> HeightsStream<'a> {
    pub(crate) fn new(
        a: impl Into<Coordinate2D>,
        b: impl Into<Coordinate2D>,
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
    pub fn next(&mut self) -> Result<Option<HeightsStreamItem<'_>>, Error> {
        if self.is_at_end() {
            return Ok(None);
        }

        self.index += 1;
        let height = if self.is_at_end() {
            self.response.final_i32()?
        } else {
            self.response.next_i32()?
        };

        Ok(Some(HeightsStreamItem {
            chunk: self,
            height,
            index: self.index,
        }))
    }

    pub fn collect(mut self) -> Result<Heights, Error> {
        assert!(self.index == 0, "cannot collect partially-consumed stream");
        let mut list = Vec::with_capacity(self.size().area());
        while let Some(item) = self.next()? {
            list.push(item.height);
        }
        Ok(Heights {
            list,
            origin: self.origin,
            size: self.size,
        })
    }

    /// Get the origin [`Coordinate2D`].
    pub const fn origin(&self) -> Coordinate2D {
        self.origin
    }

    /// Get the 3D size of the chunk.
    pub const fn size(&self) -> Size2D {
        self.size
    }

    fn is_at_end(&self) -> bool {
        self.index >= self.size.area()
    }
}

impl HeightsStreamItem<'_> {
    pub fn height(&self) -> i32 {
        self.height
    }

    pub const fn position_offset(&self) -> Coordinate2D {
        self.chunk.size.index_to_offset(self.index)
    }

    pub fn position_worldspace(&self) -> Coordinate2D {
        self.position_offset() + self.chunk.origin
    }
}

impl<'a> IntoIterator for &'a Heights {
    type Item = IterItem<'a>;
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            heights: self,
            index: 0,
        }
    }
}

/// An iterator over the height values in a [`Heights`].
///
/// Holds a shared reference to the original [`Heights`].
#[derive(Debug)]
pub struct Iter<'a> {
    heights: &'a Heights,
    index: usize,
}

/// An iterated item in a [`Heights`].
///
/// Holds a shared reference to the original [`Heights`].
#[derive(Debug)]
pub struct IterItem<'a> {
    heights: &'a Heights,
    index: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = IterItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.heights.list.len() {
            return None;
        }
        let index = self.index;
        self.index += 1;
        let item = IterItem {
            heights: self.heights,
            index,
        };
        Some(item)
    }
}

impl<'a> IterItem<'a> {
    /// Get a shared reference to the entire [`Heights`].
    pub const fn heights(&self) -> &'a Heights {
        self.heights
    }

    /// Get the height value corresponding to the [`Heights`] item.
    pub fn height(&self) -> i32 {
        *self
            .heights
            .list
            .get(self.index)
            .expect("should be valid index in chunk")
    }

    /// Get the **offset** [`Coordinate2D`] corresponding to the [`Heights`] item.
    pub const fn position_offset(&self) -> Coordinate2D {
        self.heights.size.index_to_offset(self.index)
    }

    /// Get the **worldspace** [`Coordinate2D`] corresponding to the [`Heights`] item.
    pub fn position_worldspace(&self) -> Coordinate2D {
        self.position_offset() + self.heights.origin
    }
}

impl PartialEq for IterItem<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.height() == other.height()
    }
}

impl Eq for IterItem<'_> {}

impl PartialOrd for IterItem<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IterItem<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.height() < other.height() {
            return Ordering::Less;
        }
        if self.height() > other.height() {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}
