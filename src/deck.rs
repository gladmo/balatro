#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use crate::cards::{Card, create_standard_deck};

#[derive(Resource, Default)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut deck = Deck {
            cards: create_standard_deck(),
        };
        deck
    }

    pub fn shuffle(&mut self, rng: &mut impl Rng) {
        self.cards.shuffle(rng);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn remaining(&self) -> usize {
        self.cards.len()
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.insert(0, card);
    }

    pub fn remove_card_by_id(&mut self, id: u32) -> Option<Card> {
        if let Some(pos) = self.cards.iter().position(|c| c.id == id) {
            Some(self.cards.remove(pos))
        } else {
            None
        }
    }
}

#[derive(Resource, Default)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub max_size: usize,
}

impl Hand {
    pub fn new(max_size: usize) -> Self {
        Hand {
            cards: Vec::new(),
            max_size,
        }
    }

    pub fn add_card(&mut self, card: Card) -> bool {
        if self.cards.len() < self.max_size {
            self.cards.push(card);
            true
        } else {
            false
        }
    }

    pub fn remove_at(&mut self, index: usize) -> Option<Card> {
        if index < self.cards.len() {
            Some(self.cards.remove(index))
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_full(&self) -> bool {
        self.cards.len() >= self.max_size
    }

    pub fn clear(&mut self) -> Vec<Card> {
        let cards = self.cards.drain(..).collect();
        cards
    }
}

#[derive(Resource, Default)]
pub struct SelectedCards {
    pub indices: Vec<usize>,
}

impl SelectedCards {
    pub fn toggle(&mut self, index: usize) {
        if let Some(pos) = self.indices.iter().position(|&i| i == index) {
            self.indices.remove(pos);
        } else if self.indices.len() < 5 {
            self.indices.push(index);
            self.indices.sort();
        }
    }

    pub fn clear(&mut self) {
        self.indices.clear();
    }

    pub fn contains(&self, index: usize) -> bool {
        self.indices.contains(&index)
    }

    pub fn len(&self) -> usize {
        self.indices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }
}

#[derive(Resource, Default)]
pub struct DiscardPile {
    pub cards: Vec<Card>,
}

impl DiscardPile {
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}

pub fn draw_to_hand(deck: &mut Deck, hand: &mut Hand) {
    while !hand.is_full() && deck.remaining() > 0 {
        if let Some(card) = deck.draw() {
            hand.add_card(card);
        }
    }
}

pub fn shuffle_deck(deck: &mut Deck) {
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);
}
