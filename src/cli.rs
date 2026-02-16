//! The CLI offers nice tools to launch, mod new game content and interact with existing saves.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    New,
    /// This command runs the engine and loads the main menu.
    Launch,
    /// This command runs a specific save.
    Run {
        world_name: String,
    },
}
