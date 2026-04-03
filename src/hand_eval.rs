use crate::cards::{Card, Enhancement, HandType, Rank};
use std::collections::HashMap;

/// Result of evaluating a poker hand
#[derive(Debug, Clone)]
pub struct HandEvalResult {
    pub hand_type: HandType,
    pub scoring_cards: Vec<Card>,
}

/// Evaluate the best poker hand from selected cards
pub fn evaluate_hand(cards: &[Card]) -> Option<HandEvalResult> {
    if cards.is_empty() {
        return None;
    }

    let mut ranks: Vec<Rank> = cards.iter().map(|c| c.rank).collect();
    ranks.sort();

    let rank_counts = count_ranks(cards);
    let is_flush = check_flush(cards);
    let is_straight = check_straight(cards);

    // Check from best to worst
    if cards.len() == 5 && is_flush && all_same_rank_count(&rank_counts, 5) {
        return Some(HandEvalResult {
            hand_type: HandType::FlushFive,
            scoring_cards: cards.to_vec(),
        });
    }

    if cards.len() == 5 && is_flush && has_full_house(&rank_counts) {
        return Some(HandEvalResult {
            hand_type: HandType::FlushHouse,
            scoring_cards: cards.to_vec(),
        });
    }

    if all_same_rank_count(&rank_counts, 5) {
        return Some(HandEvalResult {
            hand_type: HandType::FiveOfAKind,
            scoring_cards: cards.to_vec(),
        });
    }

    if is_flush && is_straight && has_royal(cards) {
        return Some(HandEvalResult {
            hand_type: HandType::RoyalFlush,
            scoring_cards: cards.to_vec(),
        });
    }

    if is_flush && is_straight {
        return Some(HandEvalResult {
            hand_type: HandType::StraightFlush,
            scoring_cards: cards.to_vec(),
        });
    }

    if has_n_of_a_kind(&rank_counts, 4) {
        let scoring = get_n_of_a_kind_cards(cards, &rank_counts, 4);
        return Some(HandEvalResult {
            hand_type: HandType::FourOfAKind,
            scoring_cards: scoring,
        });
    }

    if has_full_house(&rank_counts) {
        return Some(HandEvalResult {
            hand_type: HandType::FullHouse,
            scoring_cards: cards.to_vec(),
        });
    }

    if is_flush {
        return Some(HandEvalResult {
            hand_type: HandType::Flush,
            scoring_cards: cards.to_vec(),
        });
    }

    if is_straight {
        return Some(HandEvalResult {
            hand_type: HandType::Straight,
            scoring_cards: cards.to_vec(),
        });
    }

    if has_n_of_a_kind(&rank_counts, 3) {
        let scoring = get_n_of_a_kind_cards(cards, &rank_counts, 3);
        return Some(HandEvalResult {
            hand_type: HandType::ThreeOfAKind,
            scoring_cards: scoring,
        });
    }

    if count_pairs(&rank_counts) >= 2 {
        let scoring = get_pairs_cards(cards, &rank_counts);
        return Some(HandEvalResult {
            hand_type: HandType::TwoPair,
            scoring_cards: scoring,
        });
    }

    if count_pairs(&rank_counts) == 1 {
        let scoring = get_n_of_a_kind_cards(cards, &rank_counts, 2);
        return Some(HandEvalResult {
            hand_type: HandType::Pair,
            scoring_cards: scoring,
        });
    }

    // High card - only the highest card scores
    let mut sorted = cards.to_vec();
    sorted.sort_by(|a, b| b.rank.cmp(&a.rank));
    Some(HandEvalResult {
        hand_type: HandType::HighCard,
        scoring_cards: vec![sorted[0].clone()],
    })
}

/// Calculate the final score
pub fn calculate_score(
    hand_type: &HandType,
    scoring_cards: &[Card],
    base_chips: u32,
    base_mult: u32,
) -> (u64, u64) {
    let mut chips = base_chips as u64;
    let mut mult = base_mult as u64;

    for card in scoring_cards {
        if card.debuffed {
            continue;
        }
        chips += card.total_chips() as u64;
        mult += card.total_mult() as u64;

        // Edition x_mult
        let x = card.edition.x_mult();
        if x > 1.0 {
            mult = (mult as f64 * x) as u64;
        }

        // Enhancement x_mult
        let ex = card.enhancement.x_mult();
        if ex > 1.0 {
            mult = (mult as f64 * ex) as u64;
        }
    }

    let _ = hand_type; // used for base score already passed in
    (chips, mult)
}

fn count_ranks(cards: &[Card]) -> HashMap<Rank, usize> {
    let mut counts = HashMap::new();
    for card in cards {
        let rank = if card.enhancement == Enhancement::Wild {
            card.rank // Wild cards count as their rank
        } else {
            card.rank
        };
        *counts.entry(rank).or_insert(0) += 1;
    }
    counts
}

fn check_flush(cards: &[Card]) -> bool {
    if cards.len() < 5 {
        return false;
    }
    let first_suit = cards[0].suit;
    cards.iter().all(|c| {
        c.suit == first_suit || c.enhancement == Enhancement::Wild
    })
}

