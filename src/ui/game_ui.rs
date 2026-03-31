#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;
use crate::game_data::GameData;
use crate::deck::{Hand, SelectedCards, Deck, DiscardPile};

#[derive(Component)]
pub struct GameUiRoot;

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct HandsDisplay;

#[derive(Component)]
pub struct DiscardsDisplay;

#[derive(Component)]
pub struct MoneyDisplay;

#[derive(Component)]
pub struct HandTypeDisplay;

#[derive(Component)]
pub struct HandCardButton {
    pub index: usize,
}

#[derive(Component)]
pub struct PlayHandButton;

#[derive(Component)]
pub struct DiscardButton;

#[derive(Component)]
pub struct HandCardsContainer;

pub fn setup_game_ui(
    mut commands: Commands,
    game_data: Res<GameData>,
    hand: Res<Hand>,
    selected: Res<SelectedCards>,
    jokers: Res<crate::jokers::OwnedJokers>,
) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Stretch,
            ..default()
        },
        BackgroundColor(Color::srgb(0.08, 0.1, 0.12)),
        GameUiRoot,
    )).with_children(|root| {
        // Top HUD bar
        root.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(80.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.07, 0.1)),
        )).with_children(|hud| {
            // Left: Ante / Round / Blind
            hud.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
            )).with_children(|left| {
                let ante_text = format!("Ante {} / Round {}", game_data.ante, game_data.round);
                left.spawn((
                    Text::new(ante_text),
                    TextFont { font_size: 18.0, ..default() },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                ));

                left.spawn((
                    Text::new(game_data.blind_name()),
                    TextFont { font_size: 22.0, ..default() },
                    TextColor(Color::srgb(0.9, 0.7, 0.2)),
                ));
            });

            // Center: Score vs Target
            hud.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
            )).with_children(|center| {
                let score_text = format!("{} / {}", game_data.score, game_data.blind_target);
                center.spawn((
                    Text::new(score_text),
                    TextFont { font_size: 26.0, ..default() },
                    TextColor(Color::WHITE),
                    ScoreDisplay,
                ));
                center.spawn((
                    Text::new("Score / Target"),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(Color::srgb(0.6, 0.6, 0.6)),
                ));
            });

            // Right: Money, Hands, Discards
            hud.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexEnd,
                    row_gap: Val::Px(4.0),
                    ..default()
                },
            )).with_children(|right| {
                let money_text = format!("${}", game_data.money);
                right.spawn((
                    Text::new(money_text),
                    TextFont { font_size: 22.0, ..default() },
                    TextColor(Color::srgb(0.9, 0.8, 0.1)),
                    MoneyDisplay,
                ));

                let hands_text = format!("Hands: {}", game_data.hands_remaining);
                right.spawn((
                    Text::new(hands_text),
                    TextFont { font_size: 18.0, ..default() },
                    TextColor(Color::srgb(0.4, 0.8, 0.4)),
                    HandsDisplay,
                ));

                let discards_text = format!("Discards: {}", game_data.discards_remaining);
                right.spawn((
                    Text::new(discards_text),
                    TextFont { font_size: 18.0, ..default() },
                    TextColor(Color::srgb(0.8, 0.5, 0.3)),
                    DiscardsDisplay,
                ));
            });
        });

        // Joker row
        if !jokers.jokers.is_empty() {
            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(60.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(8.0),
                    padding: UiRect::horizontal(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.1, 0.08, 0.12)),
            )).with_children(|joker_row| {
                for joker in &jokers.jokers {
                    joker_row.spawn((
                        Node {
                            width: Val::Px(100.0),
                            height: Val::Px(50.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.25, 0.15, 0.35)),
                        BorderColor::from(Color::srgb(0.6, 0.4, 0.8)),
                    )).with_children(|j| {
                        j.spawn((
                            Text::new(joker.name().to_string()),
                            TextFont { font_size: 12.0, ..default() },
                            TextColor(Color::srgb(0.9, 0.8, 1.0)),
                        ));
                    });
                }
            });
        }

        // Hand type display
        root.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        )).with_children(|ht| {
            ht.spawn((
                Text::new("Select cards to play"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::srgb(0.7, 0.9, 0.7)),
                HandTypeDisplay,
            ));
        });

        // Spacer
        root.spawn((
            Node {
                flex_grow: 1.0,
                ..default()
            },
        ));

        // Hand cards area
        root.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(160.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(8.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.06, 0.09, 0.08)),
            HandCardsContainer,
        )).with_children(|cards_area| {
            for (i, card) in hand.cards.iter().enumerate() {
                let is_selected = selected.contains(i);
                let card_bg = if is_selected {
                    Color::srgb(0.5, 0.6, 0.9)
                } else {
                    Color::srgb(0.9, 0.88, 0.82)
                };

                let card_label = card.short_display();
                let suit_color = match card.suit {
                    crate::cards::Suit::Hearts | crate::cards::Suit::Diamonds => Color::srgb(0.8, 0.1, 0.1),
                    _ => Color::srgb(0.05, 0.05, 0.05),
                };

                cards_area.spawn((
                    Button,
                    Node {
                        width: Val::Px(70.0),
                        height: Val::Px(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(card_bg),
                    BorderColor::from(if is_selected { Color::srgb(0.2, 0.4, 1.0) } else { Color::srgb(0.3, 0.3, 0.3) }),
                    HandCardButton { index: i },
                )).with_children(|card_btn| {
                    card_btn.spawn((
                        Text::new(card_label),
                        TextFont { font_size: 20.0, ..default() },
                        TextColor(suit_color),
                    ));
                });
            }
        });

        // Bottom action buttons
        root.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(70.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(20.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.05, 0.08)),
        )).with_children(|actions| {
            actions.spawn((
                Button,
                Node {
                    width: Val::Px(160.0),
                    height: Val::Px(50.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.5, 0.15)),
                BorderColor::from(Color::srgb(0.3, 0.8, 0.3)),
                PlayHandButton,
            )).with_children(|btn| {
                btn.spawn((
                    Text::new("Play Hand"),
                    TextFont { font_size: 22.0, ..default() },
                    TextColor(Color::WHITE),
                ));
            });

            actions.spawn((
                Button,
                Node {
                    width: Val::Px(160.0),
                    height: Val::Px(50.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.5, 0.25, 0.05)),
                BorderColor::from(Color::srgb(0.8, 0.5, 0.1)),
                DiscardButton,
            )).with_children(|btn| {
                btn.spawn((
                    Text::new("Discard"),
                    TextFont { font_size: 22.0, ..default() },
                    TextColor(Color::WHITE),
                ));
            });
        });
    });
}

