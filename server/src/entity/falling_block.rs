use feather_blocks::Block;
use specs::{Component, DenseVecStorage};

/// Component for falling block entities.
pub struct FallingBlockComponent {
    pub block: Block,
}

impl Default for FallingBlockComponent {
    fn default() -> Self {
        FallingBlockComponent {
            block: Block::Stone,
        }
    }
}

impl Component for FallingBlockComponent {
    type Storage = DenseVecStorage<Self>;
}
