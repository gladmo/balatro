use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::game_state::GameData;

/// Save game data to a JSON file
pub fn save_game(game_data: &GameData) -> Result<(), String> {
    let json = serde_json::to_string_pretty(game_data).map_err(|e| e.to_string())?;
    fs::write("save.json", json).map_err(|e| e.to_string())?;
    Ok(())
}

/// Load game data from a JSON file
pub fn load_game() -> Result<GameData, String> {
    let json = fs::read_to_string("save.json").map_err(|e| e.to_string())?;
    serde_json::from_str(&json).map_err(|e| e.to_string())
}

/// Check if a save file exists
pub fn has_save() -> bool {
    fs::metadata("save.json").is_ok()
}

/// Delete save file
pub fn delete_save() -> Result<(), String> {
    if has_save() {
        fs::remove_file("save.json").map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Save settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub screenshake: bool,
    pub game_speed: f32,
    pub language: String,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 1.0,
            sfx_volume: 1.0,
            screenshake: true,
            game_speed: 1.0,
            language: "en-us".to_string(),
        }
    }
}
