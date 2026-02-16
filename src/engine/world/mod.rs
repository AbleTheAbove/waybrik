use log::{error, info, warn};

use crate::engine::{
    brick::InventoryID,
    chunk::{Chunk, ChunkPos},
    materials::MaterialIndex,
    object::{Object, ObjectCache, ObjectID},
    paths,
    save::SaveFolder,
    world::inventory::{Inventory, InventoryCache},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod inventory;

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
    #[serde(skip_serializing)]
    pub inventory_cache: InventoryCache,
    #[serde(skip_serializing)]
    pub object_cache: ObjectCache,
}
impl World {
    /// Create a new world. Save it to disk also.
    pub fn new(name: String) -> Result<World, WorldError> {
        let material_index = MaterialIndex::load_core();
        println!("{}", material_index);
        let save_ret = SaveFolder::new(name.clone());
        match save_ret {
            Ok(_) => info!("World creation worked."),
            Err(_) => error!("World creation failed."),
        }
        let mut inv_cache = InventoryCache::new();
        let inv = Inventory { items: vec![] };
        inv_cache.cache.insert(InventoryID { id: 0 }, inv);

        let mut objects = ObjectCache::new();
        let object_a = Object::spawn_from_script("mud_mixer".to_string()).unwrap();
        objects.cache.insert(ObjectID { id: 10 }, object_a);

        Ok(Self {
            name,
            chunk_cache: ChunkCache {
                cache: HashMap::new(),
            },
            material_index,
            inventory_cache: inv_cache,
            object_cache: objects,
        })
    }
    pub fn save(&mut self) {
        let save_path = paths::world_folder(self.name.clone());
        info!("save path in World::save {}", save_path);
        SaveFolder::update(save_path, self);
    }
    /// Run the GameTick once.
    pub fn tick(&mut self) -> &mut Self {
        for (chunk_pos, chunk) in self.chunk_cache.cache.iter_mut() {
            if chunk.dirty.is_mesh_dirty {
                // rebuild mesh here.
            }
            if chunk.dirty.is_save_dirty {
                // save here.
            }
        }

        for (object_id, object) in self.object_cache.cache.iter_mut() {
            let object_ret = object.fire_tick();
            match object_ret {
                Ok(object_ret_a) => {
                    for game_event in object_ret_a {
                        // println!("Handling event {:?}.", object_ret_a);
                        match game_event.event_type {
                            crate::engine::game_events::GameEventType::Nil => {
                                println!("A nil event fired.")
                            }
                            crate::engine::game_events::GameEventType::NewInventory {
                                inventory_size,
                            } => {
                                println!("Spawning inventory");

                                let inven = Inventory { items: vec![] };
                                self.inventory_cache
                                    .cache
                                    .insert(InventoryID { id: 0 }, inven);
                            }
                        }
                    }
                }
                Err(err) => {
                    use mlua::Error::FromLuaConversionError;
                    match err {
                        FromLuaConversionError { from, to, message } => {}
                        _ => {
                            println!("{}", err)
                        }
                    }
                }
            }
        }
        self
    }
}
