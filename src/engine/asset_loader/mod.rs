use std::collections::HashMap;

pub enum ShaderAssetType {
    Fragment,
    Vertex,
}

pub struct ShaderAsset {
    asset_name: String,
    asset_type: ShaderAssetType,
}

pub struct AssetBundle {
    shaders: HashMap<String, ShaderAssetType>,
}
