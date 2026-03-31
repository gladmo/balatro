#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;
use rand::Rng;
use crate::cards::{Card, Enhancement, Rank, Suit};
use crate::deck::{Deck, Hand, SelectedCards, DiscardPile};
use crate::game_data::GameData;
use crate::hand_eval::{evaluate_hand, HandLevels, HandType};
use crate::jokers::{JokerId, OwnedJokers};

#[derive(Debug, Clone)]
pub struct ScoringResult {
    pub chips: u64,
    pub mult: f64,
    pub final_score: u64,
    pub money_gained: i32,
    pub hand_type: HandType,
}



pub fn score_hand(
    played_cards: &[&Card],
    hand: &[&Card],
    jokers: &OwnedJokers,
    game_data: &GameData,
    hand_levels: &HandLevels,
    deck_remaining: usize,
    discards_remaining: u32,
    rng: &mut impl Rng,
) -> ScoringResult {
    let eval = evaluate_hand(&played_cards.iter().map(|c| (*c).clone()).collect::<Vec<_>>());
    let hand_type = eval.hand_type;

    let base_chips = hand_type.base_chips() + hand_levels.chips_bonus(hand_type);
    let base_mult = hand_type.base_mult() + hand_levels.mult_bonus(hand_type);

    let mut chips = base_chips as u64;
    let mut mult = base_mult as f64;

    // Score each card in the scoring set
    for &idx in &eval.scoring_cards {
        if idx >= played_cards.len() {
            continue;
        }
        let card = played_cards[idx];
        if card.debuffed {
            continue;
        }

        // Base chip value
        chips += card.base_chip_value() as u64;

        // Enhancement effects
        match card.enhancement {
            Enhancement::BonusCard => chips += 30,
            Enhancement::MultCard => mult += 4.0,
            Enhancement::GlassCard => {
                mult *= 2.0;
                // Glass card breaks 1/4 chance - handled elsewhere
            }
            Enhancement::LuckyCard => {
                chips += 20;
                if rng.gen_bool(0.2) {
                    mult += 20.0;
                }
            }
            _ => {}
        }

        // Edition effects on scoring cards
        match card.edition {
            crate::cards::Edition::Foil => chips += 50,
            crate::cards::Edition::Holographic => mult += 10.0,
            crate::cards::Edition::Polychrome => mult *= 1.5,
            _ => {}
        }
    }

    // Steel cards in hand (non-played)
    for card in hand {
        if card.enhancement == Enhancement::SteelCard && !card.debuffed {
            mult *= 1.5;
        }
    }

    // Stone cards
    let stone_count = played_cards.iter().filter(|c| c.enhancement == Enhancement::StoneCard).count();

    // Apply joker effects
    let mut money_gained = 0i32;
    let joker_count = jokers.len();

    for joker in &jokers.jokers {
        match joker.id {
            JokerId::Joker => mult += 4.0,
            JokerId::GreedyJoker => {
                let count = played_cards.iter().filter(|c| c.suit == Suit::Diamonds && !c.debuffed).count();
                mult += 4.0 * count as f64;
            }
            JokerId::LustyJoker => {
                let count = played_cards.iter().filter(|c| c.suit == Suit::Hearts && !c.debuffed).count();
                mult += 4.0 * count as f64;
            }
            JokerId::WrathfulJoker => {
                let count = played_cards.iter().filter(|c| c.suit == Suit::Spades && !c.debuffed).count();
                mult += 4.0 * count as f64;
            }
            JokerId::GluttonousJoker => {
                let count = played_cards.iter().filter(|c| c.suit == Suit::Clubs && !c.debuffed).count();
                mult += 4.0 * count as f64;
            }
            JokerId::JollyJoker => {
                if matches!(hand_type, HandType::Pair | HandType::TwoPair | HandType::FullHouse
                    | HandType::ThreeOfAKind | HandType::FourOfAKind | HandType::FiveOfAKind
                    | HandType::FlushHouse | HandType::FlushFive) {
                    mult += 8.0;
                }
            }
            JokerId::ZanyJoker => {
                if matches!(hand_type, HandType::ThreeOfAKind | HandType::FullHouse
                    | HandType::FourOfAKind | HandType::FiveOfAKind | HandType::FlushHouse | HandType::FlushFive) {
                    mult += 12.0;
                }
            }
            JokerId::MadJoker => {
                if matches!(hand_type, HandType::FourOfAKind | HandType::FiveOfAKind | HandType::FlushFive) {
                    mult += 20.0;
                }
            }
            JokerId::CrazyJoker => {
                if matches!(hand_type, HandType::Straight | HandType::StraightFlush) {
                    mult += 12.0;
                }
            }
            JokerId::DrollJoker => {
                if matches!(hand_type, HandType::Flush | HandType::StraightFlush | HandType::FlushHouse | HandType::FlushFive) {
                    mult += 10.0;
                }
            }
            JokerId::SlyJoker => {
                if matches!(hand_type, HandType::Pair | HandType::TwoPair | HandType::FullHouse
                    | HandType::ThreeOfAKind | HandType::FourOfAKind | HandType::FiveOfAKind
                    | HandType::FlushHouse | HandType::FlushFive) {
                    chips += 50;
                }
            }
            JokerId::HalfJoker => {
                if played_cards.len() <= 3 {
                    mult += 20.0;
                }
            }
            JokerId::Banner => {
                chips += 40 * discards_remaining as u64;
            }
            JokerId::Misprint => {
                let random_mult = rng.gen_range(0..=23) as f64;
                mult += random_mult;
            }
            JokerId::ScaryFace => {
                let count = played_cards.iter().filter(|c| c.rank.is_face_card() && !c.debuffed).count();
                chips += 30 * count as u64;
            }
            JokerId::AbstractJoker => {
                mult += 3.0 * joker_count as f64;
            }
            JokerId::Scholar => {
                let ace_count = played_cards.iter().filter(|c| c.rank == Rank::Ace && !c.debuffed).count();
                chips += 20 * ace_count as u64;
                mult += 4.0 * ace_count as f64;
            }
            JokerId::BusinessCard => {
                let face_count = played_cards.iter().filter(|c| c.rank.is_face_card() && !c.debuffed).count();
                for _ in 0..face_count {
                    if rng.gen_bool(0.5) {
                        money_gained += 2;
                    }
                }
            }
            JokerId::Supernova => {
                let times = game_data.times_played_this_run(hand_type);
                mult += times as f64;
            }
            JokerId::BlueJoker => {
                chips += 2 * deck_remaining as u64;
            }
            JokerId::Fibonacci => {
                let fib_ranks = [Rank::Ace, Rank::Two, Rank::Three, Rank::Five, Rank::Eight];
                let count = played_cards.iter()
                    .filter(|c| fib_ranks.contains(&c.rank) && !c.debuffed)
                    .count();
                mult += 8.0 * count as f64;
            }
            JokerId::StoneJoker => {
                chips += 25 * stone_count as u64;
            }
            JokerId::GoldenJoker => {
                // Earns $4 at end of round - handled in end-of-round logic
            }
            JokerId::Juggler | JokerId::Drunkard => {
                // Passive effects on hand size / discards
            }
        }

        // Joker edition bonuses
        match joker.edition {
            crate::cards::Edition::Foil => chips += 50,
            crate::cards::Edition::Holographic => mult += 10.0,
            crate::cards::Edition::Polychrome => mult *= 1.5,
            _ => {}
        }
    }

    let final_score = (chips as f64 * mult) as u64;

    ScoringResult {
        chips,
        mult,
        final_score,
        money_gained,
        hand_type,
    }
}

