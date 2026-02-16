// #![deny(missing_docs)]
//! The main game launcher.

use clap::Parser;

use crate::cli::Commands;

pub mod cli;
pub mod engine;
pub mod lug;
fn main() {
    let _ = lug::init();

    let cli = cli::Cli::parse();

    let mut waye = engine::WayEngine::new();
    match &cli.command {
        Commands::New => {
            waye.new_world();
            waye.save();
        }
        Commands::Launch => {
            panic!("Not finished")
        }
        Commands::Run { world_name } => {
            waye.new_world();
            // TODO: Implement world loading.
            // waye.load_world();
            waye.save();
            waye.run();
        }
    }
}
