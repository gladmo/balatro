#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;
use std::collections::HashMap;
use crate::cards::{Card, Rank, Suit};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    FiveOfAKind,
    FlushHouse,
    FlushFive,
}

impl HandType {
    pub fn name(&self) -> &'static str {
        match self {
            HandType::HighCard => "High Card",
            HandType::Pair => "Pair",
            HandType::TwoPair => "Two Pair",
            HandType::ThreeOfAKind => "Three of a Kind",
            HandType::Straight => "Straight",
            HandType::Flush => "Flush",
            HandType::FullHouse => "Full House",
            HandType::FourOfAKind => "Four of a Kind",
            HandType::StraightFlush => "Straight Flush",
            HandType::FiveOfAKind => "Five of a Kind",
            HandType::FlushHouse => "Flush House",
            HandType::FlushFive => "Flush Five",
        }
    }

    pub fn loc_key(&self) -> &'static str {
        match self {
            HandType::HighCard        => "hand.high_card",
            HandType::Pair            => "hand.pair",
            HandType::TwoPair         => "hand.two_pair",
            HandType::ThreeOfAKind    => "hand.three_of_a_kind",
            HandType::Straight        => "hand.straight",
            HandType::Flush           => "hand.flush",
            HandType::FullHouse       => "hand.full_house",
            HandType::FourOfAKind     => "hand.four_of_a_kind",
            HandType::StraightFlush   => "hand.straight_flush",
            HandType::FiveOfAKind     => "hand.five_of_a_kind",
            HandType::FlushHouse      => "hand.flush_house",
            HandType::FlushFive       => "hand.flush_five",
        }
    }

    pub fn base_chips(&self) -> u32 {
        match self {
            HandType::HighCard => 5,
            HandType::Pair => 10,
            HandType::TwoPair => 20,
            HandType::ThreeOfAKind => 30,
            HandType::Straight => 30,
            HandType::Flush => 35,
            HandType::FullHouse => 40,
            HandType::FourOfAKind => 60,
            HandType::StraightFlush => 100,
            HandType::FiveOfAKind => 120,
            HandType::FlushHouse => 140,
            HandType::FlushFive => 160,
        }
    }

    pub fn base_mult(&self) -> u32 {
        match self {
            HandType::HighCard => 1,
            HandType::Pair => 2,
            HandType::TwoPair => 2,
            HandType::ThreeOfAKind => 3,
            HandType::Straight => 4,
            HandType::Flush => 4,
            HandType::FullHouse => 4,
            HandType::FourOfAKind => 7,
            HandType::StraightFlush => 8,
            HandType::FiveOfAKind => 12,
            HandType::FlushHouse => 14,
            HandType::FlushFive => 16,
        }
    }

    pub fn all() -> Vec<HandType> {
        vec![
            HandType::HighCard,
            HandType::Pair,
            HandType::TwoPair,
            HandType::ThreeOfAKind,
            HandType::Straight,
            HandType::Flush,
            HandType::FullHouse,
            HandType::FourOfAKind,
            HandType::StraightFlush,
            HandType::FiveOfAKind,
            HandType::FlushHouse,
            HandType::FlushFive,
        ]
    }
}

#[derive(Debug, Clone)]
pub struct EvaluationResult {
    pub hand_type: HandType,
    pub scoring_cards: Vec<usize>,
    pub other_cards: Vec<usize>,
}

#[derive(Resource, Default)]
pub struct HandLevels {
    pub levels: HashMap<HandType, u32>,
    pub times_played: HashMap<HandType, u32>,
}

impl HandLevels {
    pub fn new() -> Self {
        let mut levels = HashMap::new();
        let mut times_played = HashMap::new();
        for ht in HandType::all() {
            levels.insert(ht, 1);
            times_played.insert(ht, 0);
        }
        HandLevels { levels, times_played }
    }

    pub fn level_of(&self, hand_type: HandType) -> u32 {
        *self.levels.get(&hand_type).unwrap_or(&1)
    }

    pub fn times_played(&self, hand_type: HandType) -> u32 {
        *self.times_played.get(&hand_type).unwrap_or(&0)
    }

