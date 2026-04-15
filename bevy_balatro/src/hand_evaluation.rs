use crate::card::{Enhancement, PlayingCardData, Rank, Suit};

// ── Hand Names ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PokerHand {
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

impl PokerHand {
    pub fn name(&self) -> &'static str {
        match self {
            PokerHand::HighCard => "High Card",
            PokerHand::Pair => "Pair",
            PokerHand::TwoPair => "Two Pair",
            PokerHand::ThreeOfAKind => "Three of a Kind",
            PokerHand::Straight => "Straight",
            PokerHand::Flush => "Flush",
            PokerHand::FullHouse => "Full House",
            PokerHand::FourOfAKind => "Four of a Kind",
            PokerHand::StraightFlush => "Straight Flush",
            PokerHand::FiveOfAKind => "Five of a Kind",
            PokerHand::FlushHouse => "Flush House",
            PokerHand::FlushFive => "Flush Five",
        }
    }

    /// Base (level-1) chips and mult.
    pub fn base_chips_mult(&self) -> (u32, u32) {
        match self {
            PokerHand::HighCard      => (5,   1),
            PokerHand::Pair          => (10,  2),
            PokerHand::TwoPair       => (20,  2),
            PokerHand::ThreeOfAKind  => (30,  3),
            PokerHand::Straight      => (30,  4),
            PokerHand::Flush         => (35,  4),
            PokerHand::FullHouse     => (40,  4),
            PokerHand::FourOfAKind   => (60,  7),
            PokerHand::StraightFlush => (100, 8),
            PokerHand::FiveOfAKind   => (120, 12),
            PokerHand::FlushHouse    => (140, 14),
            PokerHand::FlushFive     => (160, 16),
        }
    }

    /// Extra chips and mult gained per level-up (via Planet cards).
    pub fn level_bonus(&self) -> (u32, u32) {
        match self {
            PokerHand::HighCard      => (10, 1),
            PokerHand::Pair          => (15, 1),
            PokerHand::TwoPair       => (20, 1),
            PokerHand::ThreeOfAKind  => (20, 2),
            PokerHand::Straight      => (30, 3),
            PokerHand::Flush         => (15, 2),
            PokerHand::FullHouse     => (25, 2),
            PokerHand::FourOfAKind   => (30, 3),
            PokerHand::StraightFlush => (40, 4),
            PokerHand::FiveOfAKind   => (35, 3),
            PokerHand::FlushHouse    => (40, 4),
            PokerHand::FlushFive     => (50, 6),
        }
    }
}

// ── Hand Evaluation Result ────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct HandResult {
    pub hand: PokerHand,
    /// The scoring cards (subset of played hand).
    pub scoring_cards: Vec<usize>,
}

// ── Helper Functions ──────────────────────────────────────────────────────

fn rank_counts(cards: &[&PlayingCardData]) -> Vec<(Rank, Vec<usize>)> {
    let mut counts: Vec<(Rank, Vec<usize>)> = Vec::new();
    for (i, c) in cards.iter().enumerate() {
        if c.enhancement == Enhancement::Stone {
            continue;
        }
        if let Some(entry) = counts.iter_mut().find(|(r, _)| *r == c.rank) {
            entry.1.push(i);
        } else {
            counts.push((c.rank, vec![i]));
        }
    }
    counts.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then(b.0.cmp(&a.0)));
    counts
}

fn suit_counts(cards: &[&PlayingCardData]) -> Vec<(Suit, Vec<usize>)> {
    let mut counts: Vec<(Suit, Vec<usize>)> = Vec::new();
    for (i, c) in cards.iter().enumerate() {
        let suit = if c.enhancement == Enhancement::Wild {
            // wild cards count for all suits; use Hearts as canonical
            Suit::Hearts
        } else {
            c.suit
        };
        if let Some(entry) = counts.iter_mut().find(|(s, _)| *s == suit) {
            entry.1.push(i);
        } else {
            counts.push((suit, vec![i]));
        }
    }
    counts
}

