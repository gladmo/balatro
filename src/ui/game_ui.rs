use bevy::prelude::*;

use crate::game_state::{AppState, BlindType, GameData};
use crate::hand_eval;
use crate::jokers::{self, JokerContext};
use crate::localization::Localization;
use super::{FontAssets, GameColors};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), setup_game_ui)
            .add_systems(
                Update,
                (
                    handle_card_selection,
                    handle_game_buttons,
                    update_game_info,
                )
                    .run_if(in_state(AppState::Playing)),
            )
            .add_systems(OnExit(AppState::Playing), cleanup_game_ui);
    }
}

#[derive(Component)]
struct GameUiRoot;

#[derive(Component)]
enum GameButton {
    PlayHand,
    Discard,
    SortRank,
    SortSuit,
}

#[derive(Component)]
struct HandCardButton(usize);

#[derive(Component)]
struct ScoreDisplay;

#[derive(Component)]
struct HandsDisplay;

#[derive(Component)]
struct DiscardsDisplay;

#[derive(Component)]
struct MoneyDisplay;

#[derive(Component)]
struct BlindInfoDisplay;

#[derive(Component)]
struct HandTypeDisplay;

fn setup_game_ui(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    loc: Res<Localization>,
    game_data: Res<GameData>,
) {
    let play_text = loc.get("play_hand");
    let discard_text = loc.get("discard");
    let sort_rank_text = loc.get("sort_rank");
    let sort_suit_text = loc.get("sort_suit");
    let hands_label = loc.get("hands");
    let discards_label = loc.get("discards");
    let money_label = loc.get("money");
    let blind_label = loc.get("blind");
    let font = fonts.english.clone();

    // Root
    let root = commands.spawn((
        GameUiRoot,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(GameColors::bg_dark()),
    )).id();

    // Top bar
    let top_bar = commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Px(80.0),
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Center,
        padding: UiRect::horizontal(Val::Px(20.0)),
        ..default()
    }).id();

    let blind_info = commands.spawn((
        Text::new(format!(
            "{}: {} - {}/{}",
            blind_label, game_data.current_blind,
            game_data.blind_chips_scored, game_data.blind_chips_required
        )),
        TextFont { font: font.clone(), font_size: 22.0, ..default() },
        TextColor(GameColors::accent_red()),
        BlindInfoDisplay,
    )).id();

    let money_info = commands.spawn((
        Text::new(format!("{}: ${}", money_label, game_data.money)),
        TextFont { font: font.clone(), font_size: 24.0, ..default() },
        TextColor(GameColors::accent_gold()),
        MoneyDisplay,
    )).id();

    commands.entity(top_bar).add_children(&[blind_info, money_info]);

    // Hand type display
    let hand_type_area = commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Px(40.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }).id();

    let hand_type_text = commands.spawn((
        Text::new("Select cards to play"),
        TextFont { font: font.clone(), font_size: 22.0, ..default() },
        TextColor(GameColors::text_dim()),
        HandTypeDisplay,
    )).id();
    commands.entity(hand_type_area).add_children(&[hand_type_text]);

    // Card area
    let card_area = commands.spawn(Node {
        width: Val::Percent(100.0),
        flex_grow: 1.0,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_wrap: FlexWrap::Wrap,
        column_gap: Val::Px(6.0),
        row_gap: Val::Px(6.0),
        padding: UiRect::horizontal(Val::Px(20.0)),
        ..default()
    }).id();

    let mut card_entities = Vec::new();
    for (i, card) in game_data.hand.iter().enumerate() {
        let card_color = match card.suit {
            crate::cards::Suit::Hearts | crate::cards::Suit::Diamonds => GameColors::accent_red(),
            _ => Color::srgb(0.15, 0.15, 0.2),
        };

        let top_label = commands.spawn((
            Text::new(format!("{}{}", card.rank.display(), card.suit.symbol())),
            TextFont { font: font.clone(), font_size: 20.0, ..default() },
            TextColor(card_color),
        )).id();

        let center_label = commands.spawn((
            Text::new(card.suit.symbol().to_string()),
            TextFont { font: font.clone(), font_size: 32.0, ..default() },
            TextColor(card_color),
        )).id();

        let bottom_label = commands.spawn((
            Text::new(card.rank.display().to_string()),
            TextFont { font: font.clone(), font_size: 16.0, ..default() },
            TextColor(card_color),
        )).id();

        let card_btn = commands.spawn((
            Button,
            Node {
                width: Val::Px(75.0),
                height: Val::Px(110.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(4.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(GameColors::card_bg()),
            BorderColor::from(if card.selected {
                GameColors::accent_gold()
            } else {
                GameColors::bg_medium()
            }),
            HandCardButton(i),
        )).id();

        commands.entity(card_btn).add_children(&[top_label, center_label, bottom_label]);
        card_entities.push(card_btn);
    }
    commands.entity(card_area).add_children(&card_entities);

    // Bottom bar
    let bottom_bar = commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Px(80.0),
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        column_gap: Val::Px(15.0),
        padding: UiRect::horizontal(Val::Px(20.0)),
        ..default()
    }).id();

    let hands_text = commands.spawn((
        Text::new(format!("{}: {}", hands_label, game_data.hands_remaining)),
        TextFont { font: font.clone(), font_size: 20.0, ..default() },
        TextColor(GameColors::accent_blue()),
        HandsDisplay,
    )).id();

    let sort_rank_btn = spawn_action_button(&mut commands, &sort_rank_text, font.clone(), GameButton::SortRank, GameColors::button_normal(), 120.0);
    let play_btn = spawn_action_button(&mut commands, &play_text, font.clone(), GameButton::PlayHand, GameColors::accent_blue(), 160.0);
    let discard_btn = spawn_action_button(&mut commands, &discard_text, font.clone(), GameButton::Discard, GameColors::accent_red(), 140.0);
    let sort_suit_btn = spawn_action_button(&mut commands, &sort_suit_text, font.clone(), GameButton::SortSuit, GameColors::button_normal(), 120.0);

    let discards_text = commands.spawn((
        Text::new(format!("{}: {}", discards_label, game_data.discards_remaining)),
        TextFont { font: font.clone(), font_size: 20.0, ..default() },
        TextColor(GameColors::accent_red()),
        DiscardsDisplay,
    )).id();

    commands.entity(bottom_bar).add_children(&[
        hands_text, sort_rank_btn, play_btn, discard_btn, sort_suit_btn, discards_text,
    ]);

    commands.entity(root).add_children(&[top_bar, hand_type_area, card_area, bottom_bar]);
}

