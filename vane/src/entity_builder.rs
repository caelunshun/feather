use std::{
    alloc::{alloc, dealloc},
    any::TypeId,
    mem::{size_of, MaybeUninit},
    ptr::{self, NonNull},
};

use crate::{component::ComponentMeta, Component, EntityId, World};

/// A utility to build an entity's components.
///
/// An `EntityBuilder` can be reused to avoid repeated allocations.
#[derive(Default)]
pub struct EntityBuilder {
    /// Packed vector containing component data.
    components: Vec<MaybeUninit<u8>>,
    entries: Vec<Entry>,
}

impl EntityBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a new component for the entity.
    ///
    /// If the entity builder already contains the component,
    /// then the previous value is overriden.
    pub fn add<T: Component>(&mut self, component: T) -> &mut Self {
        let component = MaybeUninit::new(component);
        self.components.reserve(size_of::<T>());

        let offset = self.components.len();
        unsafe {
            ptr::copy_nonoverlapping(
                component.as_ptr().cast::<MaybeUninit<u8>>(),
                self.components.as_mut_ptr().add(offset),
                size_of::<T>(),
            );

            self.components
                .set_len(self.components.len() + size_of::<T>());
        }
        self.entries.push(Entry {
            component_meta: ComponentMeta::of::<T>(),
            offset,
        });

        self
    }

    /// Determines whether the builder has a component
    /// of type T.
    pub fn has<T: Component>(&self) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.component_meta.type_id == TypeId::of::<T>())
    }

    /// Spawns the entity builder into an `Ecs`.
    pub fn spawn_into(&mut self, ecs: &mut World) -> EntityId {
        ecs.spawn_builder(self)
    }

    /// Drains the builder, returning tuples of
    /// the component meta and a pointer
    /// to the component data.
    ///
    /// NB: component data is not necessarily aligned.
    pub(crate) fn drain(&mut self) -> impl Iterator<Item = (ComponentMeta, NonNull<u8>)> + '_ {
        let components = &mut self.components;
        self.entries.drain(..).map(move |entry| {
            let component = unsafe {
                NonNull::new_unchecked(components.as_mut_ptr().add(entry.offset).cast::<u8>())
            };
            (entry.component_meta, component)
        })
    }

    /// Resets the builder, clearing all components.
    ///
    /// Does not invoke component drop functions.
    pub(crate) fn reset(&mut self) {
        self.entries.clear();
        self.components.clear();
    }
}

impl Drop for EntityBuilder {
    fn drop(&mut self) {
        for entry in self.entries.drain(..) {
            unsafe {
                let src_ptr = self.components.as_ptr().add(entry.offset).cast::<u8>();
                // Pointers in the entity builder are unaligned, so a
                // separate, aligned buffer is needed to store the component for dropping.
                let buffer = alloc(entry.component_meta.layout);
                std::ptr::copy_nonoverlapping(src_ptr, buffer, entry.component_meta.layout.size());

                (entry.component_meta.drop_fn)(buffer);

                dealloc(buffer, entry.component_meta.layout);
            }
        }
    }
}

struct Entry {
    component_meta: ComponentMeta,
    offset: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_entity() {
        let mut builder = EntityBuilder::new();

        builder.add(10i32).add("a string".to_owned()).add(50usize);

        unsafe {
            let mut iter = builder.drain();
            let (meta, data) = iter.next().unwrap();
            assert_eq!(meta.type_id, TypeId::of::<i32>());
            assert_eq!(ptr::read_unaligned::<i32>(data.cast().as_ptr()), 10i32);

            let (meta, data) = iter.next().unwrap();
            assert_eq!(meta.type_id, TypeId::of::<String>());
            assert_eq!(
                ptr::read_unaligned::<String>(data.cast().as_ptr()),
                "a string"
            );

            let (meta, data) = iter.next().unwrap();
            assert_eq!(meta.type_id, TypeId::of::<usize>());
            assert_eq!(ptr::read_unaligned::<usize>(data.cast().as_ptr()), 50usize);

            assert!(iter.next().is_none());
        }

        builder.reset();
        assert_eq!(builder.drain().count(), 0);
    }

    #[test]
    fn drops_components_on_drop() {
        let mut builder = EntityBuilder::new();
        builder.add(vec![1, 2, 3]);
        drop(builder);

        // A memory leak is detected by Miri if this fails
    }
}
