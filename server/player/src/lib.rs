#![forbid(unsafe_code)]

extern crate nalgebra_glm as glm;

mod broadcasters;
mod chat;
mod join;
mod packet_handlers;
mod view;

use feather_core::inventory::{Inventory, InventoryType};
use feather_core::items::{Item, ItemStack};
use feather_core::network::packets::{PlayerInfo, PlayerInfoAction, SpawnPlayer};
use feather_core::network::Packet;
use feather_core::text::Text;
use feather_core::util::{Gamemode, Position};
use feather_server_network::NewClientInfo;
use feather_server_types::{
    CanBreak, CanInstaBreak, CanTakeDamage, ChunkHolder, CreationPacketCreator, EntitySpawnEvent,
    Game, HeldItem, InventoryUpdateEvent, LastKnownPositions, Name, Network, NetworkId, Player,
    PlayerJoinEvent, PlayerPreJoinEvent, PreviousPosition, ProfileProperties, SpawnPacketCreator,
    Uuid,
};
use feather_server_util::degrees_to_stops;
use fecs::{Entity, EntityRef, World};

pub use broadcasters::*;
pub use chat::*;
pub use join::*;
pub use packet_handlers::*;
use std::sync::atomic::Ordering;
pub use view::*;

pub const PLAYER_INVENTORY_SIZE: u32 = 46;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemTimedUse {
    pub tick_start: u64,
}

/// Creates a new player from the given `NewClientInfo`.
///
/// This function also triggers events for the player join.
pub fn create(game: &mut Game, world: &mut World, info: NewClientInfo) -> Entity {
    // TODO: blocked on https://github.com/TomGillen/legion/issues/36
    let entity = info.entity;
    world.add(entity, NetworkId(entity::new_id())).unwrap();
    world.add(entity, info.position).unwrap();
    world.add(entity, PreviousPosition(info.position)).unwrap();
    world.add(entity, info.uuid).unwrap();
    world
        .add(
            entity,
            Network {
                tx: info.sender,
                rx: info.receiver.into(),
            },
        )
        .unwrap();
    world.add(entity, info.ip).unwrap();
    world.add(entity, ProfileProperties(info.profile)).unwrap();
    world.add(entity, Name(info.username)).unwrap();
    world.add(entity, ChunkHolder::default()).unwrap();
    world.add(entity, LastKnownPositions::default()).unwrap();
    world
        .add(entity, SpawnPacketCreator(&create_spawn_packet))
        .unwrap();
    world
        .add(entity, CreationPacketCreator(&create_initialization_packet))
        .unwrap();

    let gamemode = Gamemode::from_id(info.data.gamemode as u8);
    add_gamemode_comps(world, gamemode, entity);

    let items = info.data.inventory.iter().map(|slot| {
        (
            slot.slot as usize,
            ItemStack::new(
                Item::from_identifier(&slot.item).unwrap_or(Item::Air),
                slot.count as u8,
            ),
        )
    });
    let slots = info.data.inventory.iter().map(|slot| slot.slot as usize);

    let mut inventory = Inventory::new(InventoryType::Player, PLAYER_INVENTORY_SIZE);
    items.for_each(|(index, item)| inventory.set_item_at(index, item));

    world.add(entity, inventory).unwrap();
    world.add(entity, HeldItem(0)).unwrap(); // todo: load from player data

    world.add(entity, Player).unwrap();

    game.player_count.fetch_add(1, Ordering::SeqCst);
    game.handle(world, EntitySpawnEvent { entity });
    game.handle(world, PlayerPreJoinEvent { player: entity });
    game.handle(world, PlayerJoinEvent { player: entity });
    game.handle(
        world,
        InventoryUpdateEvent {
            slots: slots.collect(),
            player: entity,
        },
    );

    entity
}

fn add_gamemode_comps(world: &mut World, gamemode: Gamemode, entity: Entity) {
    world.add(entity, gamemode).unwrap();
    match gamemode {
        Gamemode::Survival | Gamemode::Adventure => world.add(entity, CanTakeDamage).unwrap(),
        Gamemode::Creative => world.add(entity, CanInstaBreak).unwrap(),
        _ => (),
    }

    if gamemode == Gamemode::Survival || gamemode == Gamemode::Creative {
        world.add(entity, CanBreak).unwrap();
    }
}

/// Function to create a `SpawnPlayer` packet to spawn the player.
fn create_spawn_packet(accessor: &EntityRef) -> Box<dyn Packet> {
    let entity_id = accessor.get::<NetworkId>().0;
    let player_uuid = *accessor.get::<Uuid>();
    let pos = *accessor.get::<Position>();

    let packet = SpawnPlayer {
        entity_id,
        player_uuid,
        x: pos.x,
        y: pos.y,
        z: pos.z,
        yaw: degrees_to_stops(pos.yaw),
        pitch: degrees_to_stops(pos.pitch),
        metadata: Default::default(),
    };
    Box::new(packet)
}

/// Function to create a `PlayerInfo` packet to broadcast when the player joins.
fn create_initialization_packet(accessor: &EntityRef) -> Box<dyn Packet> {
    let name = accessor.get::<Name>();
    let props = accessor.get::<ProfileProperties>();
    let uuid = *accessor.get::<Uuid>();

    let props = props
        .0
        .iter()
        .map(|prop| {
            (
                prop.name.clone(),
                prop.value.clone(),
                prop.signature.clone(),
            )
        })
        .collect::<Vec<_>>();

    let display_name = Text::of(name.0.clone()).into();

    let action =
        PlayerInfoAction::AddPlayer(name.0.clone(), props, Gamemode::Creative, 50, display_name);

    let packet = PlayerInfo { action, uuid };
    Box::new(packet)
}
