#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;
use bevy::ui::widget::ImageNode;
use bevy::image::TextureAtlas;
use crate::game_data::GameData;
use crate::deck::{Hand, SelectedCards, Deck, DiscardPile};
use crate::textures::GameTextures;
use crate::animation::{CardHoverAnim, CardSelectAnim, ButtonFlash, trigger_flash};
use crate::audio::{AudioAssets, play_sfx};

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
/// Index of the card this button represents, used for tooltip lookups.
pub struct HandCardButton {
    pub index: usize,
}

#[derive(Component)]
pub struct PlayHandButton;

#[derive(Component)]
pub struct DiscardButton;

#[derive(Component)]
pub struct HandCardsContainer;

/// Tooltip panel that shows card details on hover.
#[derive(Component)]
pub struct CardTooltip;

/// Marker to link a tooltip to the card index it describes.
#[derive(Component)]
pub struct CardTooltipText;

pub fn setup_game_ui(
    mut commands: Commands,
    game_data: Res<GameData>,
    hand: Res<Hand>,
    selected: Res<SelectedCards>,
    jokers: Res<crate::jokers::OwnedJokers>,
    textures: Option<Res<GameTextures>>,
    fonts: Res<crate::ui::FontAssets>,
    loc: Res<crate::localization::Localization>,
) {
    let lang = loc.language();
    // Pre-collect localized strings before spawning
    let score_vs_target_str = loc.get("ui.score_vs_target").to_string();
    let hands_label = loc.get("ui.hands").to_string();
    let discards_label = loc.get("ui.discards").to_string();
    let select_cards_str = loc.get("ui.select_cards").to_string();
    let play_hand_str = loc.get("ui.play_hand").to_string();
    let discard_str = loc.get("ui.discard").to_string();
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
                    TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 18.0, ..default() },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                ));

                left.spawn((
                    Text::new(game_data.blind_name()),
                    TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 22.0, ..default() },
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
                    TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 26.0, ..default() },
                    TextColor(Color::WHITE),
                    ScoreDisplay,
                ));
                center.spawn((
                    Text::new(score_vs_target_str),
                    TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 14.0, ..default() },
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
                    TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 22.0, ..default() },
                    TextColor(Color::srgb(0.9, 0.8, 0.1)),
                    MoneyDisplay,
                ));

                let hands_text = format!("{}: {}", hands_label, game_data.hands_remaining);
                right.spawn((
                    Text::new(hands_text),
                    TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 18.0, ..default() },
                    TextColor(Color::srgb(0.4, 0.8, 0.4)),
                    HandsDisplay,
                ));

                let discards_text = format!("{}: {}", discards_label, game_data.discards_remaining);
                right.spawn((
                    Text::new(discards_text),
                    TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 18.0, ..default() },
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
                    height: Val::Px(80.0),
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
                    let sprite_idx = GameTextures::joker_sprite_index(joker.id);
                    if let Some(ref tex) = textures {
                        // Show joker sprite from Jokers.png atlas
                        joker_row.spawn((
                            Node {
                                width: Val::Px(71.0),
                                height: Val::Px(95.0),
                                ..default()
                            },
                            ImageNode::from_atlas_image(
                                tex.jokers.clone(),
                                TextureAtlas { layout: tex.jokers_layout.clone(), index: sprite_idx },
                            ),
                        ));
                    } else {
                        // Fallback: text badge
                        joker_row.spawn((
                            Node {
                                width: Val::Px(71.0),
                                height: Val::Px(95.0),
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
                                TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 10.0, ..default() },
                                TextColor(Color::srgb(0.9, 0.8, 1.0)),
                            ));
                        });
                    }
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
                Text::new(select_cards_str),
                TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 20.0, ..default() },
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

        // Hand cards area — relative positioning allows animation via Node.top
        root.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(170.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexEnd,
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
                let sprite_idx = GameTextures::card_sprite_index(card.suit, card.rank);
                let select_color = if is_selected { Color::srgba(0.5, 0.7, 1.0, 0.75) } else { Color::WHITE };

                // Each card uses relative positioning so Node.top drives the lift animation
                let mut select_anim = CardSelectAnim::new();
                select_anim.selected_offset = if is_selected { 20.0 } else { 0.0 };
                select_anim.current = select_anim.selected_offset;

                if let Some(ref tex) = textures {
                    cards_area.spawn((
                        Button,
                        Node {
                            width: Val::Px(71.0),
                            height: Val::Px(95.0),
                            border: UiRect::all(Val::Px(3.0)),
                            position_type: PositionType::Relative,
                            ..default()
                        },
                        BorderColor::from(if is_selected {
                            Color::srgb(0.2, 0.7, 1.0)
                        } else {
                            Color::NONE
                        }),
                        ImageNode::from_atlas_image(
                            tex.cards.clone(),
                            TextureAtlas { layout: tex.cards_layout.clone(), index: sprite_idx },
                        ).with_color(select_color),
                        HandCardButton { index: i },
                        CardHoverAnim::new(),
                        select_anim,
                    ));
                } else {
                    let card_bg = if is_selected { Color::srgb(0.5, 0.6, 0.9) } else { Color::srgb(0.9, 0.88, 0.82) };
                    let card_label = card.short_display();
                    let suit_color = match card.suit {
                        crate::cards::Suit::Hearts | crate::cards::Suit::Diamonds => Color::srgb(0.8, 0.1, 0.1),
                        _ => Color::srgb(0.05, 0.05, 0.05),
                    };
                    cards_area.spawn((
                        Button,
                        Node {
                            width: Val::Px(70.0),
                            height: Val::Px(95.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            flex_direction: FlexDirection::Column,
                            position_type: PositionType::Relative,
                            ..default()
                        },
                        BackgroundColor(card_bg),
                        BorderColor::from(if is_selected { Color::srgb(0.2, 0.4, 1.0) } else { Color::srgb(0.3, 0.3, 0.3) }),
                        HandCardButton { index: i },
                    )).with_children(|card_btn| {
                        card_btn.spawn((
                            Text::new(card_label),
                            TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 20.0, ..default() },
                            TextColor(suit_color),
                        ));
                    });
                }
            }
        });

        // Bottom action buttons with ButtonFlash animation
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
            let play_base = Color::srgb(0.15, 0.5, 0.15);
            let play_flash = Color::srgb(0.5, 1.0, 0.5);
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
                BackgroundColor(play_base),
                BorderColor::from(Color::srgb(0.3, 0.8, 0.3)),
                PlayHandButton,
                ButtonFlash::new(play_flash, play_base),
            )).with_children(|btn| {
                btn.spawn((
                    Text::new(play_hand_str),
                    TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 22.0, ..default() },
                    TextColor(Color::WHITE),
                ));
            });

            let disc_base = Color::srgb(0.5, 0.25, 0.05);
            let disc_flash = Color::srgb(1.0, 0.6, 0.2);
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
                BackgroundColor(disc_base),
                BorderColor::from(Color::srgb(0.8, 0.5, 0.1)),
                DiscardButton,
                ButtonFlash::new(disc_flash, disc_base),
            )).with_children(|btn| {
                btn.spawn((
                    Text::new(discard_str),
                    TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 22.0, ..default() },
                    TextColor(Color::WHITE),
                ));
            });
        });

        // Floating tooltip panel (initially hidden; shown by hover system)
        root.spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(185.0),
                left: Val::Px(20.0),
                padding: UiRect::all(Val::Px(8.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(2.0),
                min_width: Val::Px(120.0),
                display: Display::None, // hidden by default
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.1, 0.92)),
            BorderColor::from(Color::srgb(0.5, 0.5, 0.8)),
            ZIndex(100),
            CardTooltip,
        )).with_children(|tip| {
            tip.spawn((
                Text::new(""),
                TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 14.0, ..default() },
                TextColor(Color::WHITE),
                CardTooltipText,
            ));
        });
    });
}

