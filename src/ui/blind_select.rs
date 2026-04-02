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
    fonts: Res<crate::ui::FontAssets>,
    loc: Res<crate::localization::Localization>,
) {
    let lang = loc.language();
    let ante = game_data.ante;
    let small_target = game_data.blind_target_for(ante, 1);
    let big_target = game_data.blind_target_for(ante, 2);
    let boss_target = game_data.blind_target_for(ante, 3);
    let boss_name = crate::game_data::BossBlind::from_index((ante as usize).wrapping_sub(1)).name().to_string();

    // Pre-collect localized strings before spawning
    let title = format!("{} — {} {}", loc.get("blind.select"), loc.get("ui.ante"), ante);
    let small_blind_str = loc.get("blind.small").to_string();
    let big_blind_str = loc.get("blind.big").to_string();
    let boss_blind_str = loc.get("blind.boss").to_string();
    let target_str = loc.get("ui.target").to_string();
    let reward_str = loc.get("blind.reward").to_string();
    let play_str = loc.get("blind.play").to_string();
    let skip_str = loc.get("blind.skip").to_string();
    let face_boss_str = loc.get("blind.face_boss").to_string();
    let skip_hint_str = loc.get("blind.skip_hint").to_string();

    let small_target_text = format!("{}: {}", target_str, small_target);
    let big_target_text = format!("{}: {}", target_str, big_target);
    let boss_target_text = format!("{}: {}", target_str, boss_target);
    let small_reward_text = format!("{}: $3", reward_str);
    let big_reward_text = format!("{}: $4", reward_str);
    let boss_reward_text = format!("{}: $5", reward_str);

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
        parent.spawn((
            Text::new(title),
            TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 40.0, ..default() },
            TextColor(Color::srgb(0.9, 0.85, 0.3)),
        ));

        // Row of 3 blind cards
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
            row.spawn((
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
                BackgroundColor(Color::srgb(0.12, 0.18, 0.32)),
                BorderColor::from(Color::srgb(0.5, 0.5, 0.7)),
            )).with_children(|card| {
                card.spawn((
                    Text::new(small_blind_str),
                    TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 22.0, ..default() },
                    TextColor(Color::WHITE),
                ));
                card.spawn((
                    Text::new(small_target_text),
                    TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 18.0, ..default() },
                    TextColor(Color::srgb(0.8, 0.8, 1.0)),
                ));
                card.spawn((
                    Text::new(small_reward_text),
                    TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 18.0, ..default() },
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
                    BackgroundColor(Color::srgb(0.15, 0.5, 0.15)),
                    SelectSmallBlindButton,
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new(play_str.clone()),
                        TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 20.0, ..default() },
                        TextColor(Color::WHITE),
                    ));
                });
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
                        Text::new(skip_str.clone()),
                        TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 20.0, ..default() },
                        TextColor(Color::WHITE),
                    ));
                });
            });

            // Big blind card
            row.spawn((
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
                BackgroundColor(Color::srgb(0.12, 0.18, 0.32)),
                BorderColor::from(Color::srgb(0.5, 0.5, 0.7)),
            )).with_children(|card| {
                card.spawn((
                    Text::new(big_blind_str),
                    TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 22.0, ..default() },
                    TextColor(Color::WHITE),
                ));
                card.spawn((
                    Text::new(big_target_text),
                    TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 18.0, ..default() },
                    TextColor(Color::srgb(0.8, 0.8, 1.0)),
                ));
                card.spawn((
                    Text::new(big_reward_text),
                    TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 18.0, ..default() },
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
                    BackgroundColor(Color::srgb(0.15, 0.5, 0.15)),
                    SelectBigBlindButton,
                )).with_children(|btn| {
                    btn.spawn((
                        Text::new(play_str.clone()),
                        TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 20.0, ..default() },
                        TextColor(Color::WHITE),
                    ));
                });
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
                        Text::new(skip_str.clone()),
                        TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 20.0, ..default() },
                        TextColor(Color::WHITE),
                    ));
                });
            });

            // Boss blind card
            row.spawn((
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
                BorderColor::from(Color::srgb(0.8, 0.3, 0.3)),
            )).with_children(|card| {
                card.spawn((
                    Text::new(boss_blind_str),
                    TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 16.0, ..default() },
                    TextColor(Color::srgb(1.0, 0.4, 0.4)),
                ));
                card.spawn((
                    Text::new(boss_name),
                    TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 22.0, ..default() },
                    TextColor(Color::WHITE),
                ));
                card.spawn((
                    Text::new(boss_target_text),
                    TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 18.0, ..default() },
                    TextColor(Color::srgb(0.8, 0.8, 1.0)),
                ));
                card.spawn((
                    Text::new(boss_reward_text),
                    TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 18.0, ..default() },
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
                        Text::new(face_boss_str),
                        TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 20.0, ..default() },
                        TextColor(Color::WHITE),
                    ));
                });
            });
        });

        parent.spawn((
            Text::new(skip_hint_str),
            TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 18.0, ..default() },
            TextColor(Color::srgb(0.6, 0.6, 0.6)),
        ));
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

    macro_rules! start_round {
        ($blind_type:expr) => {{
            let blind_type = $blind_type;
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
            *deck = crate::deck::Deck::new();
            crate::deck::shuffle_deck(&mut deck);
            hand.cards.clear();
            hand.max_size = game_data.hand_size as usize;
            selected.clear();
            discard_pile.cards.clear();
            crate::deck::draw_to_hand(&mut deck, &mut hand);
        }};
    }

    for interaction in &small_query {
        if *interaction == Interaction::Pressed {
            start_round!(BlindType::Small);
            next_state.set(crate::GameState::Playing);
        }
    }
    for interaction in &big_query {
        if *interaction == Interaction::Pressed {
            start_round!(BlindType::Big);
            next_state.set(crate::GameState::Playing);
        }
    }
    for interaction in &boss_query {
        if *interaction == Interaction::Pressed {
            start_round!(BlindType::Boss);
            next_state.set(crate::GameState::Playing);
        }
    }
    for interaction in &skip_small_query {
        if *interaction == Interaction::Pressed {
            start_round!(BlindType::Big);
            next_state.set(crate::GameState::Playing);
        }
    }
    for interaction in &skip_big_query {
        if *interaction == Interaction::Pressed {
            start_round!(BlindType::Boss);
            next_state.set(crate::GameState::Playing);
        }
    }
}
