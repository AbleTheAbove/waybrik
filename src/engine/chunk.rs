use serde::{Deserialize, Serialize};

use crate::engine::brick::Brick;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

/// A chunk position. using i32 to avoid floating point drift.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, serde::Deserialize)]

pub struct ChunkPos {
    /// x coordinate.
    pub x: i32,
    /// y coordinate.
    pub y: i32,
    /// z coordinate.
    pub z: i32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ChunkDirtyFlags {
    /// If the mesh is outdated and needs a rebuild flip this.
    pub is_mesh_dirty: bool,
    /// If the chunk is updated and needs saving flip this.
    pub is_save_dirty: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Chunk {
    pub dirty: ChunkDirtyFlags,
    pub position: ChunkPos,
    bricks: Vec<Brick>,
}
impl Chunk {
    pub fn new(position: ChunkPos) -> Self {
        let mut chunk_bricks = vec![];
        for _brick_chunk_idx in 0..CHUNK_VOLUME {
            let brick = Brick { material: 0 };
            chunk_bricks.push(brick);
        }
        let dirty_flags = ChunkDirtyFlags {
            is_mesh_dirty: true,
            is_save_dirty: true,
        };
        Self {
            position,
            bricks: chunk_bricks,
            dirty: dirty_flags,
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
