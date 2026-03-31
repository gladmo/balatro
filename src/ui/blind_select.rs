#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;
use crate::game_data::{BlindType, GameData};

#[derive(Component)]
pub struct BlindSelectRoot;

#[derive(Component)]
pub struct SelectSmallBlindButton;

#[derive(Component)]
pub struct SelectBigBlindButton;

#[derive(Component)]
pub struct SelectBossBlindButton;

#[derive(Component)]
pub struct SkipSmallBlindButton;

#[derive(Component)]
pub struct SkipBigBlindButton;

pub fn setup_blind_select(
    mut commands: Commands,
    game_data: Res<GameData>,
) {
    let ante = game_data.ante;
    let small_target = game_data.blind_target_for(ante, 1);
    let big_target = game_data.blind_target_for(ante, 2);
    let boss_target = game_data.blind_target_for(ante, 3);
    let boss_name = crate::game_data::BossBlind::from_index((ante as usize).wrapping_sub(1)).name();

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(16.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.08, 0.08, 0.14)),
        BlindSelectRoot,
    )).with_children(|parent| {
        let title = format!("Select Blind — Ante {}", ante);
        parent.spawn((
            Text::new(title),
            TextFont { font_size: 40.0, ..default() },
            TextColor(Color::srgb(0.9, 0.85, 0.3)),
        ));

        // Row of 3 blinds
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(30.0),
                ..default()
            },
        )).with_children(|row| {
            // Small blind card
            spawn_blind_card(row, "Small Blind", small_target, 3, true, false);

            // Big blind card
            spawn_blind_card(row, "Big Blind", big_target, 4, true, false);

            // Boss blind card
            let boss_display = format!("{}", boss_name);
            spawn_boss_blind_card(row, &boss_display, boss_target, 5);
        });

        // Hint text
        parent.spawn((
            Text::new("Skip Small/Big Blind to keep rewards for Boss"),
            TextFont { font_size: 18.0, ..default() },
            TextColor(Color::srgb(0.6, 0.6, 0.6)),
        ));
    });
}

fn spawn_blind_card(
    parent: &mut ChildBuilder,
    name: &str,
    target: u64,
    reward: i32,
    can_skip: bool,
    is_boss: bool,
) {
    let bg_color = if is_boss {
        Color::srgb(0.35, 0.1, 0.1)
    } else {
        Color::srgb(0.12, 0.18, 0.32)
    };

    parent.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Px(200.0),
            height: Val::Px(280.0),
            row_gap: Val::Px(10.0),
            border: UiRect::all(Val::Px(3.0)),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(bg_color),
        BorderColor(Color::srgb(0.5, 0.5, 0.7)),
    )).with_children(|card| {
        card.spawn((
            Text::new(name.to_string()),
            TextFont { font_size: 22.0, ..default() },
            TextColor(Color::WHITE),
        ));

        card.spawn((
            Text::new(format!("Target: {}", target)),
            TextFont { font_size: 18.0, ..default() },
            TextColor(Color::srgb(0.8, 0.8, 1.0)),
        ));

        card.spawn((
            Text::new(format!("Reward: ${}", reward)),
            TextFont { font_size: 18.0, ..default() },
            TextColor(Color::srgb(0.9, 0.8, 0.2)),
        ));

        // Play button
        if name == "Small Blind" {
            card.spawn((
                Button,
                Node {
                    width: Val::Px(140.0),
                    height: Val::Px(40.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.5, 0.15)),
                SelectSmallBlindButton,
            )).with_children(|btn| {
                btn.spawn((
                    Text::new("Play"),
                    TextFont { font_size: 20.0, ..default() },
                    TextColor(Color::WHITE),
                ));
            });

            if can_skip {
                card.spawn((
                    Button,
                    Node {
                        width: Val::Px(140.0),
                        height: Val::Px(40.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.4, 0.35, 0.1)),
                    SkipSmallBlindButton,
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("Skip"),
                        TextFont { font_size: 20.0, ..default() },
                        TextColor(Color::WHITE),
                    ));
                });
            }
        } else if name == "Big Blind" {
            card.spawn((
                Button,
                Node {
                    width: Val::Px(140.0),
                    height: Val::Px(40.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.5, 0.15)),
                SelectBigBlindButton,
            )).with_children(|btn| {
                btn.spawn((
                    Text::new("Play"),
                    TextFont { font_size: 20.0, ..default() },
                    TextColor(Color::WHITE),
                ));
            });

            if can_skip {
                card.spawn((
                    Button,
                    Node {
                        width: Val::Px(140.0),
                        height: Val::Px(40.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.4, 0.35, 0.1)),
                    SkipBigBlindButton,
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new("Skip"),
                        TextFont { font_size: 20.0, ..default() },
                        TextColor(Color::WHITE),
                    ));
                });
            }
        }
    });
}

