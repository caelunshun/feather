use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::EnderDragon;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(EnderDragon).add(EntityKind::EnderDragon);
}
