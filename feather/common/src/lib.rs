//! Gameplay functionality: entities, components, systems, game logic, ...
//!
//! This crate implements most functionality that is generic between
//! client and server, i.e., which does not involve interaction with the network.

#![allow(clippy::unnecessary_wraps)] // systems are required to return Results

mod game;
use ecs::SystemExecutor;
pub use game::Game;

mod tick_loop;
pub use tick_loop::TickLoop;

pub mod view;

pub mod window;
pub use window::Window;

pub mod events;

pub mod level_source;

pub mod level;
pub use level::Level;

mod chunk_loading;

mod chunk_entities;

pub mod chat;
pub use chat::ChatBox;

pub mod entities;

pub mod interactable;

/// Registers gameplay systems with the given `Game` and `SystemExecutor`.
pub fn register(game: &mut Game, systems: &mut SystemExecutor<Game>) {
    view::register(game, systems);
    chunk_loading::register(game, systems);
    chunk_entities::register(systems);
    interactable::register(game);

    game.add_entity_spawn_callback(entities::add_entity_components);
}
