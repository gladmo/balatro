#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;
use rand::Rng;
use rand::seq::SliceRandom;
use crate::jokers::{Joker, JokerId};
use crate::consumables::{Consumable, TarotCard, PlanetCard, random_tarot, random_planet};
use crate::game_data::GameData;

#[derive(Debug, Clone)]
pub enum ShopItem {
    JokerItem(Joker),
    ConsumableItem(Consumable),
}

impl ShopItem {
    pub fn name(&self) -> String {
        match self {
            ShopItem::JokerItem(j) => j.name().to_string(),
            ShopItem::ConsumableItem(c) => c.name().to_string(),
        }
    }

    pub fn description(&self) -> String {
        match self {
            ShopItem::JokerItem(j) => j.description().to_string(),
            ShopItem::ConsumableItem(c) => c.description().to_string(),
        }
    }

    pub fn cost(&self) -> i32 {
        match self {
            ShopItem::JokerItem(j) => j.cost,
            ShopItem::ConsumableItem(c) => c.cost(),
        }
    }
}

#[derive(Resource)]
pub struct ShopState {
    pub items: Vec<Option<ShopItem>>,
    pub reroll_cost: i32,
}

impl Default for ShopState {
    fn default() -> Self {
        ShopState {
            items: vec![None, None, None, None],
            reroll_cost: 5,
        }
    }
}

impl ShopState {
    pub fn new() -> Self {
        ShopState::default()
    }

    pub fn generate(&mut self, rng: &mut impl Rng, owned_joker_count: usize, joker_slots: usize) {
        self.items.clear();

        // 2 joker slots
        for _ in 0..2 {
            if owned_joker_count + self.joker_count() < joker_slots + 2 {
                let all_jokers = JokerId::all();
                let joker_id = *all_jokers.choose(rng).unwrap();
                self.items.push(Some(ShopItem::JokerItem(Joker::new(joker_id))));
            }
        }

        // 2 consumable slots
        for _ in 0..2 {
            if rng.gen_bool(0.5) {
                let tarot = random_tarot(rng);
                self.items.push(Some(ShopItem::ConsumableItem(Consumable::Tarot(tarot))));
            } else {
                let planet = random_planet(rng);
                self.items.push(Some(ShopItem::ConsumableItem(Consumable::Planet(planet))));
            }
        }

        // Pad to 4 slots
        while self.items.len() < 4 {
            self.items.push(None);
        }
    }

    fn joker_count(&self) -> usize {
        self.items.iter()
            .filter(|i| matches!(i, Some(ShopItem::JokerItem(_))))
            .count()
    }

    pub fn buy_item(&mut self, index: usize) -> Option<ShopItem> {
        if index < self.items.len() {
            self.items[index].take()
        } else {
            None
        }
    }

    pub fn reroll(&mut self, rng: &mut impl Rng, owned_joker_count: usize, joker_slots: usize) {
        self.generate(rng, owned_joker_count, joker_slots);
    }
}

#[derive(Event)]
pub struct BuyItemEvent {
    pub index: usize,
}

#[derive(Event)]
pub struct SellJokerEvent {
    pub index: usize,
}

#[derive(Event)]
pub struct RerollShopEvent;

pub fn handle_buy_item(
    mut events: EventReader<BuyItemEvent>,
    mut shop: ResMut<ShopState>,
    mut game_data: ResMut<GameData>,
    mut jokers: ResMut<crate::jokers::OwnedJokers>,
    mut consumables: ResMut<crate::consumables::ConsumableSlots>,
) {
    for event in events.read() {
        let index = event.index;
        if index >= shop.items.len() {
            continue;
        }
        if let Some(ref item) = shop.items[index] {
            let cost = item.cost();
            if game_data.money >= cost {
                let item = shop.buy_item(index);
                if let Some(item) = item {
                    game_data.money -= cost;
                    match item {
                        ShopItem::JokerItem(joker) => {
                            jokers.add(joker);
                        }
                        ShopItem::ConsumableItem(consumable) => {
                            consumables.add(consumable);
                        }
                    }
                }
            }
        }
    }
}

pub fn handle_sell_joker(
    mut events: EventReader<SellJokerEvent>,
    mut jokers: ResMut<crate::jokers::OwnedJokers>,
    mut game_data: ResMut<GameData>,
) {
    for event in events.read() {
        if let Some(joker) = jokers.remove(event.index) {
            game_data.money += joker.sell_value;
        }
    }
}

pub fn handle_reroll_shop(
    mut events: EventReader<RerollShopEvent>,
    mut shop: ResMut<ShopState>,
    mut game_data: ResMut<GameData>,
    jokers: Res<crate::jokers::OwnedJokers>,
) {
    for _ in events.read() {
        if game_data.money >= shop.reroll_cost {
            game_data.money -= shop.reroll_cost;
            let mut rng = rand::thread_rng();
            shop.reroll(&mut rng, jokers.len(), game_data.joker_slots as usize);
        }
    }
}
