use std::collections::HashMap;

pub type ShaderBundle = HashMap<String, ShaderAssetType>;

pub enum ShaderAssetType {
    Fragment,
    Vertex,
}

pub struct ShaderAsset {
    asset_name: String,
    asset_type: ShaderAssetType,
}
