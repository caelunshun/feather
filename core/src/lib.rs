#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate feather_codegen;
#[macro_use]
extern crate num_derive;

pub mod bytebuf;
pub mod entitymeta;
pub mod network;
pub mod prelude;
mod save;
pub mod world;

pub use save::{level, player_data, region};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl Gamemode {
    pub fn get_id(self) -> u8 {
        match self {
            Gamemode::Survival => 0,
            Gamemode::Creative => 1,
            Gamemode::Adventure => 2,
            Gamemode::Spectator => 3,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn get_id(self) -> u8 {
        match self {
            Difficulty::Peaceful => 0,
            Difficulty::Easy => 1,
            Difficulty::Medium => 2,
            Difficulty::Hard => 3,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dimension {
    Nether,
    Overwold,
    End,
}

impl Dimension {
    pub fn get_id(self) -> i32 {
        match self {
            Dimension::Nether => -1,
            Dimension::Overwold => 0,
            Dimension::End => 1,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PvpStyle {
    Classic,
    New,
}
