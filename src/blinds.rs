use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game_state::{AppState, BlindType, GameData};

pub struct BlindsPlugin;

impl Plugin for BlindsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::BlindSelect), setup_blind_info);
    }
}

/// Boss blind types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BossBlindKind {
    TheHook,
    TheOx,
    TheHouse,
    TheMark,
    TheWheel,
    TheArm,
    TheClub,
    TheFish,
    ThePsychic,
    TheGoad,
    TheWater,
    TheWindow,
    TheManacle,
    TheEye,
    TheMouth,
    ThePlant,
    TheSerpent,
    ThePillar,
    TheNeedle,
    TheHead,
    TheTooth,
    TheWall,
    TheFlint,
}

impl BossBlindKind {
    pub fn display_name(&self) -> &str {
        match self {
            BossBlindKind::TheHook => "The Hook",
            BossBlindKind::TheOx => "The Ox",
            BossBlindKind::TheHouse => "The House",
            BossBlindKind::TheMark => "The Mark",
            BossBlindKind::TheWheel => "The Wheel",
            BossBlindKind::TheArm => "The Arm",
            BossBlindKind::TheClub => "The Club",
            BossBlindKind::TheFish => "The Fish",
            BossBlindKind::ThePsychic => "The Psychic",
            BossBlindKind::TheGoad => "The Goad",
            BossBlindKind::TheWater => "The Water",
            BossBlindKind::TheWindow => "The Window",
            BossBlindKind::TheManacle => "The Manacle",
            BossBlindKind::TheEye => "The Eye",
            BossBlindKind::TheMouth => "The Mouth",
            BossBlindKind::ThePlant => "The Plant",
            BossBlindKind::TheSerpent => "The Serpent",
            BossBlindKind::ThePillar => "The Pillar",
            BossBlindKind::TheNeedle => "The Needle",
            BossBlindKind::TheHead => "The Head",
            BossBlindKind::TheTooth => "The Tooth",
            BossBlindKind::TheWall => "The Wall",
            BossBlindKind::TheFlint => "The Flint",
        }
    }
}

/// Get chip requirement for an ante and blind type
pub fn get_blind_chips(ante: u32, blind_type: &BlindType) -> u64 {
    let base = match ante {
        1 => 300,
        2 => 800,
        3 => 2800,
        4 => 6000,
        5 => 11000,
        6 => 20000,
        7 => 35000,
        8 => 50000,
        _ => 50000 + (ante as u64 - 8) * 25000,
    };
    match blind_type {
        BlindType::SmallBlind => base,
        BlindType::BigBlind => (base as f64 * 1.5) as u64,
        BlindType::BossBlind => base * 2,
    }
}

fn setup_blind_info(mut game_data: ResMut<GameData>) {
    let chips = get_blind_chips(game_data.ante, &game_data.current_blind);
    game_data.blind_chips_required = chips;
    game_data.blind_chips_scored = 0;
}
