use shrev::EventChannel;
use specs::{
    Component, Entity, NullStorage, Read, ReadStorage, ReaderId, System, SystemData, World,
};

use feather_core::{Item, Position};

use crate::entity::NamedComponent;
use crate::player::PLAYER_EYE_HEIGHT;
use crate::util::Util;

/// Component for arrow entities.
#[derive(Default)]
pub struct ArrowComponent;

impl Component for ArrowComponent {
    type Storage = NullStorage<Self>;
}

/// Event triggered when arrow is shot.
#[derive(Debug, Clone)]
pub struct ShootArrowEvent {
    pub arrow_type: Item,
    pub shooter: Option<Entity>,
    pub position: Position,
    pub critical: bool,
}

#[derive(Default)]
pub struct ShootArrowSystem {
    reader: Option<ReaderId<ShootArrowEvent>>,
}

impl<'a> System<'a> for ShootArrowSystem {
    type SystemData = (
        Read<'a, Util>,
        Read<'a, EventChannel<ShootArrowEvent>>,
        ReadStorage<'a, NamedComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (util, shoot_arrow_events, nameds) = data;

        for event in shoot_arrow_events.read(self.reader.as_mut().unwrap()) {
            let mut pos = event.position
                + glm::vec3(0.0, PLAYER_EYE_HEIGHT, 0.0)
                + event.position.direction() * 1.5;
            pos.on_ground = false;

            // TODO: Scale velocity based on power
            let velocity = pos.direction();

            let shooter = match event.shooter {
                Some(e) => Some(nameds.get(e).unwrap().uuid),
                None => None,
            };

            util.spawn_arrow(pos, velocity, event.critical, shooter);
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader = Some(world.fetch_mut::<EventChannel<_>>().register_reader());
    }
}
