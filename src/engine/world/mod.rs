use log::{error, info, warn};

use crate::engine::{
    chunk::{Chunk, ChunkPos},
    materials::MaterialIndex,
    paths,
    save::SaveFolder,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Serialize, Deserialize, Clone)]
/// This structure represents an in memory chunk.
pub struct ChunkCache {
    pub cache: HashMap<ChunkPos, Chunk>,
}

#[derive(Debug, Clone)]
pub enum WorldError {
    WorldExists,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct World {
    pub name: String,
    pub material_index: MaterialIndex,
    /// Skip the chunk cache when saving to disk.
    /// The chunk cache will be populated based on player locations.
    #[serde(skip_serializing)]
    pub chunk_cache: ChunkCache,
}
impl World {
    /// Create a new world. Save it to disk also.
    pub fn new(name: String) -> Result<World, WorldError> {
        let material_index = MaterialIndex::load_core();
        println!("{}", material_index);
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
        let save_path = paths::world_folder(self.name.clone());
        info!("save path in World::save {}", save_path);
        SaveFolder::update(save_path, self);
    }
    /// Run the GameTick once.
    pub fn tick(&mut self) {
        for (chunk_pos, chunk) in self.chunk_cache.cache.iter_mut() {
            if chunk.dirty.is_mesh_dirty {
                // rebuild mesh here.
            }
            if chunk.dirty.is_save_dirty {
                // save here.
            }
        }
    }
}
