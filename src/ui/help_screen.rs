pub struct HelpScreenPlugin;

impl bevy::prelude::Plugin for HelpScreenPlugin {
    fn build(&self, _app: &mut bevy::prelude::App) {
        // Help screen can be triggered from various states
    }
}

/// All poker hand types for the reference screen
pub fn get_hand_reference() -> Vec<(&'static str, &'static str, u32, u32)> {
    vec![
        ("Flush Five", "5 cards, same rank and suit", 160, 16),
        ("Flush House", "Full house, all same suit", 140, 14),
        ("Five of a Kind", "5 cards, same rank", 120, 12),
        ("Royal Flush", "Straight from 10 to A, same suit", 100, 8),
        ("Straight Flush", "5 consecutive cards, same suit", 100, 8),
        ("Four of a Kind", "4 cards, same rank", 60, 7),
        ("Full House", "3 of a kind + pair", 40, 4),
        ("Flush", "5 cards, same suit", 35, 4),
        ("Straight", "5 consecutive cards", 30, 4),
        ("Three of a Kind", "3 cards, same rank", 30, 3),
        ("Two Pair", "2 different pairs", 20, 2),
        ("Pair", "2 cards, same rank", 10, 2),
        ("High Card", "Single highest card", 5, 1),
    ]
}
