#![forbid(unsafe_code)]

//! World generation for Feather.
//!
//! Generation is primarily based around the `ComposableGenerator`,
//! which allows configuration of a world generator pipeline.

use base::biome::BiomeList;
pub use superflat::SuperflatWorldGenerator;

use base::chunk::Chunk;
use base::world::{Sections, WorldHeight};
use base::ChunkPosition;
mod superflat;

pub trait WorldGenerator: Send + Sync {
    /// Generates the chunk at the given position.
    fn generate_chunk(
        &self,
        position: ChunkPosition,
        sections: Sections,
        min_y: i32,
        biomes: &BiomeList,
    ) -> Chunk;
}

pub struct EmptyWorldGenerator {}

impl WorldGenerator for EmptyWorldGenerator {
    fn generate_chunk(
        &self,
        position: ChunkPosition,
        sections: Sections,
        min_y: i32,
        _biomes: &BiomeList,
    ) -> Chunk {
        Chunk::new(position, sections, min_y / 16)
    }
}

/// Returns an index into a one-dimensional array
/// for the given x, y, and z values.
pub fn block_index(x: usize, y: i32, z: usize, world_height: WorldHeight, min_y: i32) -> usize {
    assert!(x < 16 && y >= min_y && y < min_y + *world_height as i32 && z < 16);
    (((y - min_y) as usize) << 8) | (x << 4) | z
}
