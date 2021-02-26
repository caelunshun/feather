use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Slime;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Slime).add(EntityKind::Slime);
}
