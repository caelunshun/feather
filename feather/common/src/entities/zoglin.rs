use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Zoglin};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Zoglin)
        .add(Health::new(40))
        .add(EntityKind::Zoglin);
}
