//! The waybrik engine.

use log::{error, warn};

pub mod asset_loader;

use crate::engine::{
    asset_loader::{AssetBundle, load_assets},
    chunk::{Chunk, ChunkPos},
    renderer::Renderer,
    world::World,
};
pub mod brick;
pub mod chunk;
pub mod materials;
pub mod paths;
pub mod renderer;
pub mod save;
pub mod world;

#[derive(PartialEq, Clone, Copy)]
pub enum EngineStage {
    MainMenu,
    InWorld,
}

/// The Engine core structure.
pub struct WayEngine {
    pub engine_stage: EngineStage,
    /// If no world is loaded we are in the main menu.
    pub world: Option<World>,
    pub renderer: Renderer,
    pub assets: AssetBundle,
}
impl WayEngine {
    pub fn new() -> Self {
        let assets = load_assets();
        let renderer = renderer::Renderer::new();

        Self {
            world: None,
            renderer,
            engine_stage: EngineStage::MainMenu,
            assets,
        }
    }

    pub fn new_world(&mut self) {
        let mut w = World::new("test".to_string()).unwrap();
        let zero_chunk_pos = ChunkPos::new(0, 0, 0);
        let chunk = Chunk::new(zero_chunk_pos);
        w.chunk_cache.cache.insert(zero_chunk_pos, chunk);
        self.world = Some(w);
    }

    pub fn run(&mut self) {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 5);

        let window = video_subsystem
            .window("Waybrik", 900, 700)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        let gl_context = window.gl_create_context().unwrap();
        let gl = gl::load_with(|s| {
            video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
        });

        self.renderer.setup();

        let mut event_pump = sdl.event_pump().unwrap();
        'main: loop {
            // Event loop start
            if self.engine_stage == EngineStage::InWorld && self.world.is_none() {
                // ERROR OUT HERE.
                error!("Engine stage is set to InGame but no world is loaded.");
            }

            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'main,
                    _ => {}
                }
            }

            // render window contents here
            self.renderer.render();
            // flip window buffer
            window.gl_swap_window();
        }

        self.renderer.cleanup();
    }

    pub fn save(&mut self) {
        let mut world_clone = self.world.clone().unwrap();
        world_clone.save();
    }
}
