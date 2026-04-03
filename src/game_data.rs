#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;
use std::collections::HashMap;
use crate::hand_eval::HandType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BlindType {
    #[default]
    Small,
    Big,
    Boss,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BossBlind {
    TheHook,
    TheWall,
    TheOx,
    TheWheel,
    ThePsychic,
    TheGoad,
    TheHead,
    TheClub,
    TheWindow,
    TheMark,
    CeruleanBell,
}

impl BossBlind {
    pub fn name(&self) -> &'static str {
        match self {
            BossBlind::TheHook => "The Hook",
            BossBlind::TheWall => "The Wall",
            BossBlind::TheOx => "The Ox",
            BossBlind::TheWheel => "The Wheel",
            BossBlind::ThePsychic => "The Psychic",
            BossBlind::TheGoad => "The Goad",
            BossBlind::TheHead => "The Head",
            BossBlind::TheClub => "The Club",
            BossBlind::TheWindow => "The Window",
            BossBlind::TheMark => "The Mark",
            BossBlind::CeruleanBell => "Cerulean Bell",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            BossBlind::TheHook => "Discards 2 random cards per hand played",
            BossBlind::TheWall => "Required chips ×2",
            BossBlind::TheOx => "Playing most common hand sets money to $0",
            BossBlind::TheWheel => "1 in 7 cards get flipped face-down",
            BossBlind::ThePsychic => "Must play exactly 5 cards",
            BossBlind::TheGoad => "Spades are debuffed",
            BossBlind::TheHead => "Hearts are debuffed",
            BossBlind::TheClub => "Clubs are debuffed",
            BossBlind::TheWindow => "Diamonds are debuffed",
            BossBlind::TheMark => "All face cards are drawn face-down",
            BossBlind::CeruleanBell => "Forces you to include one specific card",
        }
    }

    pub fn from_index(index: usize) -> BossBlind {
        let all = [
            BossBlind::TheHook,
            BossBlind::TheWall,
            BossBlind::TheOx,
            BossBlind::TheWheel,
            BossBlind::ThePsychic,
            BossBlind::TheGoad,
            BossBlind::TheHead,
            BossBlind::TheClub,
            BossBlind::TheWindow,
            BossBlind::TheMark,
            BossBlind::CeruleanBell,
        ];
        all[index % all.len()]
    }
}

#[derive(Resource)]
pub struct GameData {
    pub money: i32,
    pub hands_remaining: u32,
    pub discards_remaining: u32,
    pub hand_size: u32,
    pub joker_slots: u32,
    pub ante: u32,
    pub round: u32,
    pub score: u64,
    pub blind_target: u64,
    pub blind_type: BlindType,
    pub boss_blind: Option<BossBlind>,
    pub hand_play_counts: HashMap<HandType, u32>,
    pub run_score: u64,
    pub base_hands: u32,
    pub base_discards: u32,
}

impl Default for GameData {
    fn default() -> Self {
        GameData {
            money: 4,
            hands_remaining: 4,
            discards_remaining: 3,
            hand_size: 8,
            joker_slots: 5,
            ante: 1,
            round: 1,
            score: 0,
            blind_target: 300,
            blind_type: BlindType::Small,
            boss_blind: None,
            hand_play_counts: HashMap::new(),
            run_score: 0,
            base_hands: 4,
            base_discards: 3,
        }
    }
}

impl GameData {
    pub fn new() -> Self {
        GameData::default()
    }

    pub fn blind_target_for(&self, ante: u32, round: u32) -> u64 {
        let base = match ante {
            1 => [300u64, 450, 600],
            2 => [800, 1200, 1600],
            3 => [2000, 3000, 4000],
            4 => [6000, 9000, 12000],
            5 => [20000, 30000, 40000],
            6 => [60000, 90000, 120000],
            7 => [200000, 300000, 400000],
            8 => [600000, 900000, 1200000],
            _ => [600000, 900000, 1200000],
        };
        let r = (round as usize).saturating_sub(1).min(2);
        base[r]
    }

    pub fn blind_reward(&self) -> i32 {
        match self.blind_type {
            BlindType::Small => 3,
            BlindType::Big => 4,
            BlindType::Boss => 5,
        }
    }

    pub fn interest(&self) -> i32 {
        (self.money / 5).min(5)
    }

    pub fn reset_for_new_round(&mut self, extra_hands: u32, extra_discards: u32) {
        self.hands_remaining = self.base_hands + extra_hands;
        self.discards_remaining = self.base_discards + extra_discards;
        self.score = 0;
    }

    pub fn advance_blind(&mut self) {
        match self.blind_type {
            BlindType::Small => {
                self.blind_type = BlindType::Big;
                self.round = 2;
                self.blind_target = self.blind_target_for(self.ante, 2);
            }
            BlindType::Big => {
                self.blind_type = BlindType::Boss;
                self.round = 3;
                self.blind_target = self.blind_target_for(self.ante, 3);
                self.boss_blind = Some(BossBlind::from_index((self.ante as usize).wrapping_sub(1)));
            }
            BlindType::Boss => {
                self.ante += 1;
                self.blind_type = BlindType::Small;
                self.round = 1;
                self.boss_blind = None;
                self.blind_target = self.blind_target_for(self.ante, 1);
            }
        }
    }

    pub fn record_hand_play(&mut self, hand_type: HandType) {
        *self.hand_play_counts.entry(hand_type).or_insert(0) += 1;
    }

    pub fn times_played_this_run(&self, hand_type: HandType) -> u32 {
        *self.hand_play_counts.get(&hand_type).unwrap_or(&0)
    }

    pub fn is_game_won(&self) -> bool {
        self.ante > 8
    }

    pub fn blind_name(&self) -> String {
        match self.blind_type {
            BlindType::Small => "Small Blind".to_string(),
            BlindType::Big => "Big Blind".to_string(),
            BlindType::Boss => {
                if let Some(boss) = self.boss_blind {
                    boss.name().to_string()
                } else {
                    "Boss Blind".to_string()
                }
            }
        }
    }
}
