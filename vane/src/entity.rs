/// The ID of an entity.
///
/// Pass this struct to various methods on the `Ecs`
/// to access the entity's components.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityId {
    index: u32,
    generation: u32,
}

impl EntityId {
    pub fn to_bits(self) -> u64 {
        ((self.index as u64) << 32) | (self.generation as u64)
    }

    pub fn from_bits(bits: u64) -> Self {
        let index = (bits >> 32) as u32;
        let generation = bits as u32;
        Self { index, generation }
    }

    pub fn index(self) -> u32 {
        self.index
    }

    pub fn generation(self) -> u32 {
        self.generation
    }
}

#[derive(Debug)]
pub struct GenerationMismatch;

/// Allocator for entity IDs. Maintains generations
/// and indices.
#[derive(Default)]
pub(crate) struct Entities {
    free_indices: Vec<u32>,
    next_index: u32,
    generations: Vec<u32>,
}

impl Entities {
    /// Allocates a new, unique entity ID.
    pub fn allocate(&mut self) -> EntityId {
        let index = self.free_indices.pop().unwrap_or_else(|| {
            self.next_index += 1;
            self.next_index - 1
        });
        let generation = self.new_generation(index);

        EntityId { index, generation }
    }

    /// Deallocates an entity ID, allowing its index to be reused.
    pub fn deallocate(&mut self, entity: EntityId) -> Result<(), GenerationMismatch> {
        self.check_generation(entity)?;

        self.free_indices.push(entity.index);

        self.generations[entity.index as usize] += 1;

        Ok(())
    }

    fn new_generation(&mut self, index: u32) -> u32 {
        if index == self.generations.len() as u32 {
            self.generations.push(0);
            0
        } else {
            self.generations[index as usize]
        }
    }

    /// Verifies that the generation of `entity` is up to date.
    pub fn check_generation(&self, entity: EntityId) -> Result<(), GenerationMismatch> {
        if self.generations[entity.index as usize] != entity.generation {
            Err(GenerationMismatch)
        } else {
            Ok(())
        }
    }

    /// Gets the entity with generation for the given index.
    pub fn get(&self, index: u32) -> EntityId {
        EntityId {
            index,
            generation: self.generations[index as usize],
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.generations
            .iter()
            .enumerate()
            .map(|(index, &generation)| EntityId {
                index: index as u32,
                generation,
            })
            .filter(move |entity| !self.free_indices.contains(&entity.index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_bits_from_bits_roundtrip() {
        let entity = EntityId {
            index: 10000,
            generation: 10000000,
        };
        assert_eq!(EntityId::from_bits(entity.to_bits()), entity);
    }

    #[test]
    fn entities_linear_allocation() {
        let mut entities = Entities::default();

        for i in 0..100 {
            let entity = entities.allocate();
            assert_eq!(entity.index(), i);
            assert_eq!(entity.generation(), 0);
            assert!(entities.check_generation(entity).is_ok());
        }

        entities
            .deallocate(EntityId {
                index: 5,
                generation: 0,
            })
            .unwrap();

        let entity = entities.allocate();
        assert_eq!(entity.index(), 5);
        assert_eq!(entity.generation(), 1);
        assert!(entities.check_generation(entity).is_ok());
    }
}