pub fn update_score_display(
    game_data: Res<GameData>,
    loc: Res<crate::localization::Localization>,
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
        *text = Text::new(format!("{}: {}", loc.get("ui.hands"), game_data.hands_remaining));
    }
    for mut text in &mut discards_query {
        *text = Text::new(format!("{}: {}", loc.get("ui.discards"), game_data.discards_remaining));
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
    textures: Option<Res<GameTextures>>,
    fonts: Res<crate::ui::FontAssets>,
    loc: Res<crate::localization::Localization>,
) {
    let lang = loc.language();
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
                *text = Text::new(loc.get(eval.hand_type.loc_key()).to_string());
            }
        }
    }

    // Rebuild hand cards
    for entity in &container_query {
        for (btn_entity, _) in &card_button_query {
            commands.entity(btn_entity).despawn();
        }

        commands.entity(entity).with_children(|cards_area| {
            for (i, card) in hand.cards.iter().enumerate() {
                let is_selected = selected.contains(i);
                let sprite_idx = GameTextures::card_sprite_index(card.suit, card.rank);
                let select_color = if is_selected { Color::srgba(0.5, 0.7, 1.0, 0.75) } else { Color::WHITE };

                let mut sel_anim = CardSelectAnim::new();
                sel_anim.selected_offset = if is_selected { 20.0 } else { 0.0 };
                sel_anim.current = sel_anim.selected_offset;

                if let Some(ref tex) = textures {
                    cards_area.spawn((
                        Button,
                        Node {
                            width: Val::Px(71.0),
                            height: Val::Px(95.0),
                            border: UiRect::all(Val::Px(3.0)),
                            position_type: PositionType::Relative,
                            ..default()
                        },
                        BorderColor::from(if is_selected { Color::srgb(0.2, 0.7, 1.0) } else { Color::NONE }),
                        ImageNode::from_atlas_image(
                            tex.cards.clone(),
                            TextureAtlas { layout: tex.cards_layout.clone(), index: sprite_idx },
                        ).with_color(select_color),
                        HandCardButton { index: i },
                        CardHoverAnim::new(),
                        sel_anim,
                    ));
                } else {
                    let card_bg = if is_selected { Color::srgb(0.5, 0.6, 0.9) } else { Color::srgb(0.9, 0.88, 0.82) };
                    let card_label = card.short_display();
                    let suit_color = match card.suit {
                        crate::cards::Suit::Hearts | crate::cards::Suit::Diamonds => Color::srgb(0.8, 0.1, 0.1),
                        _ => Color::srgb(0.05, 0.05, 0.05),
                    };
                    cards_area.spawn((
                        Button,
                        Node {
                            width: Val::Px(70.0),
                            height: Val::Px(95.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            flex_direction: FlexDirection::Column,
                            position_type: PositionType::Relative,
                            ..default()
                        },
                        BackgroundColor(card_bg),
                        BorderColor::from(if is_selected { Color::srgb(0.2, 0.4, 1.0) } else { Color::srgb(0.3, 0.3, 0.3) }),
                        HandCardButton { index: i },
                    )).with_children(|card_btn| {
                        card_btn.spawn((
                            Text::new(card_label),
                            TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 20.0, ..default() },
                            TextColor(suit_color),
                        ));
                    });
                }
            }
        });
    }
}

