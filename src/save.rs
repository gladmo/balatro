#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub money: i32,
    pub ante: u32,
    pub round: u32,
    pub score: u64,
    pub run_score: u64,
    pub hands_remaining: u32,
    pub discards_remaining: u32,
    pub hand_size: u32,
    pub joker_slots: u32,
}

impl Default for SaveData {
    fn default() -> Self {
        SaveData {
            money: 4,
            ante: 1,
            round: 1,
            score: 0,
            run_score: 0,
            hands_remaining: 4,
            discards_remaining: 3,
            hand_size: 8,
            joker_slots: 5,
        }
    }
}

pub fn save_game(data: &SaveData, path: &str) -> Result<(), String> {
    let json = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    std::fs::write(path, json).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_game(path: &str) -> Option<SaveData> {
    let json = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&json).ok()
}

pub fn save_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}