fn spawn_action_button(
    commands: &mut Commands,
    text: &str,
    font: Handle<Font>,
    button: GameButton,
    color: Color,
    width: f32,
) -> Entity {
    let label = commands.spawn((
        Text::new(text.to_string()),
        TextFont { font, font_size: 22.0, ..default() },
        TextColor(GameColors::text_white()),
    )).id();

    let btn = commands.spawn((
        Button,
        Node {
            width: Val::Px(width),
            height: Val::Px(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(2.0)),
            border_radius: BorderRadius::all(Val::Px(8.0)),
            ..default()
        },
        BackgroundColor(color),
        BorderColor::from(GameColors::text_white()),
        button,
    )).id();
    commands.entity(btn).add_children(&[label]);
    btn
}

fn handle_card_selection(
    mut game_data: ResMut<GameData>,
    query: Query<(&Interaction, &HandCardButton), Changed<Interaction>>,
) {
    for (interaction, card_btn) in query.iter() {
        if *interaction == Interaction::Pressed {
            if let Some(card) = game_data.hand.get_mut(card_btn.0) {
                card.selected = !card.selected;
            }
        }
    }
}

fn handle_game_buttons(
    mut next_state: ResMut<NextState<AppState>>,
    mut game_data: ResMut<GameData>,
    query: Query<(&Interaction, &GameButton), Changed<Interaction>>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Pressed {
            match button {
                GameButton::PlayHand => {
                    let selected: Vec<_> = game_data.hand.iter().filter(|c| c.selected).cloned().collect();
                    if selected.is_empty() || game_data.hands_remaining == 0 {
                        return;
                    }

                    if let Some(eval) = hand_eval::evaluate_hand(&selected) {
                        let level = game_data.get_hand_level(&eval.hand_type);
                        let base_chips = level.base_chips;
                        let base_mult = level.base_mult;

                        let (chips, mult) = hand_eval::calculate_score(
                            &eval.hand_type, &eval.scoring_cards, base_chips, base_mult,
                        );

                        let ctx = JokerContext {
                            hand_type: eval.hand_type,
                            scoring_cards: &eval.scoring_cards,
                            all_hand_cards: &selected,
                            money: game_data.money,
                            discards_remaining: game_data.discards_remaining,
                        };
                        let (j_chips, j_mult, j_x_mult) =
                            jokers::apply_all_jokers(&game_data.jokers.clone(), &ctx);

                        let total_chips = (chips as i64 + j_chips).max(0) as u64;
                        let total_mult = ((mult as i64 + j_mult).max(0) as f64 * j_x_mult) as u64;
                        let score = total_chips * total_mult.max(1);

                        game_data.blind_chips_scored += score;
                        game_data.hands_remaining -= 1;
                        game_data.hand.retain(|c| !c.selected);

                        if game_data.blind_chips_scored >= game_data.blind_chips_required {
                            let earnings = game_data.calculate_earnings();
                            game_data.money += earnings;
                            game_data.return_hand_to_deck();

                            match game_data.current_blind {
                                BlindType::SmallBlind | BlindType::BigBlind => {
                                    next_state.set(AppState::Shop);
                                }
                                BlindType::BossBlind => {
                                    if game_data.ante >= 8 {
                                        game_data.game_won = true;
                                        next_state.set(AppState::GameOver);
                                    } else {
                                        game_data.ante += 1;
                                        next_state.set(AppState::Shop);
                                    }
                                }
                            }
                        } else if game_data.hands_remaining == 0 {
                            game_data.game_over = true;
                            next_state.set(AppState::GameOver);
                        }
                    }
                }
                GameButton::Discard => {
                    if game_data.discards_remaining > 0 {
                        let has_selected = game_data.hand.iter().any(|c| c.selected);
                        if has_selected {
                            game_data.discards_remaining -= 1;
                            game_data.discard_selected();
                            game_data.draw_hand();
                        }
                    }
                }
                GameButton::SortRank => {
                    game_data.hand.sort_by(|a, b| a.rank.cmp(&b.rank));
                }
                GameButton::SortSuit => {
                    game_data.hand.sort_by(|a, b| (a.suit as u8).cmp(&(b.suit as u8)));
                }
            }
        }
    }
}

