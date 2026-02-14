use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type MaterialId = u8;

#[derive(Serialize, Deserialize, Clone)]
pub struct MaterialIndex {
    index: HashMap<MaterialId, Material>,
}

impl MaterialIndex {
    pub fn default() -> Self {
        let nil_material = Material {
            id: 0,
            mat_name: "Nil".to_string(),
        };
        let stone_material = Material {
            id: 1,
            mat_name: "Dirt".to_string(),
        };
        let dirt_material = Material {
            id: 2,
            mat_name: "Dirt".to_string(),
        };

        let mut mat_idx = HashMap::new();
        mat_idx.insert(0, nil_material);
        mat_idx.insert(1, stone_material);
        mat_idx.insert(2, dirt_material);

        Self { index: mat_idx }
    }
    pub fn nil_material(&self) -> MaterialId {
        0
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Material {
    mat_name: String,
    id: MaterialId,
}
