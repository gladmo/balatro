#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JokerId {
    Joker,
    GreedyJoker,
    LustyJoker,
    WrathfulJoker,
    GluttonousJoker,
    JollyJoker,
    ZanyJoker,
    MadJoker,
    CrazyJoker,
    DrollJoker,
    SlyJoker,
    HalfJoker,
    Banner,
    Misprint,
    ScaryFace,
    AbstractJoker,
    Scholar,
    BusinessCard,
    Supernova,
    BlueJoker,
    Fibonacci,
    StoneJoker,
    GoldenJoker,
    Juggler,
    Drunkard,
}

impl JokerId {
    pub fn name(&self) -> &'static str {
        match self {
            JokerId::Joker => "Joker",
            JokerId::GreedyJoker => "Greedy Joker",
            JokerId::LustyJoker => "Lusty Joker",
            JokerId::WrathfulJoker => "Wrathful Joker",
            JokerId::GluttonousJoker => "Gluttonous Joker",
            JokerId::JollyJoker => "Jolly Joker",
            JokerId::ZanyJoker => "Zany Joker",
            JokerId::MadJoker => "Mad Joker",
            JokerId::CrazyJoker => "Crazy Joker",
            JokerId::DrollJoker => "Droll Joker",
            JokerId::SlyJoker => "Sly Joker",
            JokerId::HalfJoker => "Half Joker",
            JokerId::Banner => "Banner",
            JokerId::Misprint => "Misprint",
            JokerId::ScaryFace => "Scary Face",
            JokerId::AbstractJoker => "Abstract Joker",
            JokerId::Scholar => "Scholar",
            JokerId::BusinessCard => "Business Card",
            JokerId::Supernova => "Supernova",
            JokerId::BlueJoker => "Blue Joker",
            JokerId::Fibonacci => "Fibonacci",
            JokerId::StoneJoker => "Stone Joker",
            JokerId::GoldenJoker => "Golden Joker",
            JokerId::Juggler => "Juggler",
            JokerId::Drunkard => "Drunkard",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            JokerId::Joker => "+4 Mult",
            JokerId::GreedyJoker => "+4 Mult per Diamond played",
            JokerId::LustyJoker => "+4 Mult per Heart played",
            JokerId::WrathfulJoker => "+4 Mult per Spade played",
            JokerId::GluttonousJoker => "+4 Mult per Club played",
            JokerId::JollyJoker => "+8 Mult if hand has Pair",
            JokerId::ZanyJoker => "+12 Mult if Three of a Kind",
            JokerId::MadJoker => "+20 Mult if Four of a Kind",
            JokerId::CrazyJoker => "+12 Mult if Straight",
            JokerId::DrollJoker => "+10 Mult if Flush",
            JokerId::SlyJoker => "+50 Chips if Pair",
            JokerId::HalfJoker => "+20 Mult if 3 or fewer cards played",
            JokerId::Banner => "+40 Chips per remaining discard",
            JokerId::Misprint => "+0 to +23 random Mult",
            JokerId::ScaryFace => "+30 Chips per face card played",
            JokerId::AbstractJoker => "+3 Mult per joker owned",
            JokerId::Scholar => "+20 Chips, +4 Mult per Ace played",
            JokerId::BusinessCard => "Face cards 1/2 chance earn $2",
            JokerId::Supernova => "+1 Mult per time hand played this run",
            JokerId::BlueJoker => "+2 Chips per card remaining in deck",
            JokerId::Fibonacci => "+8 Mult for A, 2, 3, 5, 8 played",
            JokerId::StoneJoker => "+25 Chips per Stone card",
            JokerId::GoldenJoker => "Earn $4 at end of round",
            JokerId::Juggler => "+1 hand size",
            JokerId::Drunkard => "+1 discard per round",
        }
    }

    pub fn base_cost(&self) -> i32 {
        match self {
            JokerId::Joker => 2,
            JokerId::GreedyJoker => 5,
            JokerId::LustyJoker => 5,
            JokerId::WrathfulJoker => 5,
            JokerId::GluttonousJoker => 5,
            JokerId::JollyJoker => 4,
            JokerId::ZanyJoker => 4,
            JokerId::MadJoker => 4,
            JokerId::CrazyJoker => 4,
            JokerId::DrollJoker => 4,
            JokerId::SlyJoker => 3,
            JokerId::HalfJoker => 5,
            JokerId::Banner => 5,
            JokerId::Misprint => 4,
            JokerId::ScaryFace => 4,
            JokerId::AbstractJoker => 4,
            JokerId::Scholar => 4,
            JokerId::BusinessCard => 4,
            JokerId::Supernova => 5,
            JokerId::BlueJoker => 3,
            JokerId::Fibonacci => 8,
            JokerId::StoneJoker => 6,
            JokerId::GoldenJoker => 6,
            JokerId::Juggler => 4,
            JokerId::Drunkard => 4,
        }
    }

    pub fn all() -> Vec<JokerId> {
        vec![
            JokerId::Joker,
            JokerId::GreedyJoker,
            JokerId::LustyJoker,
            JokerId::WrathfulJoker,
            JokerId::GluttonousJoker,
            JokerId::JollyJoker,
            JokerId::ZanyJoker,
            JokerId::MadJoker,
            JokerId::CrazyJoker,
            JokerId::DrollJoker,
            JokerId::SlyJoker,
            JokerId::HalfJoker,
            JokerId::Banner,
            JokerId::Misprint,
            JokerId::ScaryFace,
            JokerId::AbstractJoker,
            JokerId::Scholar,
            JokerId::BusinessCard,
            JokerId::Supernova,
            JokerId::BlueJoker,
            JokerId::Fibonacci,
            JokerId::StoneJoker,
            JokerId::GoldenJoker,
            JokerId::Juggler,
            JokerId::Drunkard,
        ]
    }
}

#[derive(Debug, Clone)]
pub struct Joker {
    pub id: JokerId,
    pub edition: crate::cards::Edition,
    pub cost: i32,
    pub eternal: bool,
    pub sell_value: i32,
}

impl Joker {
    pub fn new(id: JokerId) -> Self {
        let cost = id.base_cost();
        Joker {
            id,
            edition: crate::cards::Edition::None,
            cost,
            eternal: false,
            sell_value: cost / 2,
        }
    }

    pub fn name(&self) -> &'static str {
        self.id.name()
    }

    pub fn description(&self) -> &'static str {
        self.id.description()
    }
}

#[derive(Resource, Default)]
pub struct OwnedJokers {
    pub jokers: Vec<Joker>,
    pub max_slots: usize,
}

impl OwnedJokers {
    pub fn new(max_slots: usize) -> Self {
        OwnedJokers {
            jokers: Vec::new(),
            max_slots,
        }
    }

    pub fn add(&mut self, joker: Joker) -> bool {
        if self.jokers.len() < self.max_slots {
            self.jokers.push(joker);
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<Joker> {
        if index < self.jokers.len() {
            Some(self.jokers.remove(index))
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.jokers.len()
    }

    pub fn has_space(&self) -> bool {
        self.jokers.len() < self.max_slots
    }
}
