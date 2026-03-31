#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    pub fn all() -> [Suit; 4] {
        [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades]
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Clubs => "Clubs",
            Suit::Spades => "Spades",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub fn all() -> [Rank; 13] {
        [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ]
    }

    pub fn chip_value(&self) -> u32 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
            Rank::Ace => 11,
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Rank::Two => "Two",
            Rank::Three => "Three",
            Rank::Four => "Four",
            Rank::Five => "Five",
            Rank::Six => "Six",
            Rank::Seven => "Seven",
            Rank::Eight => "Eight",
            Rank::Nine => "Nine",
            Rank::Ten => "Ten",
            Rank::Jack => "Jack",
            Rank::Queen => "Queen",
            Rank::King => "King",
            Rank::Ace => "Ace",
        }
    }

    pub fn is_face_card(&self) -> bool {
        matches!(self, Rank::Jack | Rank::Queen | Rank::King)
    }

    pub fn numeric_value(&self) -> u8 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }

    pub fn next_rank(&self) -> Option<Rank> {
        match self {
            Rank::Two => Some(Rank::Three),
            Rank::Three => Some(Rank::Four),
            Rank::Four => Some(Rank::Five),
            Rank::Five => Some(Rank::Six),
            Rank::Six => Some(Rank::Seven),
            Rank::Seven => Some(Rank::Eight),
            Rank::Eight => Some(Rank::Nine),
            Rank::Nine => Some(Rank::Ten),
            Rank::Ten => Some(Rank::Jack),
            Rank::Jack => Some(Rank::Queen),
            Rank::Queen => Some(Rank::King),
            Rank::King => Some(Rank::Ace),
            Rank::Ace => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Enhancement {
    #[default]
    None,
    BonusCard,
    MultCard,
    WildCard,
    GlassCard,
    SteelCard,
    StoneCard,
    GoldCard,
    LuckyCard,
}

impl Enhancement {
    pub fn name(&self) -> &'static str {
        match self {
            Enhancement::None => "None",
            Enhancement::BonusCard => "Bonus",
            Enhancement::MultCard => "Mult",
            Enhancement::WildCard => "Wild",
            Enhancement::GlassCard => "Glass",
            Enhancement::SteelCard => "Steel",
            Enhancement::StoneCard => "Stone",
            Enhancement::GoldCard => "Gold",
            Enhancement::LuckyCard => "Lucky",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Edition {
    #[default]
    None,
    Foil,
    Holographic,
    Polychrome,
}

impl Edition {
    pub fn name(&self) -> &'static str {
        match self {
            Edition::None => "None",
            Edition::Foil => "Foil",
            Edition::Holographic => "Holographic",
            Edition::Polychrome => "Polychrome",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Seal {
    #[default]
    None,
    Gold,
    Red,
    Blue,
    Purple,
}

impl Seal {
    pub fn name(&self) -> &'static str {
        match self {
            Seal::None => "None",
            Seal::Gold => "Gold Seal",
            Seal::Red => "Red Seal",
            Seal::Blue => "Blue Seal",
            Seal::Purple => "Purple Seal",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
    pub enhancement: Enhancement,
    pub edition: Edition,
    pub seal: Seal,
    pub face_down: bool,
    pub debuffed: bool,
    pub id: u32,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank, id: u32) -> Self {
        Card {
            suit,
            rank,
            enhancement: Enhancement::None,
            edition: Edition::None,
            seal: Seal::None,
            face_down: false,
            debuffed: false,
            id,
        }
    }

    pub fn base_chip_value(&self) -> u32 {
        match self.enhancement {
            Enhancement::StoneCard => 50,
            _ => self.rank.chip_value(),
        }
    }

    pub fn display_name(&self) -> String {
        format!("{} of {}", self.rank.name(), self.suit.name())
    }

    pub fn short_display(&self) -> String {
        format!("{}{}", self.rank.short_name(), self.suit.symbol())
    }

    pub fn effective_suit(&self, treat_as_suit: Option<Suit>) -> Suit {
        if self.enhancement == Enhancement::WildCard {
            treat_as_suit.unwrap_or(self.suit)
        } else {
            self.suit
        }
    }
}

pub fn create_standard_deck() -> Vec<Card> {
    let mut cards = Vec::new();
    let mut id = 0u32;
    for suit in Suit::all() {
        for rank in Rank::all() {
            cards.push(Card::new(suit, rank, id));
            id += 1;
        }
    }
    cards
}
