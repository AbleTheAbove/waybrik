// #![deny(missing_docs)]
//! The main game launcher.

pub mod engine;
pub mod lug;
fn main() {
    let _ = lug::init();
    let mut waye = engine::WayEngine::new();
    waye.new_world();
    waye.save();
    waye.run();
}