/// Returns true when the cards form a straight, plus the indices involved.
fn find_straight(cards: &[&PlayingCardData]) -> Option<Vec<usize>> {
    let mut indexed: Vec<(u8, usize)> = cards
        .iter()
        .enumerate()
        .filter(|(_, c)| c.enhancement != Enhancement::Stone)
        .map(|(i, c)| (c.rank as u8, i))
        .collect();

    // Deduplicate by rank (keep first occurrence index).
    indexed.sort_by_key(|&(r, _)| r);
    indexed.dedup_by_key(|e| e.0);

    if indexed.len() < 5 {
        return None;
    }

    // Sliding window of 5 consecutive ranks.
    for window in indexed.windows(5) {
        let ranks: Vec<u8> = window.iter().map(|&(r, _)| r).collect();
        let min = *ranks.iter().min().unwrap();
        let max = *ranks.iter().max().unwrap();
        if max - min == 4 {
            return Some(window.iter().map(|&(_, i)| i).collect());
        }
    }

    // Ace-low straight: A-2-3-4-5
    let has_ace = indexed.iter().any(|&(r, _)| r == 14);
    if has_ace {
        let low_ranks: Vec<(u8, usize)> = indexed
            .iter()
            .filter(|&&(r, _)| r <= 5)
            .cloned()
            .collect();
        if low_ranks.len() >= 4 {
            let ranks: Vec<u8> = low_ranks.iter().map(|&(r, _)| r).collect();
            let min = *ranks.iter().min().unwrap();
            let max = *ranks.iter().max().unwrap();
            if max - min == 3 {
                let ace_idx = indexed.iter().find(|&&(r, _)| r == 14).unwrap().1;
                let mut idxs: Vec<usize> = low_ranks.iter().map(|&(_, i)| i).collect();
                idxs.push(ace_idx);
                return Some(idxs);
            }
        }
    }

    None
}

fn is_flush(cards: &[&PlayingCardData]) -> Option<Vec<usize>> {
    // Wild cards count for any suit.
    let non_wild: Vec<(Suit, usize)> = cards
        .iter()
        .enumerate()
        .filter(|(_, c)| c.enhancement != Enhancement::Wild && c.enhancement != Enhancement::Stone)
        .map(|(i, c)| (c.suit, i))
        .collect();

    if non_wild.is_empty() {
        // All wild – counts as flush.
        return Some((0..cards.len()).collect());
    }

    let first_suit = non_wild[0].0;
    if non_wild.iter().all(|&(s, _)| s == first_suit) {
        let mut indices: Vec<usize> = non_wild.iter().map(|&(_, i)| i).collect();
        // Include wilds.
        for (i, c) in cards.iter().enumerate() {
            if c.enhancement == Enhancement::Wild && !indices.contains(&i) {
                indices.push(i);
            }
        }
        indices.sort_unstable();
        Some(indices)
    } else {
        None
    }
}

// ── Main Evaluator ────────────────────────────────────────────────────────

