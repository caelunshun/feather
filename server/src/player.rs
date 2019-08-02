//! This module provides systems and components
//! relating to players, including player movement
//! and inventory handling.

use std::ops::{Deref, DerefMut};

use hashbrown::HashSet;
use rayon::prelude::*;
use shrev::EventChannel;
use specs::storage::BTreeStorage;
use specs::{
    Component, Entities, Entity, Join, LazyUpdate, ParJoin, Read, ReadStorage, ReaderId, System,
    World, WorldExt, Write, WriteStorage,
};

use feather_core::network::cast_packet;
use feather_core::network::packet::implementation::{
    ChunkData, PlayerInfo, PlayerInfoAction, PlayerLook, PlayerPosition,
    PlayerPositionAndLookServerbound, SpawnPlayer, UnloadChunk,
};
use feather_core::network::packet::{Packet, PacketType};
use feather_core::world::chunk::Chunk;
use feather_core::world::{ChunkMap, ChunkPosition, Position};
use feather_core::Gamemode;

use crate::chunk_logic::{
    load_chunk, ChunkHolderComponent, ChunkHolderReleaseEvent, ChunkHolders, ChunkLoadEvent,
    ChunkWorkerHandle,
};
use crate::config::Config;
use crate::entity::{broadcast_entity_movement, EntityComponent, PlayerComponent};
use crate::joinhandler::{PlayerJoinEvent, SPAWN_POSITION};
use crate::network::{send_packet_to_player, NetworkComponent, PacketQueue, PlayerPreJoinEvent};
use crate::{TickCount, TPS};
use feather_core::entitymeta::{EntityMetadata, MetaEntry};
use specs::SystemData;
use std::collections::VecDeque;
use std::sync::Arc;
use uuid::Uuid;

/// Event which is called when a player disconnected.
pub struct PlayerDisconnectEvent {
    pub player: Entity,
    pub reason: String,
    pub uuid: Uuid,
}

/// Event which is called when a player moves.
pub struct PlayerMoveEvent {
    pub player: Entity,
    pub old_pos: Position,
    pub new_pos: Position,
}

/// System for handling player movement
/// packets.
pub struct PlayerMovementSystem;

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'a, EntityComponent>,
        ReadStorage<'a, PlayerComponent>,
        Read<'a, PacketQueue>,
        ReadStorage<'a, NetworkComponent>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Write<'a, EventChannel<PlayerMoveEvent>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut ecomps, pcomps, packet_queue, netcomps, entities, _, mut move_events) = data;

        // Take movement packets
        let mut packets = vec![];
        packets.append(&mut packet_queue.for_packet(PacketType::PlayerPosition));
        packets.append(&mut packet_queue.for_packet(PacketType::PlayerPositionAndLookServerbound));
        packets.append(&mut packet_queue.for_packet(PacketType::PlayerLook));

        // Handle movement packets
        for (player, packet) in packets {
            let ecomp = ecomps.get(player).unwrap();

            // Get position using packet and old position
            let (new_pos, has_moved, has_looked) = new_pos_from_packet(ecomp.position, packet);

            // Broadcast position update
            broadcast_entity_movement(
                player,
                ecomp.position,
                new_pos,
                has_moved,
                has_looked,
                &netcomps,
                &pcomps,
                &entities,
            );

            // Trigger player move event
            // TODO entity move event as well
            let event = PlayerMoveEvent {
                player,
                old_pos: ecomp.position,
                new_pos,
            };
            move_events.single_write(event);

            // Set new position
            ecomps.get_mut(player).unwrap().position = new_pos;
        }
    }
}

fn new_pos_from_packet(old_pos: Position, packet: Box<dyn Packet>) -> (Position, bool, bool) {
    let mut has_looked = false;
    let mut has_moved = false;

    let pos = match packet.ty() {
        PacketType::PlayerPosition => {
            has_moved = true;
            let packet = cast_packet::<PlayerPosition>(&*packet);

            Position::new(
                packet.x,
                packet.feet_y,
                packet.z,
                old_pos.pitch,
                old_pos.yaw,
            )
        }
        PacketType::PlayerLook => {
            has_looked = true;
            let packet = cast_packet::<PlayerLook>(&*packet);

            Position::new(old_pos.x, old_pos.y, old_pos.z, packet.pitch, packet.yaw)
        }
        PacketType::PlayerPositionAndLookServerbound => {
            has_moved = true;
            has_looked = true;
            let packet = cast_packet::<PlayerPositionAndLookServerbound>(&*packet);

            Position::new(packet.x, packet.feet_y, packet.z, packet.pitch, packet.yaw)
        }
        _ => panic!(),
    };

    (pos, has_moved, has_looked)
}

