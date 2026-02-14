use std::collections::HashMap;

use crate::engine::asset_loader::shaders::{ShaderAssetType, ShaderBundle};
mod shaders;
pub struct AssetBundle {
    shaders: ShaderBundle,
}

fn load_shaders() -> ShaderBundle {
    let shaders = HashMap::new();

    shaders
}

pub fn load_assets() -> AssetBundle {
    AssetBundle {
        shaders: load_shaders(),
    }
}
