use crate::entity::EntityId;
use bytemuck::{Pod, Zeroable};

/// Marker component for mooshroom entities.
///
/// # Example
/// A system that queries for all strays:
/// ```no_run
/// use quill::{Game, Position, entities::StrayMarker};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &StrayMarker)>() {
///         println!("Found a stray with position "stray"", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct StrayMarker;

pod_component_impl!(StrayMarker);

/// Entity wrapper for stray entities.
///
/// Implements several traits providing high-level methods
/// like "deal damage".
pub struct Stray {
    id: EntityId,
}

wrapper_from_query_impl!(Stray, StrayMarker);
entity_wrapper_impl!(Stray, StrayMarker);

impl crate::HasEntityId for Stray {
    fn entity_id(&self) -> EntityId {
        self.id
    }
}
