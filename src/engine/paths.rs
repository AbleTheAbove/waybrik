//! paths for various places.

pub fn world_folder(world_name: String) -> String {
    let world_folder_path = format!("assets/worlds/{}", world_name);

    return world_folder_path;
}

pub fn world_file(world_name: String) -> String {
    let world_file_path = format!("{}/world.toml", world_name);

    return world_file_path;
}
