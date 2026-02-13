use std::{
    fs::{self, File},
    io::Write,
};

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
        let world_file_path = format!("{}/world.toml", world_path);
        let mut file = File::create("foo.txt").unwrap();
        file.write_all(b"Hello, world!").unwrap();

        match folder {
            Ok(_) => return Ok(SaveFolder { path: path }),
            Err(_) => return Err(SaveResult::GenericError),
        }
    }
}