fn spawn_boss_blind_card(
    parent: &mut ChildBuilder,
    name: &str,
    target: u64,
    reward: i32,
) {
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Px(200.0),
            height: Val::Px(280.0),
            row_gap: Val::Px(10.0),
            border: UiRect::all(Val::Px(3.0)),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.35, 0.1, 0.1)),
        BorderColor(Color::srgb(0.8, 0.3, 0.3)),
    )).with_children(|card| {
        card.spawn((
            Text::new("BOSS BLIND"),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::srgb(1.0, 0.4, 0.4)),
        ));

        card.spawn((
            Text::new(name.to_string()),
            TextFont { font_size: 22.0, ..default() },
            TextColor(Color::WHITE),
        ));

        card.spawn((
            Text::new(format!("Target: {}", target)),
            TextFont { font_size: 18.0, ..default() },
            TextColor(Color::srgb(0.8, 0.8, 1.0)),
        ));

        card.spawn((
            Text::new(format!("Reward: ${}", reward)),
            TextFont { font_size: 18.0, ..default() },
            TextColor(Color::srgb(0.9, 0.8, 0.2)),
        ));

        card.spawn((
            Button,
            Node {
                width: Val::Px(140.0),
                height: Val::Px(40.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.6, 0.15, 0.15)),
            SelectBossBlindButton,
        )).with_children(|btn| {
            btn.spawn((
                Text::new("Face Boss"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
    });
}

pub fn blind_select_buttons(
    small_query: Query<&Interaction, (Changed<Interaction>, With<SelectSmallBlindButton>)>,
    big_query: Query<&Interaction, (Changed<Interaction>, With<SelectBigBlindButton>)>,
    boss_query: Query<&Interaction, (Changed<Interaction>, With<SelectBossBlindButton>)>,
    skip_small_query: Query<&Interaction, (Changed<Interaction>, With<SkipSmallBlindButton>)>,
    skip_big_query: Query<&Interaction, (Changed<Interaction>, With<SkipBigBlindButton>)>,
    mut next_state: ResMut<NextState<crate::GameState>>,
    mut game_data: ResMut<GameData>,
    mut deck: ResMut<crate::deck::Deck>,
    mut hand: ResMut<crate::deck::Hand>,
    mut selected: ResMut<crate::deck::SelectedCards>,
    mut discard_pile: ResMut<crate::deck::DiscardPile>,
    jokers: Res<crate::jokers::OwnedJokers>,
) {
    let extra_hands = jokers.jokers.iter().filter(|j| j.id == crate::jokers::JokerId::Juggler).count() as u32;
    let extra_discards = jokers.jokers.iter().filter(|j| j.id == crate::jokers::JokerId::Drunkard).count() as u32;

    let mut start_round = |blind_type: BlindType, game_data: &mut GameData, deck: &mut crate::deck::Deck, hand: &mut crate::deck::Hand, selected: &mut crate::deck::SelectedCards, discard_pile: &mut crate::deck::DiscardPile| {
        game_data.blind_type = blind_type;
        let ante = game_data.ante;
        let round_num = match blind_type {
            BlindType::Small => 1u32,
            BlindType::Big => 2,
            BlindType::Boss => {
                game_data.boss_blind = Some(crate::game_data::BossBlind::from_index((ante as usize).wrapping_sub(1)));
                3
            }
        };
        game_data.blind_target = game_data.blind_target_for(ante, round_num);
        game_data.reset_for_new_round(extra_hands, extra_discards);

        // Reshuffle deck and draw
        *deck = crate::deck::Deck::new();
        crate::deck::shuffle_deck(deck);
        hand.cards.clear();
        hand.max_size = game_data.hand_size as usize;
        selected.clear();
        discard_pile.cards.clear();
        crate::deck::draw_to_hand(deck, hand);
    };

    for interaction in &small_query {
        if *interaction == Interaction::Pressed {
            start_round(BlindType::Small, &mut game_data, &mut deck, &mut hand, &mut selected, &mut discard_pile);
            next_state.set(crate::GameState::Playing);
        }
    }

    for interaction in &big_query {
        if *interaction == Interaction::Pressed {
            start_round(BlindType::Big, &mut game_data, &mut deck, &mut hand, &mut selected, &mut discard_pile);
            next_state.set(crate::GameState::Playing);
        }
    }

    for interaction in &boss_query {
        if *interaction == Interaction::Pressed {
            start_round(BlindType::Boss, &mut game_data, &mut deck, &mut hand, &mut selected, &mut discard_pile);
            next_state.set(crate::GameState::Playing);
        }
    }

    for interaction in &skip_small_query {
        if *interaction == Interaction::Pressed {
            // Skip small, go directly to big blind select (same screen but game_data updated)
            game_data.blind_type = BlindType::Big;
            game_data.blind_target = game_data.blind_target_for(game_data.ante, 2);
            // Stay in blind select (the screen will need to be refreshed)
            // For simplicity, just transition to playing with Big Blind
            start_round(BlindType::Big, &mut game_data, &mut deck, &mut hand, &mut selected, &mut discard_pile);
            next_state.set(crate::GameState::Playing);
        }
    }

    for interaction in &skip_big_query {
        if *interaction == Interaction::Pressed {
            // Skip big, go to boss blind
            start_round(BlindType::Boss, &mut game_data, &mut deck, &mut hand, &mut selected, &mut discard_pile);
            next_state.set(crate::GameState::Playing);
        }
    }
}