/// Component storing what chunks are pending
/// to send to a player.
#[derive(Clone, Debug)]
pub struct ChunkPendingComponent {
    pub pending: HashSet<ChunkPosition>,
}

impl Deref for ChunkPendingComponent {
    type Target = HashSet<ChunkPosition>;

    fn deref(&self) -> &Self::Target {
        &self.pending
    }
}

impl DerefMut for ChunkPendingComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pending
    }
}

impl Component for ChunkPendingComponent {
    type Storage = BTreeStorage<Self>;
}

/// System for initializing the necessary components
/// when a player joins.
#[derive(Default)]
pub struct PlayerInitSystem {
    join_event_reader: Option<ReaderId<PlayerPreJoinEvent>>,
}

impl PlayerInitSystem {
    pub fn new() -> Self {
        Self {
            join_event_reader: None,
        }
    }
}

impl<'a> System<'a> for PlayerInitSystem {
    type SystemData = (
        Read<'a, EventChannel<PlayerPreJoinEvent>>,
        WriteStorage<'a, PlayerComponent>,
        WriteStorage<'a, EntityComponent>,
        WriteStorage<'a, ChunkPendingComponent>,
        WriteStorage<'a, LoadedChunksComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            events,
            mut player_comps,
            mut entity_comps,
            mut chunk_pending_comps,
            mut loaded_chunk_comps,
        ) = data;

        // Run through events
        for event in events.read(&mut self.join_event_reader.as_mut().unwrap()) {
            let player_comp = PlayerComponent {
                profile_properties: event.profile_properties.clone(),
                gamemode: Gamemode::Creative,
            };
            player_comps.insert(event.player, player_comp).unwrap();

            let entity_comp = EntityComponent {
                uuid: event.uuid,
                display_name: event.username.clone(),
                position: SPAWN_POSITION,
                on_ground: true,
            };
            entity_comps.insert(event.player, entity_comp).unwrap();

            let chunk_pending_comp = ChunkPendingComponent {
                pending: HashSet::new(),
            };
            chunk_pending_comps
                .insert(event.player, chunk_pending_comp)
                .unwrap();

            let loaded_chunk_comp = LoadedChunksComponent::default();
            loaded_chunk_comps
                .insert(event.player, loaded_chunk_comp)
                .unwrap();
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.join_event_reader = Some(
            world
                .fetch_mut::<EventChannel<PlayerPreJoinEvent>>()
                .register_reader(),
        );
    }
}

/// System for sending chunks to players once they're loaded.
///
/// This system listens to `ChunkLoadEvent`s.
#[derive(Default)]
pub struct ChunkSendSystem {
    load_event_reader: Option<ReaderId<ChunkLoadEvent>>,
}

impl ChunkSendSystem {
    pub fn new() -> Self {
        Self {
            load_event_reader: None,
        }
    }
}

