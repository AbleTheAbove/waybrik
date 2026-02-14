use log::info;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
use toml::Table;

pub type MaterialId = u8;

#[derive(Serialize, Deserialize, Clone)]
pub struct MaterialIndex {
    index: HashMap<MaterialId, Material>,
}

impl std::fmt::Display for MaterialIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (mat_id, material) in self.index.iter() {
            write!(f, "MatID {} ", mat_id)?;
            writeln!(f, "Material {:?}", material)?;
        }

        Ok(())
    }
}

impl MaterialIndex {
    pub fn default() -> Self {
        let mut mat_idx = HashMap::new();

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
        mat_idx.insert(0, nil_material);
        mat_idx.insert(1, stone_material);
        mat_idx.insert(2, dirt_material);
        MaterialIndex { index: mat_idx }
    }

    pub fn load_core() -> Self {
        let file_path = "assets/addons/core/materials.toml";
        info!("Loading core addon");
        let contents = fs::read_to_string(file_path).unwrap();
        let mat_idx = toml_to_materials(contents);
        mat_idx
    }
    pub fn nil_material(&self) -> MaterialId {
        0
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Material {
    mat_name: String,
    id: MaterialId,
}

pub fn toml_to_materials(toml: String) -> MaterialIndex {
    let mut mats = MaterialIndex {
        index: HashMap::new(),
    };
    let decoded: Table = toml::from_str(&toml).unwrap();
    let materials = decoded.get("material").unwrap().as_table().unwrap();
    for (mat_name, val) in materials.iter() {
        let attributes = val.as_table().unwrap();
        let id = attributes.get("id").unwrap().as_integer().unwrap();

        let material = Material {
            mat_name: mat_name.to_string(),
            id: id as u8,
        };
        mats.index.insert(id as u8, material);
    }
    mats
}
