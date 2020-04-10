//! Broadcasting of movement updates.

use crate::entity::{EntityId, PreviousPosition, PreviousVelocity, Velocity};
use crate::game::Game;
use crate::network::Network;
use crate::util::{calculate_relative_move, degrees_to_stops, protocol_velocity};
use dashmap::DashMap;
use feather_core::network::packet::implementation::{
    EntityHeadLook, EntityLook, EntityLookAndRelativeMove, EntityRelativeMove, EntityVelocity,
};
use feather_core::{Packet, Position};
use fecs::{Entity, IntoQuery, Read, World};
use smallvec::SmallVec;
use std::ops::Deref;

/// Component containing the last sent positions of all entities for a given client.
/// This component is used to determine
/// the relative movement for an entity.
#[derive(Default, Debug)]
pub struct LastKnownPositions(pub DashMap<Entity, Position>);

/// System to broadcast when an entity moves.
#[system]
pub fn broadcast_movement(game: &mut Game, world: &mut World) {
    <(Read<Position>, Read<PreviousPosition>, Read<EntityId>)>::query().par_entities_for_each(
        world.inner(),
        |(entity, (pos, prev_pos, id))| {
            let pos: Position = *pos;
            let prev_pos: Position = prev_pos.0;

            if pos == prev_pos {
                return;
            }

            let entity_id = id.0;

            let chunk = pos.chunk();
            let players = game.chunk_holders.holders_for(chunk);

            for player in players.iter().filter(|player| **player != entity) {
                if let Some(network) = world.try_get::<Network>(*player) {
                    let last_known_positions = world.get::<LastKnownPositions>(*player);
                    let last_known_positions = last_known_positions.deref();

                    if let Some(mut last_known_pos) = last_known_positions.0.get_mut(&entity) {
                        for packet in
                            packets_for_movement_update(entity_id, *last_known_pos.value(), pos)
                        {
                            network.send_boxed(packet);
                        }

                        trace!("Updated position of {:?} on client {:?}", entity, player);

                        *last_known_pos.value_mut() = pos;
                    } else {
                        trace!(
                            "Missing last position entry for {:?} on client {:?}",
                            entity,
                            player
                        );
                    };
                }
            }
        },
    );
}

pub fn on_entity_send_update_last_known_positions(world: &World, entity: Entity, client: Entity) {
    if let Some(last_known_positions) = world.try_get::<LastKnownPositions>(client) {
        let pos = *world.get::<Position>(entity);
        last_known_positions.0.insert(entity, pos);
        trace!(
            "Inserted last position entry for {:?} (player: {:?})",
            entity,
            client
        );
    }
}

pub fn on_entity_client_remove_update_last_known_positions(
    world: &World,
    entity: Entity,
    client: Entity,
) {
    if let Some(last_known_positions) = world.try_get::<LastKnownPositions>(client) {
        trace!(
            "Removing last position entry for {:?} (player: {:?})",
            entity,
            client
        );
        last_known_positions.0.remove(&entity);
    }
}

/// Broadcasts an entity's velocity.
#[system]
pub fn broadcast_velocity(world: &mut World, game: &mut Game) {
    <(Read<Velocity>, Read<PreviousVelocity>, Read<EntityId>)>::query().par_entities_for_each(
        world.inner(),
        |(entity, (vel, prev_vel, entity_id))| {
            let entity_id = entity_id.0;

            if vel.0 == prev_vel.0 {
                return;
            }

            let (velocity_x, velocity_y, velocity_z) = protocol_velocity(vel.0);

            if velocity_x == 0 && velocity_y == 0 && velocity_z == 0 {
                return;
            }

            let packet = EntityVelocity {
                entity_id,
                velocity_x,
                velocity_y,
                velocity_z,
            };
            game.broadcast_entity_update(world, packet, entity, None);
        },
    );
}

/// Returns the packet needed to notify a client
/// of a position update, from the old position to the new one.
#[allow(clippy::float_cmp)]
fn packets_for_movement_update(
    entity_id: i32,
    old_pos: Position,
    new_pos: Position,
) -> SmallVec<[Box<dyn Packet>; 2]> {
    if old_pos == new_pos {
        return smallvec![];
    }

    let mut packets = smallvec![];

    let has_moved = old_pos.x != new_pos.x || old_pos.y != new_pos.y || old_pos.z != new_pos.z;
    let has_looked = old_pos.pitch != new_pos.pitch
        || old_pos.yaw != new_pos.yaw
        || old_pos.on_ground != new_pos.on_ground;

    if has_moved {
        let (rx, ry, rz) = calculate_relative_move(old_pos, new_pos);

        if (rx == 0 && ry == 0 && rz == 0) && !has_looked {
            // Because of floating point errors,
            // the physics system may trigger an
            // event when the distance moved is minuscule,
            // which causes jittering on the client.
            // Don't send the packet if it has no effect.
            return smallvec![];
        }

        if has_looked {
            let packet: Box<dyn Packet> = Box::new(EntityLookAndRelativeMove {
                entity_id,
                delta_x: rx,
                delta_y: ry,
                delta_z: rz,
                yaw: degrees_to_stops(new_pos.yaw),
                pitch: degrees_to_stops(new_pos.pitch),
                on_ground: new_pos.on_ground,
            });
            packets.push(packet);
        } else {
            let packet: Box<dyn Packet> = Box::new(EntityRelativeMove {
                entity_id,
                delta_x: rx,
                delta_y: ry,
                delta_z: rz,
                on_ground: new_pos.on_ground,
            });
            packets.push(packet);
        }
    } else {
        let packet: Box<dyn Packet> = Box::new(EntityLook {
            entity_id,
            yaw: degrees_to_stops(new_pos.yaw),
            pitch: degrees_to_stops(new_pos.pitch),
            on_ground: new_pos.on_ground,
        });
        packets.push(packet);
    }

    // Entity Head Look also needs to be sent if the entity turned its head
    if has_looked {
        let packet: Box<dyn Packet> = Box::new(EntityHeadLook {
            entity_id,
            head_yaw: degrees_to_stops(new_pos.yaw),
        });
        packets.push(packet);
    }

    packets
}
