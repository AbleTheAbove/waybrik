use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
};

use log::{error, info};

use crate::engine::{paths, world::World};

pub struct SaveFolder {
    path: String,
}
pub enum SaveResult {
    GenericError,
}

impl SaveFolder {
    pub fn new(path: String) -> Result<Self, SaveResult> {
        let world_path = format!("assets/worlds/{}", path);
        let folder = fs::create_dir(&world_path);

        match folder {
            Ok(_) => return Ok(SaveFolder { path: path }),
            Err(err) => match err {
                _err => {
                    error!("{}", _err);
                    return Err(SaveResult::GenericError);
                }
            },
        }
    }
    /// Update the saved world with the in game one.
    pub fn update(path: String, world: &World) {
        let world_path = paths::world_file(path);
        info!("world path in Save::update {}", world_path);
        let mut file = OpenOptions::new()
            .write(true) // Enable writing
            .append(false) // Overwrite
            .create(true) // Create the file if it does not exist
            .open(world_path)
            .expect("Failed to open file with options");

        let contents = world_to_toml(world);

        file.write_all(contents.as_bytes())
            .expect("Failed to write to file");
    }
}

pub fn world_to_toml(world: &World) -> String {
    let decoded = toml::to_string(world).unwrap();
    decoded
}

pub fn toml_to_world(toml: String) -> World {
    let decoded = toml::from_str(&toml).unwrap();
    decoded
}
