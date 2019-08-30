use std::fs::File;
use std::io::Read;

use uuid::Uuid;

pub const UNSET_GAMEMODE: i32 = -1;

/// Represents the contents of a player data file.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    #[serde(default = "default_gamemode")]
    #[serde(rename = "playerGameType")]
    pub gamemode: i32,
}

/// Set to a magic value when player data doesn't exist (first-time join)
fn default_gamemode() -> i32 {
    // FIXME: Doesn't seem to work for default (non-existing NBT files)
    UNSET_GAMEMODE
}

pub fn deserialize_player_data<R: Read>(reader: R) -> Result<PlayerData, nbt::Error> {
    match nbt::from_gzip_reader::<_, PlayerData>(reader) {
        Ok(root) => Ok(root),
        Err(e) => Err(e),
    }
}

pub fn load_player_data(uuid: Uuid) -> Result<PlayerData, nbt::Error> {
    let file = File::open(format!("world/playerdata/{}.dat", uuid))?;
    let data = deserialize_player_data(file)?;
    Ok(data)
}
