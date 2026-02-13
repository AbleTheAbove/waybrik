use log::{error, info, warn};

use crate::engine::{
    chunk::{Chunk, ChunkPos},
    materials::MaterialIndex,
    save::SaveFolder,
};
use std::collections::HashMap;

/// This structure represents an in memory chunk.
pub struct ChunkCache {
    cache: HashMap<ChunkPos, Chunk>,
}

#[derive(Debug)]
pub enum WorldError {
    WorldExists,
}

pub struct World {
    name: String,
    chunk_cache: ChunkCache,
    material_index: MaterialIndex,
}
impl World {
    /// Create a new world. Save it to disk also.
    pub fn new(name: String) -> Result<World, WorldError> {
        let material_index = MaterialIndex::default();
        let save_ret = SaveFolder::new(name.clone());
        match save_ret {
            Ok(_) => {
                info!("World creation worked.")
            }
            Err(_) => error!("World creation failed."),
        }
        Ok(Self {
            name,
            chunk_cache: ChunkCache {
                cache: HashMap::new(),
            },
            material_index,
        })
    }
    pub fn save(&mut self) {
        // warn!("Not yet implemented.")
    }
}
