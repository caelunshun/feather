//! Implements block change broadcasting.
//!
//! # Bulk updates
//! The protocol provides three methods to change blocks
//! on the client:
//! * The `BlockChange` packet to update a single block.
//! * The `MultiBlockChange` packet to update multiple blocks
//! within a single chunk section.
//! * The `ChunkData` packet to overwrite entire chunk sections
//! at once.
//!
//! Feather is optimized for bulk block updates to cater to plugins
//! like WorldEdit. This module chooses the optimal packet from
//! the above three options to achieve ideal performance.

use base::{chunk::SECTION_VOLUME, position, CHUNK_WIDTH};
use common::world::Dimensions;
use common::{events::BlockChangeEvent, Game};
use ecs::{SysResult, SystemExecutor};

use crate::Server;

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems
        .group::<Server>()
        .add_system(broadcast_block_changes);
}

fn broadcast_block_changes(game: &mut Game, server: &mut Server) -> SysResult {
    for (_, event) in game.ecs.query::<&BlockChangeEvent>().iter() {
        broadcast_block_change(event, game, server);
    }
    Ok(())
}

/// Threshold at which to switch from block change to chunk
// overwrite packets.
const CHUNK_OVERWRITE_THRESHOLD: usize = SECTION_VOLUME / 2;

fn broadcast_block_change(event: &BlockChangeEvent, game: &Game, server: &mut Server) {
    if event.count() >= CHUNK_OVERWRITE_THRESHOLD {
        broadcast_block_change_chunk_overwrite(event, game, server);
    } else {
        broadcast_block_change_simple(event, game, server);
    }
}

fn broadcast_block_change_chunk_overwrite(
    event: &BlockChangeEvent,
    game: &Game,
    server: &mut Server,
) {
    let mut query = game.ecs.query::<&Dimensions>();
    let dimension = query
        .iter()
        .find(|(world, _)| *world == *event.world())
        .unwrap()
        .1
        .get(&**event.dimension())
        .unwrap();
    for (chunk_pos, _, _) in event.iter_affected_chunk_sections() {
        if let Some(chunk) = dimension.chunk_map().chunk_handle_at(chunk_pos) {
            let position = position!(
                (chunk_pos.x * CHUNK_WIDTH as i32) as f64,
                0.0,
                (chunk_pos.z * CHUNK_WIDTH as i32) as f64,
            );
            server.broadcast_nearby_with(event.world(), event.dimension(), position, |client| {
                client.overwrite_chunk(&chunk);
            })
        }
    }
}

fn broadcast_block_change_simple(event: &BlockChangeEvent, game: &Game, server: &mut Server) {
    let mut query = game.ecs.query::<&Dimensions>();
    let dimension = query
        .iter()
        .find(|(world, _)| *world == *event.world())
        .unwrap()
        .1
        .get(&**event.dimension())
        .unwrap();
    for pos in event.iter_changed_blocks() {
        let new_block = dimension.block_at(pos);
        if let Some(new_block) = new_block {
            server.broadcast_nearby_with(
                event.world(),
                event.dimension(),
                pos.position(),
                |client| client.send_block_change(pos, new_block),
            );
        }
    }
}
