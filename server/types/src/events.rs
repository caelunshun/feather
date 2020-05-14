use crate::Weather;
use feather_core::blocks::BlockId;
use feather_core::inventory::SlotIndex;
use feather_core::items::ItemStack;
use feather_core::util::{BlockPosition, ChunkPosition, ClientboundAnimation, Position};
use fecs::Entity;
use smallvec::SmallVec;

#[derive(Copy, Clone, Debug)]
pub struct BlockUpdateEvent {
    /// Position of the updated block
    pub pos: BlockPosition,
    /// Old block
    pub old: BlockId,
    /// New block
    pub new: BlockId,
    /// Cause of the block update.
    pub cause: BlockUpdateCause,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BlockUpdateCause {
    /// The update was caused by an entity performing
    /// a block break/placement. Usually a player.
    Entity(Entity),
    /// Unknown cause.
    Unknown,
}

/// Triggered directly _before_ an entity is removed from the world.
///
/// As such, components can still be accessed.
#[derive(Copy, Clone, Debug)]
pub struct EntityDespawnEvent {
    pub entity: Entity,
}

/// Triggered when a chunk is sent to a player.
#[derive(Copy, Clone, Debug)]
pub struct ChunkSendEvent {
    pub chunk: ChunkPosition,
    pub player: Entity,
}

/// Triggered right before a player joins the server.
#[derive(Copy, Clone, Debug)]
pub struct PlayerPreJoinEvent {
    pub player: Entity,
}

/// Triggered when a player joins the server.
#[derive(Copy, Clone, Debug)]
pub struct PlayerJoinEvent {
    pub player: Entity,
}

/// Triggered when a player leaves.
#[derive(Copy, Clone, Debug)]
pub struct PlayerLeaveEvent {
    pub player: Entity,
}

/// Triggered when an entity lands on the ground.
#[derive(Copy, Clone, Debug)]
pub struct EntityLandEvent {
    pub entity: Entity,
    /// Position where the entity landed.
    pub pos: Position,
}

/// Event triggered when an item is dropped.
///
/// Before this event is triggered, the item
/// is removed from the player's inventory.
#[derive(Debug, Clone)]
pub struct ItemDropEvent {
    /// The slot from which the item was dropped,
    /// if known.
    pub slot: Option<SlotIndex>,
    /// The item stack which was dropped.
    pub stack: ItemStack,
    /// The player who dropped the item.
    pub player: Entity,
}

/// Event triggered when an item is collected into an entity's
/// inventory.
///
/// Triggered before the item is deleted from the world.
#[derive(Debug, Clone)]
pub struct ItemCollectEvent {
    /// The item which was collected.
    pub item: Entity,
    /// The entity which collected the item.
    pub collector: Entity,
    /// Number of items which was collected.
    pub amount: u8,
}

/// Event which is triggered when a player
/// updates their inventory.
///
/// This event could also be triggered when the player
/// changes their held item.
#[derive(Debug, Clone)]
pub struct InventoryUpdateEvent {
    /// The slot(s) affected by the update.
    ///
    /// Multiple slots could be affected when, for
    /// example, a player uses the "drag" inventory interaction.
    pub slots: SmallVec<[SlotIndex; 2]>,
    /// The player owning the updated inventory.
    pub player: Entity,
}

/// Event triggered when an entity is created.
#[derive(Copy, Clone, Debug)]
pub struct EntitySpawnEvent {
    pub entity: Entity,
}

/// Event triggered when a player performs an animation (hits with their hand).
#[derive(Copy, Clone, Debug)]
pub struct PlayerAnimationEvent {
    pub player: Entity,
    pub animation: ClientboundAnimation,
}

/// Event triggered when a chat message is sent out
#[derive(Debug, Clone)]
pub struct ChatEvent {
    /// The JSON-formatted message
    pub message: String,
    /// The position of the message
    pub position: ChatPosition,
}

/// Different positions a chat message can be displayed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatPosition {
    /// Simple message displayed in the chat box
    Chat,
    /// System message displayed in the chat box
    SystemMessage,
    /// A text displayed above the hotbar
    GameInfo,
}

/// Event triggered when an entity crosses into a new chunk.
#[derive(Copy, Clone, Debug)]
pub struct ChunkCrossEvent {
    pub entity: Entity,
    pub old: Option<ChunkPosition>,
    pub new: ChunkPosition,
}

/// Event triggered when an entity is sent to a client.
///
/// This can be used to send additional packets along with the Spawn *
/// packet, such as entity metadata.
#[derive(Copy, Clone, Debug)]
pub struct EntitySendEvent {
    /// The entity which was sent.
    pub entity: Entity,
    /// The client to which the entity was sent.
    pub client: Entity,
}

/// Event triggered when an entity is destroyed on a client.
///
/// This can be used to clean up data. For example, the movement
/// broadcast system stores the last known position of all visible
/// entities for each client. It uses this event to remove
/// entries in that map.
#[derive(Copy, Clone, Debug)]
pub struct EntityClientRemoveEvent {
    /// The entity which was destroyed on the client.
    pub entity: Entity,
    /// The client on which the entity was destroyed.
    pub client: Entity,
}

/// Event triggered when a chunk is loaded.
#[derive(Copy, Clone, Debug)]
pub struct ChunkLoadEvent {
    pub chunk: ChunkPosition,
}

/// Event which is triggered when a chunk fails to load.
#[derive(Debug)]
pub struct ChunkLoadFailEvent {
    pub pos: ChunkPosition,
    pub error: anyhow::Error,
}

/// Event triggeered when a chunk is unloaded.
#[derive(Copy, Clone, Debug)]
pub struct ChunkUnloadEvent {
    pub chunk: ChunkPosition,
}

/// Event triggered when a chunk holder releases their hold on a chunk.
#[derive(Copy, Clone, Debug)]
pub struct ChunkHolderReleaseEvent {
    /// Entity which released their hold.
    pub entity: Entity,
    /// The chunk which was released.
    pub chunk: ChunkPosition,
}

/// Triggered when the weather changes.
#[derive(Copy, Clone, Debug)]
pub struct WeatherChangeEvent {
    pub from: Weather,
    pub to: Weather,
    pub duration: i32,
}

/// Requests that a chunk be held for the given client.
///
/// This is a "request"-type event: it has one handler defined
/// in the `chunk` crate which executes the request.
#[derive(Copy, Clone, Debug)]
pub struct HoldChunkRequest {
    pub player: Entity,
    pub chunk: ChunkPosition,
}

/// Requests that a chunk hold be removed for the given client.
#[derive(Copy, Clone, Debug)]
pub struct ReleaseChunkRequest {
    pub player: Entity,
    pub chunk: ChunkPosition,
}

/// Requests that a chunk be queued for loading.
#[derive(Copy, Clone, Debug)]
pub struct LoadChunkRequest {
    pub chunk: ChunkPosition,
}
