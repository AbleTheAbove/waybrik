use crate::engine::brick::Brick;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

/// A chunk position. using i32 to avoid floating point drift.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    /// x coordinate.
    pub x: i32,
    /// y coordinate.
    pub y: i32,
    /// z coordinate.
    pub z: i32,
}

pub struct Chunk {
    pub position: ChunkPos,
    bricks: [Brick; CHUNK_VOLUME],
}
impl Chunk {
    pub fn new(position: ChunkPos) -> Self {
        Self {
            position,
            bricks: [Brick { material: 0 }; CHUNK_VOLUME],
        }
    }
    #[inline]
    fn index(x: usize, y: usize, z: usize) -> usize {
        x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE
    }
    pub fn get_brick(&self, x: usize, y: usize, z: usize) -> Brick {
        self.bricks[Self::index(x, y, z)]
    }

    pub fn set_brick(&mut self, x: usize, y: usize, z: usize, brick: Brick) {
        let idx = Self::index(x, y, z);
        self.bricks[idx] = brick;
    }
}
