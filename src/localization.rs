#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct Localization {
    strings: HashMap<&'static str, &'static str>,
}

impl Default for Localization {
    fn default() -> Self {
        let mut strings = HashMap::new();

        // Main menu
        strings.insert("menu.title", "BALATRO");
        strings.insert("menu.new_run", "New Run");
        strings.insert("menu.continue", "Continue");
        strings.insert("menu.quit", "Quit");

        // Game UI
        strings.insert("ui.score", "Score");
        strings.insert("ui.target", "Target");
        strings.insert("ui.money", "Money");
        strings.insert("ui.hands", "Hands");
        strings.insert("ui.discards", "Discards");
        strings.insert("ui.ante", "Ante");
        strings.insert("ui.round", "Round");
        strings.insert("ui.play_hand", "Play Hand");
        strings.insert("ui.discard", "Discard");
        strings.insert("ui.jokers", "Jokers");

        // Blinds
        strings.insert("blind.small", "Small Blind");
        strings.insert("blind.big", "Big Blind");
        strings.insert("blind.boss", "Boss Blind");
        strings.insert("blind.select", "Select Blind");
        strings.insert("blind.skip", "Skip");
        strings.insert("blind.play", "Play");
        strings.insert("blind.reward", "Reward");

        // Shop
        strings.insert("shop.title", "Shop");
        strings.insert("shop.buy", "Buy");
        strings.insert("shop.sell", "Sell");
        strings.insert("shop.reroll", "Reroll ($5)");
        strings.insert("shop.continue", "Next Round");
        strings.insert("shop.money", "$");
        strings.insert("shop.jokers", "Jokers");
        strings.insert("shop.consumables", "Consumables");

        // Game over
        strings.insert("game_over.title", "Game Over");
        strings.insert("game_over.score", "Final Score");
        strings.insert("game_over.play_again", "Play Again");
        strings.insert("game_over.menu", "Main Menu");

        // Victory
        strings.insert("victory.title", "Victory!");
        strings.insert("victory.message", "You beat all 8 antes!");

        // Hand types
        strings.insert("hand.high_card", "High Card");
        strings.insert("hand.pair", "Pair");
        strings.insert("hand.two_pair", "Two Pair");
        strings.insert("hand.three_of_a_kind", "Three of a Kind");
        strings.insert("hand.straight", "Straight");
        strings.insert("hand.flush", "Flush");
        strings.insert("hand.full_house", "Full House");
        strings.insert("hand.four_of_a_kind", "Four of a Kind");
        strings.insert("hand.straight_flush", "Straight Flush");
        strings.insert("hand.five_of_a_kind", "Five of a Kind");
        strings.insert("hand.flush_house", "Flush House");
        strings.insert("hand.flush_five", "Flush Five");

        Localization { strings }
    }
}

impl Localization {
    pub fn get<'a>(&self, key: &'a str) -> &'a str {
        self.strings.get(key).copied().unwrap_or(key)
    }
}

pub fn loc(loc: &Localization, key: &str) -> String {
    loc.get(key).to_string()
}
