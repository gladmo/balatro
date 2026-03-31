#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenuRoot;

#[derive(Component)]
pub struct NewRunButton;

#[derive(Component)]
pub struct ContinueButton;

#[derive(Component)]
pub struct QuitButton;

pub fn setup_main_menu(
    mut commands: Commands,
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
        BackgroundColor(Color::srgb(0.08, 0.08, 0.12)),
        MainMenuRoot,
    )).with_children(|parent| {
        // Title
        parent.spawn((
            Text::new("BALATRO"),
            TextFont { font_size: 80.0, ..default() },
            TextColor(Color::srgb(0.9, 0.7, 0.1)),
        ));

        parent.spawn((
            Text::new("A Poker Roguelite"),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
        ));

        parent.spawn((
            Node {
                height: Val::Px(30.0),
                ..default()
            },
        ));

        // New Run button
        parent.spawn((
            Button,
            Node {
                width: Val::Px(220.0),
                height: Val::Px(55.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.45, 0.15)),
            BorderColor::from(Color::srgb(0.3, 0.7, 0.3)),
            NewRunButton,
        )).with_children(|btn| {
            btn.spawn((
                Text::new("New Run"),
                TextFont { font_size: 28.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });

        // Quit button
        parent.spawn((
            Button,
            Node {
                width: Val::Px(220.0),
                height: Val::Px(55.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.4, 0.15, 0.15)),
            BorderColor::from(Color::srgb(0.7, 0.3, 0.3)),
            QuitButton,
        )).with_children(|btn| {
            btn.spawn((
                Text::new("Quit"),
                TextFont { font_size: 28.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
    });
}

pub fn main_menu_buttons(
    new_run_query: Query<&Interaction, (Changed<Interaction>, With<NewRunButton>)>,
    quit_query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
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
    for interaction in &new_run_query {
        if *interaction == Interaction::Pressed {
            crate::ui::reset_game(
                &mut game_data, &mut deck, &mut hand, &mut selected,
                &mut discard_pile, &mut jokers, &mut hand_levels, &mut shop,
            );
            next_state.set(crate::GameState::BlindSelect);
        }
    }

    for interaction in &quit_query {
        if *interaction == Interaction::Pressed {
            std::process::exit(0);
        }
    }
}
