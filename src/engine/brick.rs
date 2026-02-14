use crate::engine::materials::MaterialId;
use serde::Serialize;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, serde::Deserialize)]

/// A brick.
pub struct Brick {
    pub material: MaterialId,
}