    pub fn level_up(&mut self, hand_type: HandType) {
        *self.levels.entry(hand_type).or_insert(1) += 1;
    }

    pub fn record_play(&mut self, hand_type: HandType) {
        *self.times_played.entry(hand_type).or_insert(0) += 1;
    }

    pub fn chips_bonus(&self, hand_type: HandType) -> u32 {
        let level = self.level_of(hand_type).saturating_sub(1);
        match hand_type {
            HandType::HighCard => level * 10,
            HandType::Pair => level * 15,
            HandType::TwoPair => level * 20,
            HandType::ThreeOfAKind => level * 20,
            HandType::Straight => level * 30,
            HandType::Flush => level * 15,
            HandType::FullHouse => level * 25,
            HandType::FourOfAKind => level * 30,
            HandType::StraightFlush => level * 40,
            HandType::FiveOfAKind => level * 35,
            HandType::FlushHouse => level * 40,
            HandType::FlushFive => level * 50,
        }
    }

    pub fn mult_bonus(&self, hand_type: HandType) -> u32 {
        let level = self.level_of(hand_type).saturating_sub(1);
        match hand_type {
            HandType::HighCard => level * 1,
            HandType::Pair => level * 1,
            HandType::TwoPair => level * 1,
            HandType::ThreeOfAKind => level * 2,
            HandType::Straight => level * 3,
            HandType::Flush => level * 2,
            HandType::FullHouse => level * 2,
            HandType::FourOfAKind => level * 3,
            HandType::StraightFlush => level * 4,
            HandType::FiveOfAKind => level * 3,
            HandType::FlushHouse => level * 4,
            HandType::FlushFive => level * 5,
        }
    }
}