fn check_straight(cards: &[Card]) -> bool {
    if cards.len() < 5 {
        return false;
    }
    let mut values: Vec<u32> = cards.iter().map(|c| c.rank as u32).collect();
    values.sort();
    values.dedup();

    if values.len() < 5 {
        return false;
    }

    // Check normal straight
    for window in values.windows(5) {
        if window[4] - window[0] == 4 {
            return true;
        }
    }

    // Check ace-low straight (A,2,3,4,5)
    if values.contains(&14) && values.contains(&2) && values.contains(&3)
        && values.contains(&4) && values.contains(&5)
    {
        return true;
    }

    false
}

fn has_royal(cards: &[Card]) -> bool {
    let ranks: Vec<Rank> = cards.iter().map(|c| c.rank).collect();
    ranks.contains(&Rank::Ten)
        && ranks.contains(&Rank::Jack)
        && ranks.contains(&Rank::Queen)
        && ranks.contains(&Rank::King)
        && ranks.contains(&Rank::Ace)
}

fn all_same_rank_count(counts: &HashMap<Rank, usize>, n: usize) -> bool {
    counts.values().any(|&v| v >= n)
}

fn has_n_of_a_kind(counts: &HashMap<Rank, usize>, n: usize) -> bool {
    counts.values().any(|&v| v >= n)
}

fn has_full_house(counts: &HashMap<Rank, usize>) -> bool {
    let has_three = counts.values().any(|&v| v >= 3);
    let has_pair = counts.values().filter(|&&v| v >= 2).count() >= 2;
    has_three && has_pair
}

fn count_pairs(counts: &HashMap<Rank, usize>) -> usize {
    counts.values().filter(|&&v| v >= 2).count()
}

fn get_n_of_a_kind_cards(
    cards: &[Card],
    counts: &HashMap<Rank, usize>,
    n: usize,
) -> Vec<Card> {
    let target_rank = counts
        .iter()
        .filter(|(_, &v)| v >= n)
        .max_by_key(|(r, _)| **r)
        .map(|(r, _)| *r);

    if let Some(rank) = target_rank {
        cards.iter().filter(|c| c.rank == rank).cloned().collect()
    } else {
        Vec::new()
    }
}

fn get_pairs_cards(cards: &[Card], counts: &HashMap<Rank, usize>) -> Vec<Card> {
    let pair_ranks: Vec<Rank> = counts
        .iter()
        .filter(|(_, &v)| v >= 2)
        .map(|(r, _)| *r)
        .collect();
    cards
        .iter()
        .filter(|c| pair_ranks.contains(&c.rank))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::{Edition, Seal, Suit};

    fn make_card(rank: Rank, suit: Suit) -> Card {
        Card {
            suit,
            rank,
            enhancement: Enhancement::None,
            edition: Edition::Base,
            seal: Seal::None,
            face_up: true,
            selected: false,
            debuffed: false,
            id: 0,
        }
    }

    #[test]
    fn test_pair() {
        let cards = vec![
            make_card(Rank::Ace, Suit::Hearts),
            make_card(Rank::Ace, Suit::Spades),
            make_card(Rank::King, Suit::Diamonds),
        ];
        let result = evaluate_hand(&cards).unwrap();
        assert_eq!(result.hand_type, HandType::Pair);
        assert_eq!(result.scoring_cards.len(), 2);
    }

    #[test]
    fn test_flush() {
        let cards = vec![
            make_card(Rank::Two, Suit::Hearts),
            make_card(Rank::Five, Suit::Hearts),
            make_card(Rank::Seven, Suit::Hearts),
            make_card(Rank::Nine, Suit::Hearts),
            make_card(Rank::Jack, Suit::Hearts),
        ];
        let result = evaluate_hand(&cards).unwrap();
        assert_eq!(result.hand_type, HandType::Flush);
    }

    #[test]
    fn test_straight() {
        let cards = vec![
            make_card(Rank::Five, Suit::Hearts),
            make_card(Rank::Six, Suit::Diamonds),
            make_card(Rank::Seven, Suit::Clubs),
            make_card(Rank::Eight, Suit::Spades),
            make_card(Rank::Nine, Suit::Hearts),
        ];
        let result = evaluate_hand(&cards).unwrap();
        assert_eq!(result.hand_type, HandType::Straight);
    }

    #[test]
    fn test_full_house() {
        let cards = vec![
            make_card(Rank::King, Suit::Hearts),
            make_card(Rank::King, Suit::Diamonds),
            make_card(Rank::King, Suit::Clubs),
            make_card(Rank::Queen, Suit::Hearts),
            make_card(Rank::Queen, Suit::Spades),
        ];
        let result = evaluate_hand(&cards).unwrap();
        assert_eq!(result.hand_type, HandType::FullHouse);
    }

    #[test]
    fn test_royal_flush() {
        let cards = vec![
            make_card(Rank::Ten, Suit::Spades),
            make_card(Rank::Jack, Suit::Spades),
            make_card(Rank::Queen, Suit::Spades),
            make_card(Rank::King, Suit::Spades),
            make_card(Rank::Ace, Suit::Spades),
        ];
        let result = evaluate_hand(&cards).unwrap();
        assert_eq!(result.hand_type, HandType::RoyalFlush);
    }

    #[test]
    fn test_high_card() {
        let cards = vec![
            make_card(Rank::Two, Suit::Hearts),
            make_card(Rank::Five, Suit::Diamonds),
            make_card(Rank::Nine, Suit::Clubs),
        ];
        let result = evaluate_hand(&cards).unwrap();
        assert_eq!(result.hand_type, HandType::HighCard);
        assert_eq!(result.scoring_cards.len(), 1);
    }
}
