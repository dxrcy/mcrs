use crate::{Block, Coordinate};

#[derive(Clone, Debug)]
pub struct Chunk {
    list: Vec<Block>,
    origin: Coordinate,
    size: ChunkSize,
}

#[derive(Clone, Copy, Debug)]
pub struct ChunkSize {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Chunk {
    pub fn new(a: Coordinate, b: Coordinate, list: Vec<Block>) -> Self {
        Self {
            list,
            origin: a.min(b),
            size: a.size_between(b),
        }
    }

    pub fn get(&self, coordinate: Coordinate) -> Option<Block> {
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
    pub fn size(&self) -> ChunkSize {
        self.size
    }

    pub fn iter(&self) -> ChunkIter {
        ChunkIter::from(self)
    }
}

pub struct ChunkIter<'a> {
    chunk: &'a Chunk,
    index: usize,
}

pub struct ChunkItem<'a> {
    chunk: &'a Chunk,
    index: usize,
}

impl<'a> ChunkIter<'a> {
    pub fn from(chunk: &'a Chunk) -> Self {
        Self { chunk, index: 0 }
    }
}

impl<'a> Iterator for ChunkIter<'a> {
    type Item = ChunkItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.chunk.list.len() {
            return None;
        }
        let index = self.index;
        self.index += 1;
        let item = ChunkItem {
            chunk: self.chunk,
            index,
        };
        Some(item)
    }
}

impl<'a> ChunkItem<'a> {
    pub fn chunk(&self) -> &'a Chunk {
        self.chunk
    }

    pub fn block(&self) -> Block {
        *self
            .chunk
            .list
            .get(self.index)
            .expect("should be valid index in chunk")
    }

    pub fn position_relative(&self) -> Coordinate {
        self.chunk.size.index_to_coordinate(self.index)
    }
    pub fn position_absolute(&self) -> Coordinate {
        self.position_relative() + self.chunk.origin
    }
}

impl ChunkSize {
    pub fn index_to_coordinate(&self, index: usize) -> Coordinate {
        let z = (index % self.z as usize) as i32;
        let xy = index / self.z as usize;
        let x = (xy % self.x as usize) as i32;
        let y = (xy / self.x as usize) as i32;
        Coordinate { x, y, z }
    }

    pub fn coordinate_to_index(&self, coordinate: Coordinate) -> usize {
        let [x, y, z] = [
            coordinate.x as usize,
            coordinate.y as usize,
            coordinate.z as usize,
        ];
        z + (x + y * self.x as usize) * self.z as usize
    }

    pub fn contains(&self, coordinate: Coordinate) -> bool {
        (0..self.x as i32).contains(&coordinate.x)
            && (0..self.y as i32).contains(&coordinate.y)
            && (0..self.z as i32).contains(&coordinate.z)
    }
}
