#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;

pub mod main_menu;
pub mod game_ui;
pub mod blind_select;
pub mod shop_ui;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Main menu
            .add_systems(OnEnter(crate::GameState::MainMenu), main_menu::setup_main_menu)
            .add_systems(Update, main_menu::main_menu_buttons.run_if(in_state(crate::GameState::MainMenu)))
            .add_systems(OnExit(crate::GameState::MainMenu), cleanup_screen::<main_menu::MainMenuRoot>)
            // Blind select
            .add_systems(OnEnter(crate::GameState::BlindSelect), blind_select::setup_blind_select)
            .add_systems(Update, blind_select::blind_select_buttons.run_if(in_state(crate::GameState::BlindSelect)))
            .add_systems(OnExit(crate::GameState::BlindSelect), cleanup_screen::<blind_select::BlindSelectRoot>)
            // Playing
            .add_systems(OnEnter(crate::GameState::Playing), game_ui::setup_game_ui)
            .add_systems(Update, game_ui::game_buttons.run_if(in_state(crate::GameState::Playing)))
            .add_systems(Update, game_ui::update_score_display.run_if(in_state(crate::GameState::Playing)))
            .add_systems(Update, game_ui::update_hand_display.run_if(in_state(crate::GameState::Playing)))
            .add_systems(Update, game_ui::card_selection_buttons.run_if(in_state(crate::GameState::Playing)))
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
) {
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
            TextFont { font_size: 72.0, ..default() },
            TextColor(Color::srgb(0.9, 0.2, 0.2)),
        ));

        let score_text = format!("Score: {}", game_data.run_score);
        parent.spawn((
            Text::new(score_text),
            TextFont { font_size: 36.0, ..default() },
            TextColor(Color::WHITE),
        ));

        let ante_text = format!("Reached Ante {}", game_data.ante);
        parent.spawn((
            Text::new(ante_text),
            TextFont { font_size: 24.0, ..default() },
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
                TextFont { font_size: 24.0, ..default() },
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
                TextFont { font_size: 24.0, ..default() },
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
) {
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
            TextFont { font_size: 72.0, ..default() },
            TextColor(Color::srgb(1.0, 0.9, 0.1)),
        ));

        parent.spawn((
            Text::new("You beat all 8 antes!"),
            TextFont { font_size: 36.0, ..default() },
            TextColor(Color::WHITE),
        ));

        let score_text = format!("Final Score: {}", game_data.run_score);
        parent.spawn((
            Text::new(score_text),
            TextFont { font_size: 28.0, ..default() },
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
                TextFont { font_size: 24.0, ..default() },
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