impl<'a> System<'a> for ChunkSendSystem {
    type SystemData = (
        WriteStorage<'a, ChunkPendingComponent>,
        ReadStorage<'a, NetworkComponent>,
        Read<'a, ChunkMap>,
        Read<'a, EventChannel<ChunkLoadEvent>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut pendings, netcomps, chunk_map, load_events) = data;

        for event in load_events.read(&mut self.load_event_reader.as_mut().unwrap()) {
            // TODO perhaps this is slightly inefficient?
            (&netcomps, &mut pendings)
                .par_join()
                .for_each(|(net, pending)| {
                    if pending.contains(&event.pos) {
                        // It's safe to unwrap the chunk value now,
                        // because we know it's been loaded.
                        let chunk = chunk_map.chunk_at(event.pos).unwrap();
                        send_chunk_data(chunk, net);

                        pending.remove(&event.pos);
                    }
                });
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.load_event_reader = Some(
            world
                .fetch_mut::<EventChannel<ChunkLoadEvent>>()
                .register_reader(),
        );
    }
}

/// Component for storing which chunks a client
/// has loaded and which are queued to be unloaded
/// on the client.
#[derive(Clone, Default, Debug)]
pub struct LoadedChunksComponent {
    /// All chunks which are loaded on the client, i.e.
    /// which have had a Chunk Data packet sent.
    loaded_chunks: HashSet<ChunkPosition>,
    /// Chunks queued for unloading on the client.
    ///
    /// Note that that these chunks will not be unloaded
    /// on the server - all that will happen is that an Unload
    /// Chunk packet will be sent to the client. This avoids client-side
    /// memory leaks.
    unload_queue: VecDeque<(ChunkPosition, u64)>,
}

impl Component for LoadedChunksComponent {
    type Storage = BTreeStorage<Self>;
}

/// Time after a player can no longer see a chunk
/// that it is unloaded.
const CHUNK_UNLOAD_TIME: u64 = TPS * 5; // 5 seconds

/// System that checks when a player crosses chunk boundaries.
/// When the player does so, the system sends Chunk Data packets
/// for chunks within the view distance and also unloads
/// chunks no longer within the player's view distance.
#[derive(Default)]
pub struct ChunkCrossSystem {
    reader: Option<ReaderId<PlayerMoveEvent>>,
}

impl<'a> System<'a> for ChunkCrossSystem {
    type SystemData = (
        Read<'a, EventChannel<PlayerMoveEvent>>,
        Read<'a, ChunkMap>,
        Read<'a, TickCount>,
        Read<'a, Arc<Config>>,
        WriteStorage<'a, LoadedChunksComponent>,
        WriteStorage<'a, ChunkHolderComponent>,
        ReadStorage<'a, NetworkComponent>,
        Write<'a, ChunkHolders>,
        Read<'a, ChunkWorkerHandle>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            events,
            chunk_map,
            tick_count,
            config,
            mut loaded_chunks_comps,
            mut chunk_holder_comps,
            net_comps,
            mut holders,
            chunk_handle,
            lazy,
        ) = data;

        // Go through events and handle them accordingly
        for event in events.read(&mut self.reader.as_mut().unwrap()) {
            let old_chunk_pos = event.old_pos.chunk_pos();
            let new_chunk_pos = event.new_pos.chunk_pos();

            if old_chunk_pos != new_chunk_pos {
                // Player has moved across chunk boundaries. Handle accordingly.
                let net = net_comps.get(event.player).unwrap();
                let chunk_holder = chunk_holder_comps.get_mut(event.player).unwrap();
                let loaded_chunks = loaded_chunks_comps.get_mut(event.player).unwrap();

                let chunks = chunks_within_view_distance(&config, new_chunk_pos);

                for chunk in &chunks {
                    if loaded_chunks.loaded_chunks.contains(chunk) {
                        // Already sent - nothing to do.
                        continue;
                    }

                    send_chunk_to_player(
                        *chunk,
                        net,
                        event.player,
                        &chunk_map,
                        &chunk_handle,
                        &mut holders,
                        chunk_holder,
                        loaded_chunks,
                        &lazy,
                    );
                }

                // Now, queue all chunks which need to be unloaded for unloading.
                let old_chunks = chunks_within_view_distance(&config, old_chunk_pos);

                for chunk in old_chunks {
                    if chunks.contains(&chunk) {
                        // Chunk should remain loaded. Nothing to do
                        continue;
                    }

                    // Queue chunk for unloading.
                    let time = tick_count.0 + CHUNK_UNLOAD_TIME;
                    loaded_chunks.unload_queue.push_back((chunk, time));
                }
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader = Some(
            world
                .fetch_mut::<EventChannel<PlayerMoveEvent>>()
                .register_reader(),
        );
    }
}

/// System for sending the Unload Chunk packet when the time comes.
pub struct ClientChunkUnloadSystem;

impl<'a> System<'a> for ClientChunkUnloadSystem {
    type SystemData = (
        WriteStorage<'a, LoadedChunksComponent>,
        ReadStorage<'a, NetworkComponent>,
        ReadStorage<'a, EntityComponent>,
        WriteStorage<'a, ChunkHolderComponent>,
        Entities<'a>,
        Write<'a, ChunkHolders>,
        Read<'a, TickCount>,
        Read<'a, Arc<Config>>,
        Write<'a, EventChannel<ChunkHolderReleaseEvent>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut loaded_chunks_comps,
            net_comps,
            entity_comps,
            mut chunk_holder_comps,
            entities,
            mut chunk_holders,
            tick_count,
            config,
            mut holder_release_events,
        ) = data;

