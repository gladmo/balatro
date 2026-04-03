use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::cards::{Card, HandType, JokerCard};

/// Main game states
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    BlindSelect,
    Playing,
    Shop,
    GameOver,
}

/// Deck types available
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeckType {
    RedDeck,
    BlueDeck,
    YellowDeck,
    GreenDeck,
    BlackDeck,
    MagicDeck,
    NebulaDeck,
    GhostDeck,
    AbandonedDeck,
    CheckeredDeck,
    ZodiacDeck,
    PaintedDeck,
    AnnageDeck,
    PlasmaFeck,
    ErrataFeck,
}

impl Default for DeckType {
    fn default() -> Self {
        DeckType::RedDeck
    }
}

/// Tracks leveled-up hand types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandLevel {
    pub hand_type: HandType,
    pub level: u32,
    pub base_chips: u32,
    pub base_mult: u32,
}

impl HandLevel {
    pub fn new(hand_type: HandType) -> Self {
        let (chips, mult) = hand_type.base_score();
        Self {
            hand_type,
            level: 1,
            base_chips: chips,
            base_mult: mult,
        }
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        let (chip_add, mult_add) = self.hand_type.level_up_amount();
        self.base_chips += chip_add;
        self.base_mult += mult_add;
    }
}

/// Main game data resource
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GameData {
    pub ante: u32,
    pub round: u32,
    pub money: i32,
    pub score: u64,
    pub hands_remaining: u32,
    pub discards_remaining: u32,
    pub hand_size: u32,
    pub joker_slots: u32,
    pub consumable_slots: u32,
    pub deck_type: DeckType,
    pub hand_levels: Vec<HandLevel>,
    pub deck: Vec<Card>,
    pub hand: Vec<Card>,
    pub played_cards: Vec<Card>,
    pub jokers: Vec<JokerCard>,
    pub consumables: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub blind_chips_required: u64,
    pub blind_chips_scored: u64,
    pub current_blind: BlindType,
    pub game_over: bool,
    pub game_won: bool,
    pub reroll_cost: u32,
    pub interest_cap: u32,
}

impl Default for GameData {
    fn default() -> Self {
        let hand_levels = vec![
            HandLevel::new(HandType::HighCard),
            HandLevel::new(HandType::Pair),
            HandLevel::new(HandType::TwoPair),
            HandLevel::new(HandType::ThreeOfAKind),
            HandLevel::new(HandType::Straight),
            HandLevel::new(HandType::Flush),
            HandLevel::new(HandType::FullHouse),
            HandLevel::new(HandType::FourOfAKind),
            HandLevel::new(HandType::StraightFlush),
            HandLevel::new(HandType::RoyalFlush),
            HandLevel::new(HandType::FiveOfAKind),
            HandLevel::new(HandType::FlushHouse),
            HandLevel::new(HandType::FlushFive),
        ];
        Self {
            ante: 1,
            round: 1,
            money: 4,
            score: 0,
            hands_remaining: 4,
            discards_remaining: 3,
            hand_size: 8,
            joker_slots: 5,
            consumable_slots: 2,
            deck_type: DeckType::default(),
            hand_levels,
            deck: Vec::new(),
            hand: Vec::new(),
            played_cards: Vec::new(),
            jokers: Vec::new(),
            consumables: Vec::new(),
            discard_pile: Vec::new(),
            blind_chips_required: 300,
            blind_chips_scored: 0,
            current_blind: BlindType::SmallBlind,
            game_over: false,
            game_won: false,
            reroll_cost: 5,
            interest_cap: 25,
        }
    }
}

impl GameData {
    pub fn new_run(&mut self) {
        *self = Self::default();
        self.build_deck();
        self.shuffle_deck();
    }

    pub fn build_deck(&mut self) {
        use crate::cards::{Enhancement, Rank, Seal, Suit};
        self.deck.clear();
        let suits = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
        let ranks = [
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
        ];
        for &suit in &suits {
            for &rank in &ranks {
                self.deck.push(Card {
                    suit,
                    rank,
                    enhancement: Enhancement::None,
                    edition: crate::cards::Edition::Base,
                    seal: Seal::None,
                    face_up: true,
                    selected: false,
                    debuffed: false,
                    id: self.deck.len() as u32,
                });
            }
        }
    }

    pub fn shuffle_deck(&mut self) {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        self.deck.shuffle(&mut rng);
    }

    pub fn draw_hand(&mut self) {
        let draw_count = self.hand_size as usize;
        let available = self.deck.len().min(draw_count);
        for _ in 0..available {
            if let Some(card) = self.deck.pop() {
                self.hand.push(card);
            }
        }
    }

    pub fn discard_selected(&mut self) {
        let (selected, remaining): (Vec<Card>, Vec<Card>) =
            self.hand.drain(..).partition(|c| c.selected);
        self.hand = remaining;
        for mut card in selected {
            card.selected = false;
            self.discard_pile.push(card);
        }
    }

    pub fn return_hand_to_deck(&mut self) {
        while let Some(mut card) = self.hand.pop() {
            card.selected = false;
            self.deck.push(card);
        }
        while let Some(mut card) = self.played_cards.pop() {
            card.selected = false;
            self.deck.push(card);
        }
    }

    pub fn get_hand_level(&self, hand_type: &HandType) -> &HandLevel {
        self.hand_levels
            .iter()
            .find(|hl| hl.hand_type == *hand_type)
            .unwrap_or(&self.hand_levels[0])
    }

    pub fn calculate_earnings(&self) -> i32 {
        let blind_reward = match self.current_blind {
            BlindType::SmallBlind => 3,
            BlindType::BigBlind => 4,
            BlindType::BossBlind => 5,
        };
        let interest = (self.money / 5).min(self.interest_cap as i32);
        blind_reward + interest
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlindType {
    SmallBlind,
    BigBlind,
    BossBlind,
}

impl Default for BlindType {
    fn default() -> Self {
        BlindType::SmallBlind
    }
}

impl std::fmt::Display for BlindType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlindType::SmallBlind => write!(f, "Small Blind"),
            BlindType::BigBlind => write!(f, "Big Blind"),
            BlindType::BossBlind => write!(f, "Boss Blind"),
        }
    }
}
