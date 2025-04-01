use std::{cmp::Ordering, fmt};

use crate::{Coordinate2D, Size2D};

/// Stores a 2D area of the world with the `y`-values of the highest solid block
/// in each column (`x`, `z` coordinate).
#[derive(Clone)]
pub struct Heights {
    list: Vec<i32>,
    origin: Coordinate2D,
    size: Size2D,
}

impl Heights {
    pub(crate) fn new(
        a: impl Into<Coordinate2D>,
        b: impl Into<Coordinate2D>,
        list: Vec<i32>,
    ) -> Self {
        let a = a.into();
        let b = b.into();
        Self {
            list,
            origin: a.min(b),
            size: a.size_between(b),
        }
    }

    /// Get the height value at the **offset** [`Coordinate2D`].
    pub fn get_offset(&self, coordinate: impl Into<Coordinate2D>) -> Option<i32> {
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

    /// Get the height value at the **worldspace** [`Coordinate2D`].
    pub fn get_worldspace(&self, coordinate: impl Into<Coordinate2D>) -> Option<i32> {
        self.get_offset(coordinate.into() - self.origin)
    }

    /// Get the origin [`Coordinate2D`].
    pub fn origin(&self) -> Coordinate2D {
        self.origin
    }

    /// Get the 2D size of the area.
    pub fn size(&self) -> Size2D {
        self.size
    }

    /// Create an iterator over the height values in the area.
    pub fn iter(&self) -> Iter {
        Iter::from(self)
    }
}

impl fmt::Debug for Heights {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<HeightMap {:?}>", self.size)
    }
}

impl<'a> IntoIterator for &'a Heights {
    type Item = IterItem<'a>;
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Iter::from(self)
    }
}

/// An iterator over the height values in a [`Heights`].
///
/// Holds a shared reference to the original [`Heights`].
pub struct Iter<'a> {
    height_map: &'a Heights,
    index: usize,
}

/// An iterated item in a [`Heights`].
///
/// Holds a shared reference to the original [`Heights`].
pub struct IterItem<'a> {
    height_map: &'a Heights,
    index: usize,
}

impl<'a> Iter<'a> {
    // TODO(refactor): Remove and inline in `Heights::iter`
    pub fn from(chunk: &'a Heights) -> Self {
        Self {
            height_map: chunk,
            index: 0,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = IterItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.height_map.list.len() {
            return None;
        }
        let index = self.index;
        self.index += 1;
        let item = IterItem {
            height_map: self.height_map,
            index,
        };
        Some(item)
    }
}

impl<'a> IterItem<'a> {
    /// Get a shared reference to the entire [`Heights`].
    pub fn height_map(&self) -> &'a Heights {
        self.height_map
    }

    /// Get the height value corresponding to the [`Heights`] item.
    pub fn height(&self) -> i32 {
        *self
            .height_map
            .list
            .get(self.index)
            .expect("should be valid index in chunk")
    }

    /// Get the **offset** [`Coordinate2D`] corresponding to the [`Heights`] item.
    pub fn position_offset(&self) -> Coordinate2D {
        self.height_map.size.index_to_offset(self.index)
    }

    /// Get the **worldspace** [`Coordinate2D`] corresponding to the [`Heights`] item.
    pub fn position_worldspace(&self) -> Coordinate2D {
        self.position_offset() + self.height_map.origin
    }
}

impl fmt::Debug for IterItem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<HeightMap item {} {}>",
            self.position_offset(),
            self.height(),
        )
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
