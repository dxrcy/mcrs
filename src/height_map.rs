use std::{cmp::Ordering, fmt};

use crate::{Coordinate, Size2D};

/// Stores a 2D area of the world with the `y`-values of the highest solid block
/// at each (`x`, `z`)
#[derive(Clone)]
pub struct HeightMap {
    list: Vec<i32>,
    origin: Coordinate,
    size: Size2D,
}

impl HeightMap {
    pub(crate) fn new(a: impl Into<Coordinate>, b: impl Into<Coordinate>, list: Vec<i32>) -> Self {
        let a = a.into();
        let b = b.into();
        Self {
            list,
            origin: a.min(b),
            size: Size2D::from(a.size_between(b)),
        }
    }

    /// Get the height value at the **relative** `y`-agnostic [`Coordinate`]
    pub fn get(&self, coordinate: impl Into<Coordinate>) -> Option<i32> {
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

    /// Get the height value at the **absolute** `y`-agnostic [`Coordinate`]
    pub fn get_absolute(&self, coordinate: impl Into<Coordinate>) -> Option<i32> {
        self.get(coordinate.into() - self.origin)
    }

    /// Get the origin [`Coordinate`]
    pub fn origin(&self) -> Coordinate {
        self.origin
    }

    /// Get the 2D size of the height map
    pub fn size(&self) -> Size2D {
        self.size
    }

    /// Create an iterator over the height values in the height map
    pub fn iter(&self) -> Iter {
        Iter::from(self)
    }
}

impl fmt::Debug for HeightMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<HeightMap {:?}>", self.size)
    }
}

impl<'a> IntoIterator for &'a HeightMap {
    type Item = IterItem<'a>;
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Iter::from(self)
    }
}

/// An iterator over the height values in a [`HeightMap`]
pub struct Iter<'a> {
    height_map: &'a HeightMap,
    index: usize,
}

/// An iterated item in a [`HeightMap`]
pub struct IterItem<'a> {
    height_map: &'a HeightMap,
    index: usize,
}

impl<'a> Iter<'a> {
    /// Create an iterator over the height values in a [`HeightMap`]
    pub fn from(chunk: &'a HeightMap) -> Self {
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
    /// Get a reference to the entire [`HeightMap`]
    pub fn height_map(&self) -> &'a HeightMap {
        self.height_map
    }

    /// Get the height value corresponding to the [`HeightMap`] item
    pub fn height(&self) -> i32 {
        *self
            .height_map
            .list
            .get(self.index)
            .expect("should be valid index in chunk")
    }

    /// Get the **relative** `y`-agnostic [`Coordinate`] corresponding to the
    /// [`HeightMap`] item
    pub fn position_relative(&self) -> Coordinate {
        self.height_map.size.index_to_coordinate(self.index)
    }

    /// Get the **absolute** `y`-agnostic [`Coordinate`] corresponding to the
    /// [`HeightMap`] item
    pub fn position_absolute(&self) -> Coordinate {
        self.position_relative() + self.height_map.origin
    }
}

impl<'a> fmt::Debug for IterItem<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<HeightMap item {} {}>",
            self.position_relative(),
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
