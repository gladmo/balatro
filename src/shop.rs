use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::cards::{Card, Edition, Enhancement, JokerCard, JokerKind, Rank, Seal, Suit};
use crate::game_state::{AppState, GameData};

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShopInventory::default())
            .add_systems(OnEnter(AppState::Shop), generate_shop);
    }
}

#[derive(Debug, Clone)]
pub enum ShopItem {
    PlayingCard(Card),
    JokerItem(JokerCard),
    Consumable(ConsumableKind),
    Voucher(VoucherKind),
    BoosterPack(BoosterKind),
}

impl ShopItem {
    pub fn cost(&self) -> u32 {
        match self {
            ShopItem::PlayingCard(_) => 3,
            ShopItem::JokerItem(j) => j.kind.base_cost(),
            ShopItem::Consumable(_) => 3,
            ShopItem::Voucher(_) => 10,
            ShopItem::BoosterPack(_) => 4,
        }
    }

    pub fn display_name(&self) -> String {
        match self {
            ShopItem::PlayingCard(c) => c.display_name(),
            ShopItem::JokerItem(j) => j.kind.display_name().to_string(),
            ShopItem::Consumable(c) => c.display_name().to_string(),
            ShopItem::Voucher(v) => v.display_name().to_string(),
            ShopItem::BoosterPack(b) => b.display_name().to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsumableKind {
    TheFool,
    TheMagician,
    TheHighPriestess,
    TheEmpress,
    TheEmperor,
    TheHierophant,
    TheLovers,
    TheChariot,
    Justice,
    TheHermit,
    WheelOfFortune,
    Strength,
    TheHangedMan,
    Death,
    Temperance,
    TheDevil,
    TheTower,
    TheStar,
    TheMoon,
    TheSun,
    Judgement,
    TheWorld,
    // Planets
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
    Pluto,
    PlanetX,
    Ceres,
    Eris,
}

impl ConsumableKind {
    pub fn display_name(&self) -> &str {
        match self {
            ConsumableKind::TheFool => "The Fool",
            ConsumableKind::TheMagician => "The Magician",
            ConsumableKind::TheHighPriestess => "The High Priestess",
            ConsumableKind::TheEmpress => "The Empress",
            ConsumableKind::TheEmperor => "The Emperor",
            ConsumableKind::TheHierophant => "The Hierophant",
            ConsumableKind::TheLovers => "The Lovers",
            ConsumableKind::TheChariot => "The Chariot",
            ConsumableKind::Justice => "Justice",
            ConsumableKind::TheHermit => "The Hermit",
            ConsumableKind::WheelOfFortune => "Wheel of Fortune",
            ConsumableKind::Strength => "Strength",
            ConsumableKind::TheHangedMan => "The Hanged Man",
            ConsumableKind::Death => "Death",
            ConsumableKind::Temperance => "Temperance",
            ConsumableKind::TheDevil => "The Devil",
            ConsumableKind::TheTower => "The Tower",
            ConsumableKind::TheStar => "The Star",
            ConsumableKind::TheMoon => "The Moon",
            ConsumableKind::TheSun => "The Sun",
            ConsumableKind::Judgement => "Judgement",
            ConsumableKind::TheWorld => "The World",
            ConsumableKind::Mercury => "Mercury",
            ConsumableKind::Venus => "Venus",
            ConsumableKind::Earth => "Earth",
            ConsumableKind::Mars => "Mars",
            ConsumableKind::Jupiter => "Jupiter",
            ConsumableKind::Saturn => "Saturn",
            ConsumableKind::Uranus => "Uranus",
            ConsumableKind::Neptune => "Neptune",
            ConsumableKind::Pluto => "Pluto",
            ConsumableKind::PlanetX => "Planet X",
            ConsumableKind::Ceres => "Ceres",
            ConsumableKind::Eris => "Eris",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoucherKind {
    Overstock,
    Clearance,
    Hone,
    Reroll,
    Crystal,
    Telescope,
    Grabber,
    Wasteful,
    Seed,
    Blank,
    MagicTrick,
    Hieroglyph,
    Directors,
    Paint,
    Antimatter,
}

impl VoucherKind {
    pub fn display_name(&self) -> &str {
        match self {
            VoucherKind::Overstock => "Overstock",
            VoucherKind::Clearance => "Clearance Sale",
            VoucherKind::Hone => "Hone",
            VoucherKind::Reroll => "Reroll Surplus",
            VoucherKind::Crystal => "Crystal Ball",
            VoucherKind::Telescope => "Telescope",
            VoucherKind::Grabber => "Grabber",
            VoucherKind::Wasteful => "Wasteful",
            VoucherKind::Seed => "Seed Money",
            VoucherKind::Blank => "Blank",
            VoucherKind::MagicTrick => "Magic Trick",
            VoucherKind::Hieroglyph => "Hieroglyph",
            VoucherKind::Directors => "Director's Cut",
            VoucherKind::Paint => "Paint Brush",
            VoucherKind::Antimatter => "Antimatter",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoosterKind {
    ArcanaPack,
    CelestialPack,
    SpectralPack,
    StandardPack,
    BuffoonPack,
}

impl BoosterKind {
    pub fn display_name(&self) -> &str {
        match self {
            BoosterKind::ArcanaPack => "Arcana Pack",
            BoosterKind::CelestialPack => "Celestial Pack",
            BoosterKind::SpectralPack => "Spectral Pack",
            BoosterKind::StandardPack => "Standard Pack",
            BoosterKind::BuffoonPack => "Buffoon Pack",
        }
    }
}

#[derive(Resource, Default)]
pub struct ShopInventory {
    pub items: Vec<ShopItem>,
    pub voucher: Option<ShopItem>,
    pub booster_packs: Vec<ShopItem>,
}

fn generate_shop(mut inventory: ResMut<ShopInventory>, _game_data: Res<GameData>) {
    let mut rng = rand::thread_rng();
    inventory.items.clear();
    inventory.voucher = None;
    inventory.booster_packs.clear();

    // Generate 2 random jokers
    let joker_pool = [
        JokerKind::Joker,
        JokerKind::GreedyJoker,
        JokerKind::LustyJoker,
        JokerKind::JollyJoker,
        JokerKind::ZanyJoker,
        JokerKind::MadJoker,
        JokerKind::CrazyJoker,
        JokerKind::DrollJoker,
        JokerKind::HalfJoker,
        JokerKind::BannerJoker,
        JokerKind::Fibonacci,
        JokerKind::ScaryFace,
        JokerKind::Abstract,
        JokerKind::Supernova,
        JokerKind::GreenJoker,
        JokerKind::Bull,
        JokerKind::Blackboard,
    ];

    for _ in 0..2 {
        if let Some(&kind) = joker_pool.choose(&mut rng) {
            inventory
                .items
                .push(ShopItem::JokerItem(JokerCard::new(kind)));
        }
    }

    // Generate 1 random consumable
    let consumable_pool = [
        ConsumableKind::TheFool,
        ConsumableKind::TheMagician,
        ConsumableKind::TheChariot,
        ConsumableKind::Jupiter,
        ConsumableKind::Saturn,
        ConsumableKind::Mercury,
    ];
    if let Some(&kind) = consumable_pool.choose(&mut rng) {
        inventory.items.push(ShopItem::Consumable(kind));
    }

    // Generate 1 playing card
    let suits = Suit::all();
    let ranks = Rank::all();
    if let (Some(&suit), Some(&rank)) = (suits.choose(&mut rng), ranks.choose(&mut rng)) {
        inventory.items.push(ShopItem::PlayingCard(Card {
            suit,
            rank,
            enhancement: Enhancement::None,
            edition: Edition::Base,
            seal: Seal::None,
            face_up: true,
            selected: false,
            debuffed: false,
            id: 999,
        }));
    }

    // Generate voucher
    let voucher_pool = [
        VoucherKind::Overstock,
        VoucherKind::Clearance,
        VoucherKind::Grabber,
        VoucherKind::Wasteful,
    ];
    if let Some(&kind) = voucher_pool.choose(&mut rng) {
        inventory.voucher = Some(ShopItem::Voucher(kind));
    }

    // Generate 2 booster packs
    let booster_pool = [
        BoosterKind::ArcanaPack,
        BoosterKind::CelestialPack,
        BoosterKind::StandardPack,
        BoosterKind::BuffoonPack,
    ];
    for _ in 0..2 {
        if let Some(&kind) = booster_pool.choose(&mut rng) {
            inventory
                .booster_packs
                .push(ShopItem::BoosterPack(kind));
        }
    }
}

/// Attempt to purchase an item from the shop
pub fn try_purchase(
    game_data: &mut GameData,
    item: &ShopItem,
) -> bool {
    let cost = item.cost() as i32;
    if game_data.money < cost {
        return false;
    }
    game_data.money -= cost;

    match item {
        ShopItem::JokerItem(joker) => {
            if game_data.jokers.len() < game_data.joker_slots as usize {
                game_data.jokers.push(joker.clone());
                return true;
            }
            game_data.money += cost; // refund
            false
        }
        ShopItem::PlayingCard(card) => {
            game_data.deck.push(card.clone());
            true
        }
        _ => true,
    }
}
