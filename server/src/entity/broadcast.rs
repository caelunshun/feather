//! Module for broadcasting when an entity comes within
//! range of a player.

use crate::entity::movement::degrees_to_stops;
use crate::entity::{EntityComponent, EntityType};
use crate::network::{send_packet_to_all_players, NetworkComponent};
use feather_core::entitymeta::EntityMetadata;
use feather_core::network::packet::implementation::SpawnPlayer;
use shrev::EventChannel;
use specs::{Entities, Entity, Read, ReadStorage, ReaderId, System, SystemData, World};

//const ITEM_OBJECT_ID: i8 = 2;

/// Event triggered when an entity of any
/// type is spawned.
#[derive(Debug, Clone)]
pub struct EntitySpawnEvent {
    /// The spawned entity.
    pub entity: Entity,
    /// The type of the spawned entity.
    pub ty: EntityType,
}

/// System for broadcasting when an entity is spawned.
///
/// Different entity types require different packets
/// to send.
///
/// This system listens to `EntitySpawnEvent`s.
#[derive(Default)]
pub struct EntityBroadcastSystem {
    reader: Option<ReaderId<EntitySpawnEvent>>,
}

impl<'a> System<'a> for EntityBroadcastSystem {
    type SystemData = (
        ReadStorage<'a, EntityComponent>,
        ReadStorage<'a, NetworkComponent>,
        Read<'a, EventChannel<EntitySpawnEvent>>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entity_comps, networks, events, entities) = data;

        for event in events.read(&mut self.reader.as_mut().unwrap()) {
            let entity = entity_comps.get(event.entity).unwrap();

            // Send spawn packet to clients.
            // The packet type depends on the type
            // of entity.

            // The Player Info packet was already sent by `JoinBroadcastSystem`.
            match event.ty {
                EntityType::Player => {
                    let packet = SpawnPlayer {
                        entity_id: event.entity.id() as i32,
                        player_uuid: entity.uuid,
                        x: entity.position.x,
                        y: entity.position.y,
                        z: entity.position.z,
                        yaw: degrees_to_stops(entity.position.yaw),
                        pitch: degrees_to_stops(entity.position.pitch),
                        metadata: EntityMetadata::new(),
                    };

                    send_packet_to_all_players(&networks, &entities, packet, Some(event.entity));
                }
                EntityType::Item => unimplemented!(),
                EntityType::ExperienceOrb => unimplemented!(),
                EntityType::Thunderbolt => unimplemented!(),
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader = Some(world.fetch_mut::<EventChannel<_>>().register_reader());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testframework as t;
    use feather_core::network::cast_packet;
    use feather_core::network::packet::PacketType;
    use specs::WorldExt;

    #[test]
    fn test_spawn_player() {
        let (mut w, mut d) = t::init_world();

        let player1 = t::add_player(&mut w);
        let player2 = t::add_player(&mut w);

        let event = EntitySpawnEvent {
            entity: player1.entity,
            ty: EntityType::Player,
        };

        w.fetch_mut::<EventChannel<_>>().single_write(event);

        d.dispatch(&w);
        w.maintain();

        t::assert_packet_not_received(&player1, PacketType::SpawnPlayer); // Player shouldn't have received packet for themselves

        let packet = t::assert_packet_received(&player2, PacketType::SpawnPlayer);
        let packet = cast_packet::<SpawnPlayer>(&*packet);

        assert_eq!(packet.entity_id, player1.entity.id() as i32);
    }
}