pub fn evaluate_hand(cards: &[Card]) -> EvaluationResult {
    if cards.is_empty() {
        return EvaluationResult {
            hand_type: HandType::HighCard,
            scoring_cards: vec![],
            other_cards: vec![],
        };
    }

    let indices: Vec<usize> = (0..cards.len()).collect();

    // Count ranks
    let mut rank_counts: HashMap<Rank, Vec<usize>> = HashMap::new();
    let mut suit_counts: HashMap<Suit, Vec<usize>> = HashMap::new();

    for (i, card) in cards.iter().enumerate() {
        rank_counts.entry(card.rank).or_default().push(i);
        suit_counts.entry(card.suit).or_default().push(i);
    }

    let mut counts: Vec<(usize, Rank, Vec<usize>)> = rank_counts
        .iter()
        .map(|(r, idxs)| (idxs.len(), *r, idxs.clone()))
        .collect();
    counts.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));

    let is_flush = suit_counts.values().any(|v| v.len() >= 5)
        || (cards.len() == 5 && suit_counts.len() == 1)
        || (cards.len() < 5 && suit_counts.len() == 1);

    let is_flush_all = cards.len() >= 1 && suit_counts.len() == 1;

    let is_straight = check_straight(cards);

    let has_five = counts[0].0 >= 5;
    let has_four = counts[0].0 >= 4;
    let has_three = counts[0].0 >= 3;
    let has_two = counts[0].0 >= 2;
    let has_second_two = counts.len() >= 2 && counts[1].0 >= 2;
    let has_second_three = counts.len() >= 2 && counts[1].0 >= 3;

    // Flush Five: 5 of same rank and suit
    if is_flush_all && has_five {
        let scoring: Vec<usize> = counts[0].2.iter().take(5).cloned().collect();
        let other: Vec<usize> = indices.iter().filter(|&&i| !scoring.contains(&i)).cloned().collect();
        return EvaluationResult { hand_type: HandType::FlushFive, scoring_cards: scoring, other_cards: other };
    }

    // Flush House: full house and flush
    if is_flush_all && has_three && has_second_two {
        let scoring: Vec<usize> = counts[0].2.iter().chain(counts[1].2.iter()).cloned().collect();
        let other: Vec<usize> = indices.iter().filter(|&&i| !scoring.contains(&i)).cloned().collect();
        return EvaluationResult { hand_type: HandType::FlushHouse, scoring_cards: scoring, other_cards: other };
    }

    // Five of a Kind
    if has_five {
        let scoring: Vec<usize> = counts[0].2.iter().take(5).cloned().collect();
        let other: Vec<usize> = indices.iter().filter(|&&i| !scoring.contains(&i)).cloned().collect();
        return EvaluationResult { hand_type: HandType::FiveOfAKind, scoring_cards: scoring, other_cards: other };
    }

    // Straight Flush
    if is_straight && is_flush_all {
        let scoring: Vec<usize> = indices.clone();
        return EvaluationResult { hand_type: HandType::StraightFlush, scoring_cards: scoring, other_cards: vec![] };
    }

    // Four of a Kind
    if has_four {
        let scoring: Vec<usize> = counts[0].2.iter().take(4).cloned().collect();
        let other: Vec<usize> = indices.iter().filter(|&&i| !scoring.contains(&i)).cloned().collect();
        return EvaluationResult { hand_type: HandType::FourOfAKind, scoring_cards: scoring, other_cards: other };
    }

    // Full House
    if has_three && has_second_two {
        let scoring: Vec<usize> = counts[0].2.iter().chain(counts[1].2.iter()).cloned().collect();
        let other: Vec<usize> = indices.iter().filter(|&&i| !scoring.contains(&i)).cloned().collect();
        return EvaluationResult { hand_type: HandType::FullHouse, scoring_cards: scoring, other_cards: other };
    }

    // Flush
    if is_flush_all && cards.len() >= 5 {
        let scoring: Vec<usize> = indices.clone();
        return EvaluationResult { hand_type: HandType::Flush, scoring_cards: scoring, other_cards: vec![] };
    }

    // Straight
    if is_straight {
        let scoring: Vec<usize> = indices.clone();
        return EvaluationResult { hand_type: HandType::Straight, scoring_cards: scoring, other_cards: vec![] };
    }

    // Three of a Kind
    if has_three {
        let scoring: Vec<usize> = counts[0].2.clone();
        let other: Vec<usize> = indices.iter().filter(|&&i| !scoring.contains(&i)).cloned().collect();
        return EvaluationResult { hand_type: HandType::ThreeOfAKind, scoring_cards: scoring, other_cards: other };
    }

    // Two Pair
    if has_two && has_second_two {
        let scoring: Vec<usize> = counts[0].2.iter().chain(counts[1].2.iter()).cloned().collect();
        let other: Vec<usize> = indices.iter().filter(|&&i| !scoring.contains(&i)).cloned().collect();
        return EvaluationResult { hand_type: HandType::TwoPair, scoring_cards: scoring, other_cards: other };
    }

    // Pair
    if has_two {
        let scoring: Vec<usize> = counts[0].2.clone();
        let other: Vec<usize> = indices.iter().filter(|&&i| !scoring.contains(&i)).cloned().collect();
        return EvaluationResult { hand_type: HandType::Pair, scoring_cards: scoring, other_cards: other };
    }

    // High Card
    let best_idx = cards.iter().enumerate()
        .max_by_key(|(_, c)| c.rank)
        .map(|(i, _)| i)
        .unwrap_or(0);
    let scoring = vec![best_idx];
    let other: Vec<usize> = indices.iter().filter(|&&i| i != best_idx).cloned().collect();
    EvaluationResult { hand_type: HandType::HighCard, scoring_cards: scoring, other_cards: other }
}

fn check_straight(cards: &[Card]) -> bool {
    if cards.len() < 5 {
        return false;
    }
    let mut values: Vec<u8> = cards.iter().map(|c| c.rank.numeric_value()).collect();
    values.sort();
    values.dedup();
    if values.len() < 5 {
        return false;
    }

    // Check for 5 consecutive
    for window in values.windows(5) {
        if window[4] - window[0] == 4 {
            return true;
        }
    }

    // Ace-low straight: A-2-3-4-5
    if values.contains(&14) {
        let mut low_values = values.clone();
        low_values.retain(|&v| v != 14);
        low_values.insert(0, 1);
        low_values.sort();
        for window in low_values.windows(5) {
            if window[4] - window[0] == 4 {
                return true;
            }
        }
    }

    false
}
