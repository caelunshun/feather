use std::{cell::RefCell, mem, rc::Rc, sync::Arc};

use base::{Position, Text, Title};
use ecs::{
    Ecs, Entity, EntityBuilder, HasEcs, HasResources, NoSuchEntity, Resources, SysResult,
    SystemExecutor,
};
use libcraft_core::EntityKind;
use quill_common::entities::Player;

use crate::events::PlayerRespawnEvent;
use crate::{
    chat::{ChatKind, ChatMessage},
    chunk::entities::ChunkEntities,
    events::{EntityCreateEvent, EntityRemoveEvent, PlayerJoinEvent},
    ChatBox,
};

type EntitySpawnCallback = Box<dyn FnMut(&mut EntityBuilder, EntityKind)>;

/// Stores the entire state of a Minecraft game.
///
/// This contains:
/// * A [`World`](crate::World) containing chunks and blocks.
/// * An [`Ecs`](ecs::Ecs) containing entities.
/// * A [`Resources`](ecs::Resources) containing additional, user-defined data.
/// * A [`SystemExecutor`] to run systems.
///
/// `feather-common` provides `Game` methods for actions such
/// as "drop item" or "kill entity." These high-level methods
/// should be preferred over raw interaction with the ECS.
pub struct Game {
    /// Contains entities, including players.
    pub ecs: Ecs,
    /// Contains systems.
    pub system_executor: Rc<RefCell<SystemExecutor<Game>>>,

    /// User-defined resources.
    ///
    /// Stored in an `Arc` for borrow-checker purposes.
    pub resources: Arc<Resources>,

    /// A spatial index to efficiently find which entities are in a given chunk.
    pub chunk_entities: ChunkEntities,

    /// Total ticks elapsed since the server started.
    pub tick_count: u64,

    entity_spawn_callbacks: Vec<EntitySpawnCallback>,

    entity_builder: EntityBuilder,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    /// Creates a new, empty `Game`.
    pub fn new() -> Self {
        Self {
            ecs: Ecs::new(),
            system_executor: Rc::new(RefCell::new(SystemExecutor::new())),
            resources: Arc::new(Resources::new()),
            chunk_entities: ChunkEntities::default(),
            tick_count: 0,
            entity_spawn_callbacks: Vec::new(),
            entity_builder: EntityBuilder::new(),
        }
    }

    /// Inserts a new resource.
    ///
    /// An existing resource with type `T` is overridden.
    ///
    /// # Panics
    /// Panics if any resources are currently borrowed.
    pub fn insert_resource<T>(&mut self, resource: T)
    where
        T: 'static,
    {
        Arc::get_mut(&mut self.resources)
            .expect("attempted to insert into resources while resources are borrowed")
            .insert(resource);
    }

    /// Adds a new entity spawn callback, invoked
    /// before an entity is created.
    ///
    /// This allows you to add components to entities
    /// before they are built.
    pub fn add_entity_spawn_callback(
        &mut self,
        callback: impl FnMut(&mut EntityBuilder, EntityKind) + 'static,
    ) {
        self.entity_spawn_callbacks.push(Box::new(callback));
    }

    /// Creates an empty entity builder to create entities in
    /// the ecs world.
    pub fn create_empty_entity_builder(&mut self) -> EntityBuilder {
        mem::take(&mut self.entity_builder)
    }

    /// Creates an entity builder with the default components
    /// for an entity of type `init`.
    pub fn create_entity_builder(&mut self, position: Position, kind: EntityKind) -> EntityBuilder {
        let mut builder = mem::take(&mut self.entity_builder);
        builder.add(position);
        self.invoke_entity_spawn_callbacks(&mut builder, kind);
        builder
    }

    /// Spawns an entity and returns its [`Entity`](ecs::Entity) handle.
    ///
    /// Also triggers necessary events, like `EntitySpawnEvent` and `PlayerJoinEvent`.
    pub fn spawn_entity(&mut self, mut builder: EntityBuilder) -> Entity {
        let entity = self.ecs.spawn(builder.build());
        self.entity_builder = builder;

        self.trigger_entity_spawn_events(entity);

        entity
    }

    fn invoke_entity_spawn_callbacks(&mut self, builder: &mut EntityBuilder, kind: EntityKind) {
        let mut callbacks = mem::take(&mut self.entity_spawn_callbacks);
        for callback in &mut callbacks {
            callback(builder, kind);
        }
        self.entity_spawn_callbacks = callbacks;
    }

    fn trigger_entity_spawn_events(&mut self, entity: Entity) {
        self.ecs
            .insert_entity_event(entity, EntityCreateEvent)
            .unwrap();
        if self.ecs.get::<Player>(entity).is_ok() {
            self.ecs
                .insert_entity_event(entity, PlayerJoinEvent)
                .unwrap();
            self.ecs
                .insert_entity_event(entity, PlayerRespawnEvent)
                .unwrap();
        }
    }

    /// Causes the given entity to be removed on the next tick.
    /// In the meantime, triggers `EntityRemoveEvent`.
    pub fn remove_entity(&mut self, entity: Entity) -> Result<(), NoSuchEntity> {
        self.ecs.defer_despawn(entity);
        self.ecs.insert_entity_event(entity, EntityRemoveEvent)
    }

    /// Broadcasts a chat message to all entities with
    /// a `ChatBox` component (usually just players).
    pub fn broadcast_chat(&self, kind: ChatKind, message: impl Into<Text>) {
        let message = message.into();
        for (_, mailbox) in self.ecs.query::<&mut ChatBox>().iter() {
            mailbox.send(ChatMessage::new(kind, message.clone()));
        }
    }

    /// Utility method to send a message to an entity.
    pub fn send_message(&mut self, entity: Entity, message: ChatMessage) -> SysResult {
        let mut mailbox = self.ecs.get_mut::<ChatBox>(entity)?;
        mailbox.send(message);
        Ok(())
    }

    /// Utility method to send a title to an entity.
    pub fn send_title(&mut self, entity: Entity, title: Title) -> SysResult {
        let mut mailbox = self.ecs.get_mut::<ChatBox>(entity)?;
        mailbox.send_title(title);
        Ok(())
    }
}

impl HasResources for Game {
    fn resources(&self) -> Arc<Resources> {
        Arc::clone(&self.resources)
    }
}

impl HasEcs for Game {
    fn ecs(&self) -> &Ecs {
        &self.ecs
    }

    fn ecs_mut(&mut self) -> &mut Ecs {
        &mut self.ecs
    }
}
