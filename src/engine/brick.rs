use crate::engine::materials::MaterialId;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// A brick.
pub struct Brick {
    pub material: MaterialId,
}
