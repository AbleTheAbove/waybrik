//! The waybrik engine.

use log::error;

use crate::engine::world::World;
pub mod brick;
pub mod chunk;
pub mod materials;
pub mod save;
pub mod world;

#[derive(PartialEq)]
pub enum EngineStage {
    MainMenu,
    InWorld,
}

/// The Engine core structure.
pub struct WayEngine {
    pub engine_stage: EngineStage,
    /// If no world is loaded we are in the main menu.
    pub world: Option<World>,
}
impl WayEngine {
    pub fn new() -> Self {
        Self {
            world: None,
            engine_stage: EngineStage::MainMenu,
        }
    }

    pub fn new_world(&mut self) {
        let w = World::new("test".to_string()).unwrap();
        self.world = Some(w);
    }

    pub fn run(&mut self) {
        loop {
            // Event loop start
            if self.engine_stage == EngineStage::InWorld && self.world.is_none() {
                // ERROR OUT HERE.
                error!("Engine stage is set to InGame but no world is loaded.");
            }
        }
    }
}
