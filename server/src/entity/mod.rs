//! Provides several useful components, including `EntityComponent`
//! and `PlayerComponent`. In the future, will also
//! provide entity-specific components and systems.

mod broadcast;
mod chunk;
mod component;
mod destroy;
mod impls;
pub mod metadata;
mod movement;
mod save;

pub use impls::*;

use crate::systems::{
    BLOCK_FALLING_LANDING, CHUNK_CROSS, CHUNK_ENTITIES_LOAD, CHUNK_ENTITIES_UPDATE, CHUNK_SAVE,
    ENTITY_DESTROY, ENTITY_DESTROY_BROADCAST, ENTITY_METADATA_BROADCAST, ENTITY_MOVE_BROADCAST,
    ENTITY_PHYSICS, ENTITY_SPAWN_BROADCAST, ENTITY_VELOCITY_BROADCAST, ITEM_COLLECT, ITEM_MERGE,
    ITEM_SPAWN, JOIN_BROADCAST, SHOOT_ARROW,
};
pub use arrow::{ArrowComponent, ShootArrowEvent};
pub use broadcast::send_entity_to_player;
pub use broadcast::{EntitySendEvent, EntitySpawnEvent};
pub use chunk::ChunkEntities;
pub use chunk::ChunkEntityUpdateSystem;
pub use component::{
    NamedComponent, PacketCreatorComponent, PlayerComponent, PositionComponent,
    SerializerComponent, VelocityComponent,
};
pub use destroy::EntityDestroyEvent;
pub use falling_block::FallingBlockComponent;
pub use item::ItemComponent;
pub use metadata::{EntityBitMask, Metadata};
pub use movement::{degrees_to_stops, LastKnownPositionComponent};

pub use save::save_chunks;

use crate::entity::arrow::ShootArrowSystem;
use crate::entity::chunk::EntityChunkLoadSystem;
use crate::entity::destroy::EntityDestroyBroadcastSystem;
use crate::entity::falling_block::FallingBlockLandSystem;
use crate::entity::item::ItemCollectSystem;
use crate::entity::metadata::MetadataBroadcastSystem;
use crate::entity::save::ChunkSaveSystem;
use broadcast::EntityBroadcastSystem;
use component::ComponentResetSystem;
use destroy::EntityDestroySystem;
use item::{ItemMergeSystem, ItemSpawnSystem};
use movement::{EntityMoveBroadcastSystem, EntityVelocityBroadcastSystem};
use specs::DispatcherBuilder;

pub fn init_logic(dispatcher: &mut DispatcherBuilder) {
    dispatcher.add(ItemCollectSystem::default(), ITEM_COLLECT, &[]);
}

pub fn init_handlers(dispatcher: &mut DispatcherBuilder) {
    dispatcher.add(
        ChunkEntityUpdateSystem::default(),
        CHUNK_ENTITIES_UPDATE,
        &[],
    );
    dispatcher.add(EntityChunkLoadSystem::default(), CHUNK_ENTITIES_LOAD, &[]);
    dispatcher.add(EntityDestroySystem::default(), ENTITY_DESTROY, &[]);
    dispatcher.add(ItemSpawnSystem::default(), ITEM_SPAWN, &[]);
    dispatcher.add(ItemMergeSystem::default(), ITEM_MERGE, &[]);
    dispatcher.add(
        MetadataBroadcastSystem::default(),
        ENTITY_METADATA_BROADCAST,
        &[],
    );
    dispatcher.add(ShootArrowSystem::default(), SHOOT_ARROW, &[]);
    dispatcher.add(ChunkSaveSystem::default(), CHUNK_SAVE, &[]);
}

pub fn init_broadcast(dispatcher: &mut DispatcherBuilder) {
    dispatcher.add(
        EntityMoveBroadcastSystem::default(),
        ENTITY_MOVE_BROADCAST,
        &[],
    );
    dispatcher.add(
        EntityBroadcastSystem::default(),
        ENTITY_SPAWN_BROADCAST,
        &[JOIN_BROADCAST, CHUNK_CROSS],
    );
    dispatcher.add(
        EntityVelocityBroadcastSystem::default(),
        ENTITY_VELOCITY_BROADCAST,
        &[],
    );
    dispatcher.add(
        EntityDestroyBroadcastSystem::default(),
        ENTITY_DESTROY_BROADCAST,
        &[],
    );
    dispatcher.add(
        FallingBlockLandSystem::default(),
        BLOCK_FALLING_LANDING,
        &[ENTITY_PHYSICS],
    );
    dispatcher.add_thread_local(ComponentResetSystem);
}
