#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;
use crate::localization::{Language, Localization};

pub mod main_menu;
pub mod game_ui;
pub mod blind_select;
pub mod shop_ui;
pub mod help_screen;

/// Bevy font handles for the supported UI languages.
#[derive(Resource, Default)]
pub struct FontAssets {
    pub english: Handle<Font>,
    pub chinese: Handle<Font>,
}

/// Returns the correct font handle for the currently active language.
pub fn current_font(lang: Language, fonts: &FontAssets) -> Handle<Font> {
    match lang {
        Language::Chinese => fonts.chinese.clone(),
        Language::English => fonts.english.clone(),
    }
}

/// PostStartup system — loads font assets into `FontAssets` resource.
pub fn load_fonts(
    mut fonts: ResMut<FontAssets>,
    asset_server: Res<AssetServer>,
) {
    fonts.english = asset_server.load("fonts/m6x11plus.ttf");
    fonts.chinese = asset_server.load("fonts/SourceHanSansSC-Regular.ttf");
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FontAssets>()
           .add_systems(PostStartup, load_fonts)
            // Main menu
            .add_systems(OnEnter(crate::GameState::MainMenu), main_menu::setup_main_menu)
            .add_systems(Update, main_menu::main_menu_buttons.run_if(in_state(crate::GameState::MainMenu)))
            .add_systems(OnExit(crate::GameState::MainMenu), cleanup_screen::<main_menu::MainMenuRoot>)
            // Help screen
            .add_systems(OnEnter(crate::GameState::HelpScreen), help_screen::setup_help_screen)
            .add_systems(Update, help_screen::help_close_button.run_if(in_state(crate::GameState::HelpScreen)))
            .add_systems(OnExit(crate::GameState::HelpScreen), cleanup_screen::<help_screen::HelpRoot>)
            // Blind select
            .add_systems(OnEnter(crate::GameState::BlindSelect), blind_select::setup_blind_select)
            .add_systems(Update, blind_select::blind_select_buttons.run_if(in_state(crate::GameState::BlindSelect)))
            .add_systems(OnExit(crate::GameState::BlindSelect), cleanup_screen::<blind_select::BlindSelectRoot>)
            // Playing
            .add_systems(OnEnter(crate::GameState::Playing), game_ui::setup_game_ui)
            // game_buttons writes &mut BackgroundColor + &mut ButtonFlash (same types as
            // animate_button_flash in AnimationPlugin) → must run before the animation chain.
            .add_systems(Update, game_ui::game_buttons
                .before(crate::animation::animate_button_flash)
                .run_if(in_state(crate::GameState::Playing)))
            .add_systems(Update, game_ui::update_score_display.run_if(in_state(crate::GameState::Playing)))
            .add_systems(Update, game_ui::update_hand_display.run_if(in_state(crate::GameState::Playing)))
            // card_selection_buttons writes CardSelectAnim.selected_offset; animate_card_select
            // (in AnimationPlugin) then reads+writes it.  Ordering them avoids the B0001 conflict.
            .add_systems(Update, game_ui::card_selection_buttons
                .before(crate::animation::animate_card_select)
                .run_if(in_state(crate::GameState::Playing)))
            // update_card_tooltip writes &mut Node (With<CardTooltip>); the animation chain
            // (animate_card_hover, animate_card_select) also writes &mut Node.  Run tooltip
            // update first so the animation chain can see the final state in the same tick.
            .add_systems(Update, game_ui::update_card_tooltip
                .before(crate::animation::animate_card_hover)
                .run_if(in_state(crate::GameState::Playing)))
            .add_systems(OnExit(crate::GameState::Playing), cleanup_screen::<game_ui::GameUiRoot>)
            // Shop
            .add_systems(OnEnter(crate::GameState::Shop), shop_ui::setup_shop)
            .add_systems(Update, shop_ui::shop_buttons.run_if(in_state(crate::GameState::Shop)))
            .add_systems(OnExit(crate::GameState::Shop), cleanup_screen::<shop_ui::ShopRoot>)
            // Game over
            .add_systems(OnEnter(crate::GameState::GameOver), setup_game_over)
            .add_systems(Update, game_over_buttons.run_if(in_state(crate::GameState::GameOver)))
            .add_systems(OnExit(crate::GameState::GameOver), cleanup_screen::<GameOverRoot>)
            // Victory
            .add_systems(OnEnter(crate::GameState::Victory), setup_victory)
            .add_systems(Update, victory_buttons.run_if(in_state(crate::GameState::Victory)))
            .add_systems(OnExit(crate::GameState::Victory), cleanup_screen::<VictoryRoot>);
    }
}

pub fn cleanup_screen<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// ─── Game Over Screen ───────────────────────────────────────────────────────

#[derive(Component)]
pub struct GameOverRoot;

#[derive(Component)]
pub struct GameOverPlayAgainButton;

#[derive(Component)]
pub struct GameOverMenuButton;