pub fn update_score_display(
    game_data: Res<GameData>,
    mut score_query: Query<&mut Text, With<ScoreDisplay>>,
    mut hands_query: Query<&mut Text, (With<HandsDisplay>, Without<ScoreDisplay>)>,
    mut discards_query: Query<&mut Text, (With<DiscardsDisplay>, Without<ScoreDisplay>, Without<HandsDisplay>)>,
    mut money_query: Query<&mut Text, (With<MoneyDisplay>, Without<ScoreDisplay>, Without<HandsDisplay>, Without<DiscardsDisplay>)>,
) {
    if !game_data.is_changed() {
        return;
    }

    for mut text in &mut score_query {
        *text = Text::new(format!("{} / {}", game_data.score, game_data.blind_target));
    }
    for mut text in &mut hands_query {
        *text = Text::new(format!("Hands: {}", game_data.hands_remaining));
    }
    for mut text in &mut discards_query {
        *text = Text::new(format!("Discards: {}", game_data.discards_remaining));
    }
    for mut text in &mut money_query {
        *text = Text::new(format!("${}", game_data.money));
    }
}

pub fn update_hand_display(
    mut commands: Commands,
    hand: Res<Hand>,
    selected: Res<SelectedCards>,
    mut hand_type_query: Query<&mut Text, With<HandTypeDisplay>>,
    container_query: Query<Entity, With<HandCardsContainer>>,
    card_button_query: Query<(Entity, &HandCardButton)>,
) {
    if !hand.is_changed() && !selected.is_changed() {
        return;
    }

    // Update hand type display
    if !selected.is_empty() {
        let selected_cards: Vec<&crate::cards::Card> = selected.indices.iter()
            .filter_map(|&i| hand.cards.get(i))
            .collect();

        if !selected_cards.is_empty() {
            let eval = crate::hand_eval::evaluate_hand(
                &selected_cards.iter().map(|c| (*c).clone()).collect::<Vec<_>>()
            );
            for mut text in &mut hand_type_query {
                *text = Text::new(eval.hand_type.name().to_string());
            }
        }
    }

    // Rebuild hand cards
    for entity in &container_query {
        // Remove old card buttons
        for (btn_entity, _) in &card_button_query {
            commands.entity(btn_entity).despawn();
        }

        // Spawn new card buttons
        commands.entity(entity).with_children(|cards_area| {
            for (i, card) in hand.cards.iter().enumerate() {
                let is_selected = selected.contains(i);
                let card_bg = if is_selected {
                    Color::srgb(0.5, 0.6, 0.9)
                } else {
                    Color::srgb(0.9, 0.88, 0.82)
                };

                let card_label = card.short_display();
                let suit_color = match card.suit {
                    crate::cards::Suit::Hearts | crate::cards::Suit::Diamonds => Color::srgb(0.8, 0.1, 0.1),
                    _ => Color::srgb(0.05, 0.05, 0.05),
                };

                cards_area.spawn((
                    Button,
                    Node {
                        width: Val::Px(70.0),
                        height: Val::Px(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(card_bg),
                    BorderColor::from(if is_selected { Color::srgb(0.2, 0.4, 1.0) } else { Color::srgb(0.3, 0.3, 0.3) }),
                    HandCardButton { index: i },
                )).with_children(|card_btn| {
                    card_btn.spawn((
                        Text::new(card_label),
                        TextFont { font_size: 20.0, ..default() },
                        TextColor(suit_color),
                    ));
                });
            }
        });
    }
}

pub fn card_selection_buttons(
    query: Query<(&Interaction, &HandCardButton), Changed<Interaction>>,
    mut selected: ResMut<SelectedCards>,
) {
    for (interaction, btn) in &query {
        if *interaction == Interaction::Pressed {
            selected.toggle(btn.index);
        }
    }
}

pub fn game_buttons(
    play_query: Query<&Interaction, (Changed<Interaction>, With<PlayHandButton>)>,
    discard_query: Query<&Interaction, (Changed<Interaction>, With<DiscardButton>)>,
    mut game_data: ResMut<GameData>,
    mut deck: ResMut<Deck>,
    mut hand: ResMut<Hand>,
    mut selected: ResMut<SelectedCards>,
    mut discard_pile: ResMut<DiscardPile>,
    jokers: Res<crate::jokers::OwnedJokers>,
    mut hand_levels: ResMut<crate::hand_eval::HandLevels>,
    mut next_state: ResMut<NextState<crate::GameState>>,
) {
    for interaction in &play_query {
        if *interaction == Interaction::Pressed {
            if !selected.is_empty() && game_data.hands_remaining > 0 {
                if let Some(new_state) = crate::scoring::execute_play_hand(
                    &mut game_data,
                    &mut deck,
                    &mut hand,
                    &mut selected,
                    &mut discard_pile,
                    &jokers,
                    &mut hand_levels,
                ) {
                    next_state.set(new_state);
                }
            }
        }
    }

    for interaction in &discard_query {
        if *interaction == Interaction::Pressed {
            if !selected.is_empty() && game_data.discards_remaining > 0 {
                crate::scoring::execute_discard(
                    &mut game_data,
                    &mut deck,
                    &mut hand,
                    &mut selected,
                    &mut discard_pile,
                );
            }
        }
    }
}
