#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy::asset::AssetPlugin;

mod cards;
mod deck;
mod hand_eval;
mod jokers;
mod scoring;
mod game_data;
mod shop;
mod consumables;
mod audio;
mod animation;
mod card_shader;
mod localization;
mod save;
mod textures;
mod ui;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    HelpScreen,
    BlindSelect,
    Playing,
    Scoring,
    Shop,
    GameOver,
    Victory,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Balatro".to_string(),
                    resolution: WindowResolution::new(1280, 720),
                    ..default()
                }),
                ..default()
            })
            // Load assets from resources/ to match the original project structure
            .set(AssetPlugin {
                file_path: "resources".into(),
                ..default()
            })
        )
        // States
        .init_state::<GameState>()
        // Resources
        .init_resource::<game_data::GameData>()
        .init_resource::<localization::Localization>()
        .init_resource::<hand_eval::HandLevels>()
        .init_resource::<consumables::ConsumableSlots>()
        .init_resource::<shop::ShopState>()
        // Plugins
        .add_plugins(audio::AudioPlugin)
        .add_plugins(animation::AnimationPlugin)
        .add_plugins(card_shader::CardShaderPlugin)
        .add_plugins(ui::UiPlugin)
        // Startup systems
        .add_systems(Startup, setup)
        // Load textures after the window is created so we can read the scale factor
        .add_systems(PostStartup, textures::load_game_textures)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Initialize deck, hand, selected, discard with default values
    let mut deck = deck::Deck::new();
    deck::shuffle_deck(&mut deck);

    let hand = deck::Hand::new(8);
    let selected = deck::SelectedCards::default();
    let discard_pile = deck::DiscardPile::default();
    let jokers = jokers::OwnedJokers::new(5);
    let hand_levels = hand_eval::HandLevels::new();

    commands.insert_resource(deck);
    commands.insert_resource(hand);
    commands.insert_resource(selected);
    commands.insert_resource(discard_pile);
    commands.insert_resource(jokers);
    commands.insert_resource(hand_levels);
}
