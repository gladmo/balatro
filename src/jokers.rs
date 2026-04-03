use crate::cards::{Card, HandType, JokerCard, JokerKind, Rank, Suit};

/// Context for joker evaluation
pub struct JokerContext<'a> {
    pub hand_type: HandType,
    pub scoring_cards: &'a [Card],
    pub all_hand_cards: &'a [Card],
    pub money: i32,
    pub discards_remaining: u32,
}

/// Result of applying a joker
#[derive(Debug, Default)]
pub struct JokerEffect {
    pub chip_add: i64,
    pub mult_add: i64,
    pub x_mult: f64,
    pub money_add: i32,
    pub retrigger: bool,
}

/// Apply a single joker's effect
pub fn apply_joker(joker: &JokerCard, ctx: &JokerContext) -> JokerEffect {
    let mut effect = JokerEffect {
        x_mult: 1.0,
        ..Default::default()
    };

    match joker.kind {
        JokerKind::Joker => {
            effect.mult_add = 4;
        }
        JokerKind::GreedyJoker => {
            let count = ctx
                .scoring_cards
                .iter()
                .filter(|c| c.suit == Suit::Diamonds)
                .count();
            effect.mult_add = count as i64 * 3;
        }
        JokerKind::LustyJoker => {
            let count = ctx
                .scoring_cards
                .iter()
                .filter(|c| c.suit == Suit::Hearts)
                .count();
            effect.mult_add = count as i64 * 3;
        }
        JokerKind::WrathfulJoker => {
            let count = ctx
                .scoring_cards
                .iter()
                .filter(|c| c.suit == Suit::Spades)
                .count();
            effect.mult_add = count as i64 * 3;
        }
        JokerKind::GluttonousJoker => {
            let count = ctx
                .scoring_cards
                .iter()
                .filter(|c| c.suit == Suit::Clubs)
                .count();
            effect.mult_add = count as i64 * 3;
        }
        JokerKind::JollyJoker => {
            if matches!(
                ctx.hand_type,
                HandType::Pair
                    | HandType::TwoPair
                    | HandType::FullHouse
                    | HandType::FiveOfAKind
                    | HandType::FlushHouse
                    | HandType::FlushFive
            ) {
                effect.mult_add = 8;
            }
        }
        JokerKind::ZanyJoker => {
            if matches!(
                ctx.hand_type,
                HandType::ThreeOfAKind
                    | HandType::FullHouse
                    | HandType::FiveOfAKind
                    | HandType::FlushHouse
            ) {
                effect.mult_add = 12;
            }
        }
        JokerKind::MadJoker => {
            if matches!(ctx.hand_type, HandType::TwoPair | HandType::FlushHouse) {
                effect.mult_add = 10;
            }
        }
        JokerKind::CrazyJoker => {
            if matches!(
                ctx.hand_type,
                HandType::Straight | HandType::StraightFlush | HandType::RoyalFlush
            ) {
                effect.mult_add = 12;
            }
        }
        JokerKind::DrollJoker => {
            if matches!(
                ctx.hand_type,
                HandType::Flush
                    | HandType::StraightFlush
                    | HandType::RoyalFlush
                    | HandType::FlushHouse
                    | HandType::FlushFive
            ) {
                effect.mult_add = 10;
            }
        }
        JokerKind::SlyJoker => {
            if matches!(
                ctx.hand_type,
                HandType::Pair
                    | HandType::TwoPair
                    | HandType::FullHouse
                    | HandType::FiveOfAKind
            ) {
                effect.chip_add = 50;
            }
        }
        JokerKind::WilyJoker => {
            if matches!(
                ctx.hand_type,
                HandType::ThreeOfAKind
                    | HandType::FullHouse
                    | HandType::FourOfAKind
                    | HandType::FiveOfAKind
            ) {
                effect.chip_add = 100;
            }
        }
        JokerKind::CleverJoker => {
            if matches!(ctx.hand_type, HandType::TwoPair | HandType::FlushHouse) {
                effect.chip_add = 80;
            }
        }
        JokerKind::DeviousJoker => {
            if matches!(
                ctx.hand_type,
                HandType::Straight | HandType::StraightFlush | HandType::RoyalFlush
            ) {
                effect.chip_add = 100;
            }
        }
        JokerKind::CraftyJoker => {
            if matches!(
                ctx.hand_type,
                HandType::Flush
                    | HandType::StraightFlush
                    | HandType::RoyalFlush
                    | HandType::FlushHouse
                    | HandType::FlushFive
            ) {
                effect.chip_add = 80;
            }
        }
        JokerKind::HalfJoker => {
            if ctx.all_hand_cards.len() <= 3 {
                effect.mult_add = 20;
            }
        }
        JokerKind::BannerJoker => {
            effect.chip_add = ctx.discards_remaining as i64 * 30;
        }
        JokerKind::MysticSummit => {
            if ctx.discards_remaining == 0 {
                effect.mult_add = 15;
            }
        }
        JokerKind::Fibonacci => {
            let fib_ranks = [Rank::Ace, Rank::Two, Rank::Three, Rank::Five, Rank::Eight];
            let count = ctx
                .scoring_cards
                .iter()
                .filter(|c| fib_ranks.contains(&c.rank))
                .count();
            effect.mult_add = count as i64 * 8;
        }
        JokerKind::ScaryFace => {
            let count = ctx.scoring_cards.iter().filter(|c| c.rank.is_face()).count();
            effect.chip_add = count as i64 * 30;
        }
        JokerKind::Abstract => {
            // +3 mult for each joker - approximation
            effect.mult_add = 3;
        }
        JokerKind::Supernova => {
            // +1 mult for times played this run (use counter)
            effect.mult_add = joker.counter as i64;
        }
        JokerKind::GreenJoker => {
            // +1 mult per hand played, counter tracks this
            effect.mult_add = joker.counter as i64;
        }
        JokerKind::Bull => {
            effect.chip_add = ctx.money as i64 * 2;
        }
        JokerKind::SteelJoker => {
            effect.x_mult = 1.0 + (joker.counter as f64 * 0.2);
        }
        JokerKind::Blackboard => {
            let all_black = ctx
                .all_hand_cards
                .iter()
                .all(|c| c.suit == Suit::Spades || c.suit == Suit::Clubs);
            if all_black {
                effect.x_mult = 3.0;
            }
        }
        JokerKind::Baron => {
            let king_count = ctx
                .all_hand_cards
                .iter()
                .filter(|c| c.rank == Rank::King)
                .count();
            effect.x_mult = 1.5_f64.powi(king_count as i32);
        }
        JokerKind::Photograph => {
            if ctx.scoring_cards.iter().any(|c| c.rank.is_face()) {
                effect.x_mult = 2.0;
            }
        }
        _ => {}
    }

    effect
}

/// Apply all jokers and return total (chip_add, mult_add, x_mult_total)
pub fn apply_all_jokers(
    jokers: &[JokerCard],
    ctx: &JokerContext,
) -> (i64, i64, f64) {
    let mut total_chips: i64 = 0;
    let mut total_mult: i64 = 0;
    let mut total_x_mult: f64 = 1.0;

    for joker in jokers {
        let effect = apply_joker(joker, ctx);
        total_chips += effect.chip_add;
        total_mult += effect.mult_add;
        total_x_mult *= effect.x_mult;
    }

    (total_chips, total_mult, total_x_mult)
}