        (
            &mut loaded_chunks_comps,
            &net_comps,
            &entity_comps,
            &mut chunk_holder_comps,
            &entities,
        )
            .join()
            .for_each(
                |(loaded_chunks_comp, net_comp, entity_comp, chunk_holder_comp, player)| {
                    // Go through queue and see if it's time to unload any chunks.
                    while let Some((chunk, time)) = loaded_chunks_comp.unload_queue.front() {
                        let chunk = *chunk;
                        if tick_count.0 >= *time {
                            // Unload if needed.

                            let chunks_within_view_distance = chunks_within_view_distance(
                                &config,
                                entity_comp.position.chunk_pos(),
                            );

                            if chunks_within_view_distance.contains(&chunk) {
                                // Chunk is within view distance again - don't unload it.
                                loaded_chunks_comp.unload_queue.pop_front();
                                continue;
                            }

                            let unload_chunk = UnloadChunk::new(chunk.x, chunk.z);
                            send_packet_to_player(net_comp, unload_chunk);

                            // Remove chunk from queue.
                            loaded_chunks_comp.unload_queue.pop_front();
                            // Remove from loaded chunk list.
                            loaded_chunks_comp.loaded_chunks.remove(&chunk);
                            // Remove hold on chunk so it can be unloaded.
                            chunk_holders.remove_holder(chunk, player, &mut holder_release_events);
                            // Remove hold from chunk holder component.
                            chunk_holder_comp.holds.remove(&chunk);
                        } else {
                            // No more chunks in queue that should
                            // be unloaded - finished.
                            break;
                        }
                    }
                },
            );
    }
}

/// Returns the set of all chunk positions
/// within the server view distance of a given
/// chunk.
fn chunks_within_view_distance(config: &Config, chunk: ChunkPosition) -> HashSet<ChunkPosition> {
    let view_distance = i32::from(config.server.view_distance);
    let mut results = HashSet::with_capacity((view_distance * view_distance) as usize);

    for x in -view_distance..=view_distance {
        for z in -view_distance..=view_distance {
            results.insert(ChunkPosition::new(chunk.x + x, chunk.z + z));
        }
    }

    results
}

/// System for broadcasting when a player joins
/// the game. Also spawns other players to
/// the player's client.
#[derive(Default)]
pub struct JoinBroadcastSystem {
    reader: Option<ReaderId<PlayerJoinEvent>>,
}

impl JoinBroadcastSystem {
    pub fn new() -> Self {
        Self { reader: None }
    }
}

impl<'a> System<'a> for JoinBroadcastSystem {
    type SystemData = (
        Read<'a, EventChannel<PlayerJoinEvent>>,
        ReadStorage<'a, EntityComponent>,
        ReadStorage<'a, PlayerComponent>,
        ReadStorage<'a, NetworkComponent>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (join_events, entity_comps, player_comps, net_comps, entities) = data;

        for event in join_events.read(&mut self.reader.as_mut().unwrap()) {
            // Broadcast join
            let entity_comp = entity_comps.get(event.player).unwrap();
            let player_comp = player_comps.get(event.player).unwrap();

            let (player_info, spawn_player) =
                get_player_initialization_packets(entity_comp, player_comp, event.player);

            for (player, net) in (&entities, &net_comps).join() {
                // Send player info to the player who joined
                // so they can see themselves in the tablist,
                // but don't send spawn player.
                send_packet_to_player(net, player_info.clone());
                if player != event.player {
                    send_packet_to_player(net, spawn_player.clone());
                }
            }

            let net_comp = net_comps.get(event.player).unwrap();

            // Send existing players to new player
            for (entity_comp, player_comp, entity) in
                (&entity_comps, &player_comps, &entities).join()
            {
                if entity != event.player {
                    let (player_info, spawn_player) =
                        get_player_initialization_packets(entity_comp, player_comp, entity);
                    send_packet_to_player(net_comp, player_info);
                    send_packet_to_player(net_comp, spawn_player);
                }
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader = Some(
            world
                .fetch_mut::<EventChannel<PlayerJoinEvent>>()
                .register_reader(),
        );
    }
}

/// Returns the player info and spawn player packets
/// for the given player.
fn get_player_initialization_packets(
    ecomp: &EntityComponent,
    pcomp: &PlayerComponent,
    player: Entity,
) -> (PlayerInfo, SpawnPlayer) {
    let display_name = json!({
        "text": ecomp.display_name
    })
    .to_string();

    let mut props = vec![];
    for prop in pcomp.profile_properties.iter() {
        props.push((
            prop.name.clone(),
            prop.value.clone(),
            prop.signature.clone(),
        ));
    }

    let action = PlayerInfoAction::AddPlayer(
        ecomp.display_name.clone(),
        props,
        Gamemode::Creative,
        50,
        display_name,
    );
    let player_info = PlayerInfo::new(action, ecomp.uuid);

    let metadata = EntityMetadata::new().with(&[
        (0, MetaEntry::Byte(0)),
        (1, MetaEntry::VarInt(300)),
        (2, MetaEntry::OptChat(None)),
        (3, MetaEntry::Boolean(false)),
        (4, MetaEntry::Boolean(false)),
        (5, MetaEntry::Boolean(false)),
        (6, MetaEntry::Byte(0)),
        (7, MetaEntry::Float(1.0)),
        (8, MetaEntry::VarInt(0)),
        (9, MetaEntry::Boolean(false)),
        (10, MetaEntry::VarInt(0)),
        (11, MetaEntry::Float(0.0)),
        (12, MetaEntry::VarInt(0)),
        (13, MetaEntry::Byte(0)),
        (14, MetaEntry::Byte(1)),
        // TODO NBT
    ]);

    let spawn_player = SpawnPlayer::new(
        player.id() as i32,
        ecomp.uuid,
        ecomp.position.x,
        ecomp.position.y,
        ecomp.position.z,
        degrees_to_stops(ecomp.position.pitch),
        degrees_to_stops(ecomp.position.yaw),
        metadata,
    );

    (player_info, spawn_player)
}

/// System for broadcasting when a player disconnects.
#[derive(Default)]
pub struct DisconnectBroadcastSystem {
    reader: Option<ReaderId<PlayerDisconnectEvent>>,
}

impl DisconnectBroadcastSystem {
    pub fn new() -> Self {
        Self { reader: None }
    }
}

impl<'a> System<'a> for DisconnectBroadcastSystem {
    type SystemData = (
        Read<'a, EventChannel<PlayerDisconnectEvent>>,
        ReadStorage<'a, NetworkComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (events, net_comps) = data;

        for event in events.read(&mut self.reader.as_mut().unwrap()) {
            // Broadcast disconnect.
            // Note that the Destroy Entity packet is sent
            // in a separate system (crate::entity::EntityDestroyBroadcastSystem).
            // This system only updates the tablist for all clients.
            let player_info = PlayerInfo::new(PlayerInfoAction::RemovePlayer, event.uuid);

            for net in net_comps.join() {
                send_packet_to_player(net, player_info.clone());
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader = Some(
            world
                .fetch_mut::<EventChannel<PlayerDisconnectEvent>>()
                .register_reader(),
        );
    }
}

fn send_chunk_data(chunk: &Chunk, net: &NetworkComponent) {
    let packet = ChunkData::new(chunk.clone());
    send_packet_to_player(net, packet);
}

/// Attempts to send the chunk at the given position to
/// the given player. If the chunk is not loaded, it will
/// be loaded and sent at a later time as soon as it is
/// loaded.
#[allow(clippy::too_many_arguments)] // TODO: get rid of LoadedChunksComponent
pub fn send_chunk_to_player(
    chunk_pos: ChunkPosition,
    net: &NetworkComponent,
    player: Entity,
    chunk_map: &ChunkMap,
    chunk_handle: &ChunkWorkerHandle,
    holders: &mut ChunkHolders,
    holder: &mut ChunkHolderComponent,
    loaded_chunks: &mut LoadedChunksComponent,
    lazy: &LazyUpdate,
) {
    holders.insert_holder(chunk_pos, player);
    holder.holds.insert(chunk_pos);
    loaded_chunks.loaded_chunks.insert(chunk_pos);

    if let Some(chunk) = chunk_map.chunk_at(chunk_pos) {
        send_chunk_data(chunk, net);
    } else {
        // Queue for loading
        load_chunk(chunk_handle, chunk_pos);
        lazy.exec_mut(move |world| {
            world
                .write_component::<ChunkPendingComponent>()
                .get_mut(player)
                .unwrap()
                .pending
                .insert(chunk_pos);
        });
    }
}

fn degrees_to_stops(degs: f32) -> u8 {
    ((degs / 360.0) * 256.) as u8
}
