#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;
use rand::Rng;
use rand::seq::SliceRandom;
use crate::jokers::{Joker, JokerId};
use crate::game_data::GameData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TarotCard {
    Magician,
    Empress,
    HighPriestess,
    Emperor,
    Hierophant,
    Lovers,
    Chariot,
    Justice,
    Hermit,
    Strength,
    HangedMan,
    Death,
    Devil,
    Tower,
    Judgement,
}

impl TarotCard {
    pub fn name(&self) -> &'static str {
        match self {
            TarotCard::Magician => "The Magician",
            TarotCard::Empress => "The Empress",
            TarotCard::HighPriestess => "The High Priestess",
            TarotCard::Emperor => "The Emperor",
            TarotCard::Hierophant => "The Hierophant",
            TarotCard::Lovers => "The Lovers",
            TarotCard::Chariot => "The Chariot",
            TarotCard::Justice => "Justice",
            TarotCard::Hermit => "The Hermit",
            TarotCard::Strength => "Strength",
            TarotCard::HangedMan => "The Hanged Man",
            TarotCard::Death => "Death",
            TarotCard::Devil => "The Devil",
            TarotCard::Tower => "The Tower",
            TarotCard::Judgement => "Judgement",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            TarotCard::Magician => "Add Lucky enhancement to 1 card",
            TarotCard::Empress => "Add Mult enhancement to 1-2 cards",
            TarotCard::HighPriestess => "Create 2 planet cards",
            TarotCard::Emperor => "Create 2 tarot cards",
            TarotCard::Hierophant => "Add Bonus enhancement to 1-2 cards",
            TarotCard::Lovers => "Add Wild enhancement to 1 card",
            TarotCard::Chariot => "Add Steel enhancement to 1 card",
            TarotCard::Justice => "Add Glass enhancement to 1 card",
            TarotCard::Hermit => "Double money (max $20)",
            TarotCard::Strength => "Increase rank of 1-2 cards by 1",
            TarotCard::HangedMan => "Destroy 1-2 cards",
            TarotCard::Death => "Copy left card enhancement to right card",
            TarotCard::Devil => "Add Gold enhancement to 1-2 cards",
            TarotCard::Tower => "Add Stone enhancement to 1-2 cards",
            TarotCard::Judgement => "Create random joker",
        }
    }

    pub fn all() -> Vec<TarotCard> {
        vec![
            TarotCard::Magician, TarotCard::Empress, TarotCard::HighPriestess,
            TarotCard::Emperor, TarotCard::Hierophant, TarotCard::Lovers,
            TarotCard::Chariot, TarotCard::Justice, TarotCard::Hermit,
            TarotCard::Strength, TarotCard::HangedMan, TarotCard::Death,
            TarotCard::Devil, TarotCard::Tower, TarotCard::Judgement,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlanetCard {
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
    Pluto,
    PlanetX,
    Ceres,
    Eris,
}

impl PlanetCard {
    pub fn name(&self) -> &'static str {
        match self {
            PlanetCard::Mercury => "Mercury",
            PlanetCard::Venus => "Venus",
            PlanetCard::Earth => "Earth",
            PlanetCard::Mars => "Mars",
            PlanetCard::Jupiter => "Jupiter",
            PlanetCard::Saturn => "Saturn",
            PlanetCard::Uranus => "Uranus",
            PlanetCard::Neptune => "Neptune",
            PlanetCard::Pluto => "Pluto",
            PlanetCard::PlanetX => "Planet X",
            PlanetCard::Ceres => "Ceres",
            PlanetCard::Eris => "Eris",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            PlanetCard::Mercury => "Level up Pair",
            PlanetCard::Venus => "Level up Three of a Kind",
            PlanetCard::Earth => "Level up Full House",
            PlanetCard::Mars => "Level up Four of a Kind",
            PlanetCard::Jupiter => "Level up Flush",
            PlanetCard::Saturn => "Level up Straight",
            PlanetCard::Uranus => "Level up Two Pair",
            PlanetCard::Neptune => "Level up Straight Flush",
            PlanetCard::Pluto => "Level up High Card",
            PlanetCard::PlanetX => "Level up Five of a Kind",
            PlanetCard::Ceres => "Level up Flush House",
            PlanetCard::Eris => "Level up Flush Five",
        }
    }

    pub fn hand_type(&self) -> crate::hand_eval::HandType {
        match self {
            PlanetCard::Mercury => crate::hand_eval::HandType::Pair,
            PlanetCard::Venus => crate::hand_eval::HandType::ThreeOfAKind,
            PlanetCard::Earth => crate::hand_eval::HandType::FullHouse,
            PlanetCard::Mars => crate::hand_eval::HandType::FourOfAKind,
            PlanetCard::Jupiter => crate::hand_eval::HandType::Flush,
            PlanetCard::Saturn => crate::hand_eval::HandType::Straight,
            PlanetCard::Uranus => crate::hand_eval::HandType::TwoPair,
            PlanetCard::Neptune => crate::hand_eval::HandType::StraightFlush,
            PlanetCard::Pluto => crate::hand_eval::HandType::HighCard,
            PlanetCard::PlanetX => crate::hand_eval::HandType::FiveOfAKind,
            PlanetCard::Ceres => crate::hand_eval::HandType::FlushHouse,
            PlanetCard::Eris => crate::hand_eval::HandType::FlushFive,
        }
    }

    pub fn all() -> Vec<PlanetCard> {
        vec![
            PlanetCard::Mercury, PlanetCard::Venus, PlanetCard::Earth,
            PlanetCard::Mars, PlanetCard::Jupiter, PlanetCard::Saturn,
            PlanetCard::Uranus, PlanetCard::Neptune, PlanetCard::Pluto,
            PlanetCard::PlanetX, PlanetCard::Ceres, PlanetCard::Eris,
        ]
    }
}

#[derive(Debug, Clone)]
pub enum Consumable {
    Tarot(TarotCard),
    Planet(PlanetCard),
}

impl Consumable {
    pub fn name(&self) -> &'static str {
        match self {
            Consumable::Tarot(t) => t.name(),
            Consumable::Planet(p) => p.name(),
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Consumable::Tarot(t) => t.description(),
            Consumable::Planet(p) => p.description(),
        }
    }

    pub fn cost(&self) -> i32 {
        3
    }
}

#[derive(Resource, Default)]
pub struct ConsumableSlots {
    pub slots: Vec<Consumable>,
    pub max_slots: usize,
}

impl ConsumableSlots {
    pub fn new(max_slots: usize) -> Self {
        ConsumableSlots {
            slots: Vec::new(),
            max_slots,
        }
    }

    pub fn add(&mut self, consumable: Consumable) -> bool {
        if self.slots.len() < self.max_slots {
            self.slots.push(consumable);
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<Consumable> {
        if index < self.slots.len() {
            Some(self.slots.remove(index))
        } else {
            None
        }
    }
}

pub struct UseConsumableEvent {
    pub index: usize,
    pub selected_card_indices: Vec<usize>,
}

pub fn random_tarot(rng: &mut impl Rng) -> TarotCard {
    let all = TarotCard::all();
    *all.choose(rng).unwrap()
}

pub fn random_planet(rng: &mut impl Rng) -> PlanetCard {
    let all = PlanetCard::all();
    *all.choose(rng).unwrap()
}