fn update_game_info(
    game_data: Res<GameData>,
    loc: Res<Localization>,
    mut blind_q: Query<&mut Text, (With<BlindInfoDisplay>, Without<MoneyDisplay>, Without<HandsDisplay>, Without<DiscardsDisplay>, Without<HandTypeDisplay>)>,
    mut money_q: Query<&mut Text, (With<MoneyDisplay>, Without<BlindInfoDisplay>, Without<HandsDisplay>, Without<DiscardsDisplay>, Without<HandTypeDisplay>)>,
    mut hands_q: Query<&mut Text, (With<HandsDisplay>, Without<BlindInfoDisplay>, Without<MoneyDisplay>, Without<DiscardsDisplay>, Without<HandTypeDisplay>)>,
    mut discards_q: Query<&mut Text, (With<DiscardsDisplay>, Without<BlindInfoDisplay>, Without<MoneyDisplay>, Without<HandsDisplay>, Without<HandTypeDisplay>)>,
    mut hand_type_q: Query<&mut Text, (With<HandTypeDisplay>, Without<BlindInfoDisplay>, Without<MoneyDisplay>, Without<HandsDisplay>, Without<DiscardsDisplay>)>,
) {
    let blind_label = loc.get("blind");
    let money_label = loc.get("money");
    let hands_label = loc.get("hands");
    let discards_label = loc.get("discards");

    for mut text in blind_q.iter_mut() {
        **text = format!(
            "{}: {} - {}/{}",
            blind_label, game_data.current_blind,
            game_data.blind_chips_scored, game_data.blind_chips_required
        );
    }

    for mut text in money_q.iter_mut() {
        **text = format!("{}: ${}", money_label, game_data.money);
    }

    for mut text in hands_q.iter_mut() {
        **text = format!("{}: {}", hands_label, game_data.hands_remaining);
    }

    for mut text in discards_q.iter_mut() {
        **text = format!("{}: {}", discards_label, game_data.discards_remaining);
    }

    let selected: Vec<_> = game_data.hand.iter().filter(|c| c.selected).cloned().collect();
    let hand_type_text = if selected.is_empty() {
        "Select cards to play".to_string()
    } else if let Some(eval) = hand_eval::evaluate_hand(&selected) {
        let level = game_data.get_hand_level(&eval.hand_type);
        format!(
            "{} (Lvl {}) - {} × {}",
            eval.hand_type.display_name(), level.level, level.base_chips, level.base_mult
        )
    } else {
        "Invalid hand".to_string()
    };

    for mut text in hand_type_q.iter_mut() {
        **text = hand_type_text.clone();
    }
}

fn cleanup_game_ui(mut commands: Commands, query: Query<Entity, With<GameUiRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
