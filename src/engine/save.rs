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
        let world_name = format!("assets/worlds/{}", path);
        let folder = fs::create_dir(world_name);

        let mut file = File::create("foo.txt").unwrap();
        file.write_all(b"Hello, world!").unwrap();

        match folder {
            Ok(_) => return Ok(SaveFolder { path: path }),
            Err(_) => return Err(SaveResult::GenericError),
        }
    }
}