pub fn execute_play_hand(
    game_data: &mut GameData,
    deck: &mut Deck,
    hand: &mut Hand,
    selected: &mut SelectedCards,
    discard_pile: &mut DiscardPile,
    jokers: &OwnedJokers,
    hand_levels: &mut HandLevels,
) -> Option<crate::GameState> {
    if selected.is_empty() || game_data.hands_remaining == 0 {
        return None;
    }

    let played_indices: Vec<usize> = selected.indices.clone();
    let played_cards: Vec<Card> = played_indices.iter()
        .filter_map(|&i| hand.cards.get(i).cloned())
        .collect();

    let hand_cards: Vec<&Card> = hand.cards.iter().collect();
    let played_refs: Vec<&Card> = played_cards.iter().collect();

    let mut rng = rand::thread_rng();
    let result = score_hand(
        &played_refs,
        &hand_cards,
        jokers,
        game_data,
        hand_levels,
        deck.remaining(),
        game_data.discards_remaining,
        &mut rng,
    );

    game_data.score += result.final_score;
    game_data.run_score += result.final_score;
    game_data.money += result.money_gained;
    game_data.hands_remaining -= 1;
    game_data.record_hand_play(result.hand_type);
    hand_levels.record_play(result.hand_type);

    let mut sorted_indices = played_indices.clone();
    sorted_indices.sort_by(|a, b| b.cmp(a));
    for idx in sorted_indices {
        if let Some(card) = hand.remove_at(idx) {
            discard_pile.add_card(card);
        }
    }

    selected.clear();
    crate::deck::draw_to_hand(deck, hand);

    if game_data.score >= game_data.blind_target {
        Some(crate::GameState::Shop)
    } else if game_data.hands_remaining == 0 {
        Some(crate::GameState::GameOver)
    } else {
        None
    }
}

pub fn execute_discard(
    game_data: &mut GameData,
    deck: &mut Deck,
    hand: &mut Hand,
    selected: &mut SelectedCards,
    discard_pile: &mut DiscardPile,
) {
    if selected.is_empty() || game_data.discards_remaining == 0 {
        return;
    }

    let discard_indices: Vec<usize> = selected.indices.clone();
    let mut sorted_indices = discard_indices.clone();
    sorted_indices.sort_by(|a, b| b.cmp(a));

    for idx in sorted_indices {
        if let Some(card) = hand.remove_at(idx) {
            discard_pile.add_card(card);
        }
    }

    selected.clear();
    game_data.discards_remaining -= 1;
    crate::deck::draw_to_hand(deck, hand);
}