/// Handles card selection toggle + selection animation + SFX.
pub fn card_selection_buttons(
    mut query: Query<(&Interaction, &HandCardButton, &mut CardSelectAnim, &mut BorderColor), Changed<Interaction>>,
    mut selected: ResMut<SelectedCards>,
    mut commands: Commands,
    audio: Option<Res<AudioAssets>>,
) {
    for (interaction, btn, mut anim, mut border) in &mut query {
        if *interaction == Interaction::Pressed {
            selected.toggle(btn.index);
            let is_sel = selected.contains(btn.index);
            // Animate card up/down
            anim.selected_offset = if is_sel { 20.0 } else { 0.0 };
            // Highlight border
            *border = BorderColor::from(if is_sel { Color::srgb(0.2, 0.7, 1.0) } else { Color::NONE });
            // Play SFX
            if let Some(ref a) = audio {
                play_sfx(&mut commands, a.card_select.clone());
            }
        }
    }
}

/// Shows/hides and updates the card tooltip based on which card is hovered.
pub fn update_card_tooltip(
    hand: Res<Hand>,
    hover_query: Query<(&Interaction, &HandCardButton), Changed<Interaction>>,
    mut tooltip_query: Query<&mut Node, With<CardTooltip>>,
    mut tooltip_text_query: Query<&mut Text, With<CardTooltipText>>,
) {
    for (interaction, btn) in &hover_query {
        let Some(card) = hand.cards.get(btn.index) else { continue };
        match interaction {
            Interaction::Hovered => {
                // Build tooltip text from the card's attributes
                let mut lines = vec![
                    format!("{} of {}", card.rank.name(), card.suit.name()),
                    format!("Chips: {}", card.base_chip_value()),
                ];
                if card.enhancement != crate::cards::Enhancement::None {
                    lines.push(format!("Enhancement: {:?}", card.enhancement));
                }
                if card.edition != crate::cards::Edition::None {
                    lines.push(format!("Edition: {:?}", card.edition));
                }
                if card.seal != crate::cards::Seal::None {
                    lines.push(format!("Seal: {:?}", card.seal));
                }
                let text = lines.join("\n");
                if let Ok(mut t) = tooltip_text_query.single_mut() {
                    *t = Text::new(text);
                }
                if let Ok(mut n) = tooltip_query.single_mut() {
                    n.display = Display::Flex;
                }
            }
            _ => {
                if let Ok(mut n) = tooltip_query.single_mut() {
                    n.display = Display::None;
                }
            }
        }
    }
}