pub fn setup_game_over(
    mut commands: Commands,
    game_data: Res<crate::game_data::GameData>,
    fonts: Res<FontAssets>,
    loc: Res<Localization>,
) {
    let lang = loc.language();
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.05, 0.05)),
        GameOverRoot,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("GAME OVER"),
            TextFont { font: current_font(lang, &fonts), font_size: 72.0, ..default() },
            TextColor(Color::srgb(0.9, 0.2, 0.2)),
        ));

        let score_text = format!("Score: {}", game_data.run_score);
        parent.spawn((
            Text::new(score_text),
            TextFont { font: current_font(lang, &fonts), font_size: 36.0, ..default() },
            TextColor(Color::WHITE),
        ));

        let ante_text = format!("Reached Ante {}", game_data.ante);
        parent.spawn((
            Text::new(ante_text),
            TextFont { font: current_font(lang, &fonts), font_size: 24.0, ..default() },
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
        ));

        parent.spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.6, 0.2)),
            GameOverPlayAgainButton,
        )).with_children(|btn| {
            btn.spawn((
                Text::new("Play Again"),
                TextFont { font: current_font(lang, &fonts), font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });

        parent.spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
            GameOverMenuButton,
        )).with_children(|btn| {
            btn.spawn((
                Text::new("Main Menu"),
                TextFont { font: current_font(lang, &fonts), font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
    });
}

pub fn game_over_buttons(
    play_query: Query<&Interaction, (Changed<Interaction>, With<GameOverPlayAgainButton>)>,
    menu_query: Query<&Interaction, (Changed<Interaction>, With<GameOverMenuButton>)>,
    mut next_state: ResMut<NextState<crate::GameState>>,
    mut game_data: ResMut<crate::game_data::GameData>,
    mut deck: ResMut<crate::deck::Deck>,
    mut hand: ResMut<crate::deck::Hand>,
    mut selected: ResMut<crate::deck::SelectedCards>,
    mut discard_pile: ResMut<crate::deck::DiscardPile>,
    mut jokers: ResMut<crate::jokers::OwnedJokers>,
    mut hand_levels: ResMut<crate::hand_eval::HandLevels>,
    mut shop: ResMut<crate::shop::ShopState>,
) {
    for interaction in &play_query {
        if *interaction == Interaction::Pressed {
            reset_game(
                &mut game_data, &mut deck, &mut hand, &mut selected,
                &mut discard_pile, &mut jokers, &mut hand_levels, &mut shop,
            );
            next_state.set(crate::GameState::MainMenu);
        }
    }
    for interaction in &menu_query {
        if *interaction == Interaction::Pressed {
            reset_game(
                &mut game_data, &mut deck, &mut hand, &mut selected,
                &mut discard_pile, &mut jokers, &mut hand_levels, &mut shop,
            );
            next_state.set(crate::GameState::MainMenu);
        }
    }
}

// ─── Victory Screen ─────────────────────────────────────────────────────────

#[derive(Component)]
pub struct VictoryRoot;

#[derive(Component)]
pub struct VictoryMenuButton;

pub fn setup_victory(
    mut commands: Commands,
    game_data: Res<crate::game_data::GameData>,
    fonts: Res<FontAssets>,
    loc: Res<Localization>,
) {
    let lang = loc.language();
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.05, 0.1, 0.05)),
        VictoryRoot,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("VICTORY!"),
            TextFont { font: current_font(lang, &fonts), font_size: 72.0, ..default() },
            TextColor(Color::srgb(1.0, 0.9, 0.1)),
        ));

        parent.spawn((
            Text::new("You beat all 8 antes!"),
            TextFont { font: current_font(lang, &fonts), font_size: 36.0, ..default() },
            TextColor(Color::WHITE),
        ));

        let score_text = format!("Final Score: {}", game_data.run_score);
        parent.spawn((
            Text::new(score_text),
            TextFont { font: current_font(lang, &fonts), font_size: 28.0, ..default() },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        ));

        parent.spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.6, 0.4, 0.1)),
            VictoryMenuButton,
        )).with_children(|btn| {
            btn.spawn((
                Text::new("Main Menu"),
                TextFont { font: current_font(lang, &fonts), font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
    });
}

pub fn victory_buttons(
    query: Query<&Interaction, (Changed<Interaction>, With<VictoryMenuButton>)>,
    mut next_state: ResMut<NextState<crate::GameState>>,
    mut game_data: ResMut<crate::game_data::GameData>,
    mut deck: ResMut<crate::deck::Deck>,
    mut hand: ResMut<crate::deck::Hand>,
    mut selected: ResMut<crate::deck::SelectedCards>,
    mut discard_pile: ResMut<crate::deck::DiscardPile>,
    mut jokers: ResMut<crate::jokers::OwnedJokers>,
    mut hand_levels: ResMut<crate::hand_eval::HandLevels>,
    mut shop: ResMut<crate::shop::ShopState>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            reset_game(
                &mut game_data, &mut deck, &mut hand, &mut selected,
                &mut discard_pile, &mut jokers, &mut hand_levels, &mut shop,
            );
            next_state.set(crate::GameState::MainMenu);
        }
    }
}

pub fn reset_game(
    game_data: &mut crate::game_data::GameData,
    deck: &mut crate::deck::Deck,
    hand: &mut crate::deck::Hand,
    selected: &mut crate::deck::SelectedCards,
    discard_pile: &mut crate::deck::DiscardPile,
    jokers: &mut crate::jokers::OwnedJokers,
    hand_levels: &mut crate::hand_eval::HandLevels,
    shop: &mut crate::shop::ShopState,
) {
    *game_data = crate::game_data::GameData::new();
    *deck = crate::deck::Deck::new();
    crate::deck::shuffle_deck(deck);
    hand.cards.clear();
    selected.clear();
    discard_pile.cards.clear();
    jokers.jokers.clear();
    *hand_levels = crate::hand_eval::HandLevels::new();
    *shop = crate::shop::ShopState::new();
}
