use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::EndCrystal;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(EndCrystal).add(EntityKind::EndCrystal);
}