/// Handles Play/Discard button presses with SFX and button flash effects.
pub fn game_buttons(
    // Explicit Without<> on each query makes the disjointness visible to Bevy's static checker
    // and prevents the B0001 conflict on &mut BackgroundColor / &mut ButtonFlash.
    mut play_query: Query<(&Interaction, &mut BackgroundColor, &mut ButtonFlash), (Changed<Interaction>, With<PlayHandButton>, Without<DiscardButton>)>,
    mut discard_query: Query<(&Interaction, &mut BackgroundColor, &mut ButtonFlash), (Changed<Interaction>, With<DiscardButton>, Without<PlayHandButton>)>,
    mut game_data: ResMut<GameData>,
    mut deck: ResMut<Deck>,
    mut hand: ResMut<Hand>,
    mut selected: ResMut<SelectedCards>,
    mut discard_pile: ResMut<DiscardPile>,
    jokers: Res<crate::jokers::OwnedJokers>,
    mut hand_levels: ResMut<crate::hand_eval::HandLevels>,
    mut next_state: ResMut<NextState<crate::GameState>>,
    mut commands: Commands,
    audio: Option<Res<AudioAssets>>,
) {
    for (interaction, mut bg, mut flash) in &mut play_query {
        if *interaction == Interaction::Pressed {
            if !selected.is_empty() && game_data.hands_remaining > 0 {
                // Flash the button
                trigger_flash(&mut flash, &mut bg);
                // Play SFX
                if let Some(ref a) = audio {
                    play_sfx(&mut commands, a.card_play.clone());
                }
                if let Some(new_state) = crate::scoring::execute_play_hand(
                    &mut game_data, &mut deck, &mut hand, &mut selected,
                    &mut discard_pile, &jokers, &mut hand_levels,
                ) {
                    next_state.set(new_state);
                }
            }
        }
    }

    for (interaction, mut bg, mut flash) in &mut discard_query {
        if *interaction == Interaction::Pressed {
            if !selected.is_empty() && game_data.discards_remaining > 0 {
                trigger_flash(&mut flash, &mut bg);
                if let Some(ref a) = audio {
                    play_sfx(&mut commands, a.card_discard.clone());
                }
                crate::scoring::execute_discard(
                    &mut game_data, &mut deck, &mut hand, &mut selected, &mut discard_pile,
                );
            }
        }
    }
}
