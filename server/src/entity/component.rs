//! Various Specs components.

use feather_core::entity::EntityData;
use feather_core::world::Position;
use feather_core::{Gamemode, Packet};
use glm::DVec3;
use specs::storage::BTreeStorage;
use specs::{Component, Entity, FlaggedStorage, Join, System, VecStorage, World, WriteStorage};
use uuid::Uuid;

pub struct PlayerComponent {
    pub profile_properties: Vec<mojang_api::ProfileProperty>,
    pub gamemode: Gamemode,
}

impl Component for PlayerComponent {
    type Storage = BTreeStorage<Self>;
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct PositionComponent {
    /// The current position of this entity.
    pub current: Position,
    /// The position of this entity on the previous
    /// tick. At the end of each tick, `reset` should
    /// be called.
    pub previous: Position,
}

impl PositionComponent {
    /// Resets the current and previous position.
    /// Should be called at the end of every tick.
    pub fn reset(&mut self) {
        self.previous = self.current;
    }
}

impl Component for PositionComponent {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

/// An entity's velocity, in blocks per tick.
///
/// Entities without this component are assumed
/// to have a velocity of 0.
#[derive(Deref, DerefMut, Debug, PartialEq, Clone, Copy)]
pub struct VelocityComponent(pub DVec3);

impl Component for VelocityComponent {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

impl Default for VelocityComponent {
    fn default() -> Self {
        Self(glm::vec3(0.0, 0.0, 0.0))
    }
}

#[derive(Clone, Debug)]
pub struct NamedComponent {
    pub display_name: String,
    pub uuid: Uuid,
}

impl Component for NamedComponent {
    type Storage = BTreeStorage<Self>;
}

pub trait PacketCreator: Fn(&World, Entity) -> Box<dyn Packet> + Send + Sync {}

impl<F: Fn(&World, Entity) -> Box<dyn Packet> + Send + Sync> PacketCreator for F {}

/// Component containing a closure which returns the packet
/// needed to spawn an entity.
///
/// The closure requires world access because it may need to access
/// arbitrary components.
pub struct PacketCreatorComponent(pub &'static dyn PacketCreator);

impl Component for PacketCreatorComponent {
    type Storage = VecStorage<Self>;
}

pub trait EntitySerializer: Fn(&World, Entity) -> EntityData + Send + Sync {}

impl<F: Fn(&World, Entity) -> EntityData + Send + Sync> EntitySerializer for F {}

/// Component containing a closure which returns the `EntityData`
/// for an entity.
///
/// The closure requires world access because it may need to access
/// arbitrary components.
pub struct SerializerComponent(pub &'static dyn EntitySerializer);

impl Component for SerializerComponent {
    type Storage = VecStorage<Self>;
}

/// System for resetting an entity's components
/// at the end of the tick.
pub struct ComponentResetSystem;

impl<'a> System<'a> for ComponentResetSystem {
    type SystemData = WriteStorage<'a, PositionComponent>;

    fn run(&mut self, mut positions: Self::SystemData) {
        // Ensure that position update events are not triggered
        // for this. See #81
        positions.set_event_emission(false);

        for position in (&mut positions).join() {
            position.reset();
        }

        positions.set_event_emission(true);
    }
}