/// Evaluate the best poker hand in a slice of played cards.
/// `cards` should be 1–5 PlayingCardData references.
pub fn evaluate_hand(card_refs: &[&PlayingCardData]) -> HandResult {
    let counts = rank_counts(card_refs);
    let flush_idxs = is_flush(card_refs);
    let straight_idxs = find_straight(card_refs);

    // ── Flush Five ───────────────────────────────────────────────────────
    if let Some(ref fi) = flush_idxs {
        if counts.first().map(|(_, v)| v.len()).unwrap_or(0) >= 5 {
            return HandResult {
                hand: PokerHand::FlushFive,
                scoring_cards: fi.clone(),
            };
        }
    }

    // ── Flush House ──────────────────────────────────────────────────────
    if let Some(ref fi) = flush_idxs {
        let threes = counts.iter().filter(|(_, v)| v.len() >= 3).count();
        let twos = counts.iter().filter(|(_, v)| v.len() >= 2).count();
        if threes >= 1 && twos >= 2 {
            return HandResult {
                hand: PokerHand::FlushHouse,
                scoring_cards: fi.clone(),
            };
        }
    }

    // ── Five of a Kind ───────────────────────────────────────────────────
    if let Some((_, idxs)) = counts.first() {
        if idxs.len() >= 5 {
            return HandResult {
                hand: PokerHand::FiveOfAKind,
                scoring_cards: idxs.clone(),
            };
        }
    }

    // ── Straight Flush ───────────────────────────────────────────────────
    if flush_idxs.is_some() && straight_idxs.is_some() {
        return HandResult {
            hand: PokerHand::StraightFlush,
            scoring_cards: straight_idxs.unwrap(),
        };
    }

    // ── Four of a Kind ───────────────────────────────────────────────────
    if let Some((_, idxs)) = counts.first() {
        if idxs.len() >= 4 {
            return HandResult {
                hand: PokerHand::FourOfAKind,
                scoring_cards: idxs.clone(),
            };
        }
    }

    // ── Full House ───────────────────────────────────────────────────────
    {
        let threes: Vec<&Vec<usize>> = counts.iter().filter(|(_, v)| v.len() >= 3).map(|(_, v)| v).collect();
        let twos:   Vec<&Vec<usize>> = counts.iter().filter(|(_, v)| v.len() >= 2).map(|(_, v)| v).collect();
        if !threes.is_empty() && twos.len() >= 2 {
            let mut scoring: Vec<usize> = threes[0].clone();
            // Add the pair (different group).
            for pair_idxs in &twos {
                if *pair_idxs != threes[0] {
                    scoring.extend_from_slice(pair_idxs);
                    break;
                }
            }
            return HandResult {
                hand: PokerHand::FullHouse,
                scoring_cards: scoring,
            };
        }
    }

    // ── Flush ────────────────────────────────────────────────────────────
    if let Some(fi) = flush_idxs {
        return HandResult {
            hand: PokerHand::Flush,
            scoring_cards: fi,
        };
    }

    // ── Straight ─────────────────────────────────────────────────────────
    if let Some(si) = straight_idxs {
        return HandResult {
            hand: PokerHand::Straight,
            scoring_cards: si,
        };
    }

    // ── Three of a Kind ──────────────────────────────────────────────────
    if let Some((_, idxs)) = counts.first() {
        if idxs.len() >= 3 {
            return HandResult {
                hand: PokerHand::ThreeOfAKind,
                scoring_cards: idxs.clone(),
            };
        }
    }

    // ── Two Pair ─────────────────────────────────────────────────────────
    {
        let pairs: Vec<&Vec<usize>> = counts.iter().filter(|(_, v)| v.len() >= 2).map(|(_, v)| v).collect();
        if pairs.len() >= 2 {
            let mut scoring = pairs[0].clone();
            scoring.extend_from_slice(pairs[1]);
            return HandResult {
                hand: PokerHand::TwoPair,
                scoring_cards: scoring,
            };
        }
    }

    // ── Pair ─────────────────────────────────────────────────────────────
    if let Some((_, idxs)) = counts.first() {
        if idxs.len() >= 2 {
            return HandResult {
                hand: PokerHand::Pair,
                scoring_cards: idxs.clone(),
            };
        }
    }

    // ── High Card ────────────────────────────────────────────────────────
    let best_idx = card_refs
        .iter()
        .enumerate()
        .filter(|(_, c)| c.enhancement != Enhancement::Stone)
        .max_by_key(|(_, c)| c.rank as u8)
        .map(|(i, _)| i)
        .unwrap_or(0);

    HandResult {
        hand: PokerHand::HighCard,
        scoring_cards: vec![best_idx],
    }
}
