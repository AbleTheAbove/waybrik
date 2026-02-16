use crate::engine::materials::MaterialId;
use serde::{Deserialize, Serialize};
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct InventoryID {
    pub id: u64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]

/// A brick.
pub struct Brick {
    pub material: MaterialId,
    pub inventory_id: Option<InventoryID>,
}
