use std::collections::HashMap;

pub type MaterialId = u8;
pub struct MaterialIndex {
    index: HashMap<MaterialId, Material>,
}

impl MaterialIndex {
    pub fn default() -> Self {
        let nil_material = Material { id: 0 };
        let stone_material = Material { id: 1 };
        let dirt_material = Material { id: 2 };

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

pub struct Material {
    id: MaterialId,
}
