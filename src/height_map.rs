use crate::{chunk::ChunkSize, Coordinate};

#[derive(Clone, Debug)]
pub struct HeightMap {
    list: Vec<i32>,
    origin: Coordinate,
    size: HeightMapSize,
}

#[derive(Clone, Copy, Debug)]
pub struct HeightMapSize {
    pub x: u32,
    pub z: u32,
}

impl HeightMap {
    pub fn new(a: Coordinate, b: Coordinate, list: Vec<i32>) -> Self {
        Self {
            list,
            origin: a.min(b),
            size: HeightMapSize::from(a.size_between(b)),
        }
    }

    pub fn get(&self, coordinate: Coordinate) -> Option<i32> {
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

    pub fn origin(&self) -> Coordinate {
        self.origin
    }
    pub fn size(&self) -> HeightMapSize {
        self.size
    }

    pub fn iter(&self) -> HeightMapIter {
        HeightMapIter::from(self)
    }
}

impl HeightMapSize {
    pub fn from(size: ChunkSize) -> Self {
        Self {
            x: size.x,
            z: size.z,
        }
    }

    pub fn index_to_coordinate(&self, index: usize) -> Coordinate {
        let z = (index % self.z as usize) as i32;
        let x = (index / self.z as usize) as i32;
        Coordinate { x, y: 0, z }
    }

    pub fn coordinate_to_index(&self, coordinate: Coordinate) -> usize {
        coordinate.z as usize + coordinate.x as usize * self.z as usize
    }

    pub fn contains(self, coordinate: Coordinate) -> bool {
        (0..self.x as i32).contains(&coordinate.x) && (0..self.z as i32).contains(&coordinate.z)
    }
}

pub struct HeightMapIter<'a> {
    height_map: &'a HeightMap,
    index: usize,
}

pub struct HeightMapItem<'a> {
    height_map: &'a HeightMap,
    index: usize,
}

impl<'a> HeightMapIter<'a> {
    pub fn from(chunk: &'a HeightMap) -> Self {
        Self {
            height_map: chunk,
            index: 0,
        }
    }
}

impl<'a> Iterator for HeightMapIter<'a> {
    type Item = HeightMapItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.height_map.list.len() {
            return None;
        }
        let index = self.index;
        self.index += 1;
        let item = HeightMapItem {
            height_map: self.height_map,
            index,
        };
        Some(item)
    }
}

impl<'a> HeightMapItem<'a> {
    pub fn chunk(&self) -> &'a HeightMap {
        self.height_map
    }

    pub fn height(&self) -> i32 {
        *self
            .height_map
            .list
            .get(self.index)
            .expect("should be valid index in chunk")
    }

    pub fn position_relative(&self) -> Coordinate {
        self.height_map.size.index_to_coordinate(self.index)
    }
    pub fn position_absolute(&self) -> Coordinate {
        self.position_relative() + self.height_map.origin
    }
}
