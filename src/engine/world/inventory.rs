//! An inventory is any thing that wishes to store items.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::engine::brick::InventoryID;

#[derive(Serialize, Deserialize, Clone)]
/// This structure represents an in memory chunk.
pub struct InventoryCache {
    pub cache: HashMap<InventoryID, Inventory>,
}
impl InventoryCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Inventory {
    pub items: Vec<u8>,
}
