use bevy::prelude::*;

use crate::blinds::get_blind_chips;
use crate::game_state::{AppState, BlindType, GameData};
use crate::localization::Localization;
use super::{FontAssets, GameColors};

pub struct BlindSelectPlugin;

impl Plugin for BlindSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::BlindSelect), setup_blind_select)
            .add_systems(
                Update,
                handle_blind_buttons.run_if(in_state(AppState::BlindSelect)),
            )
            .add_systems(OnExit(AppState::BlindSelect), cleanup_blind_select);
    }
}

#[derive(Component)]
struct BlindSelectRoot;

#[derive(Component)]
enum BlindButton {
    SelectSmall,
    SelectBig,
    SelectBoss,
    SkipSmall,
    SkipBig,
}

fn setup_blind_select(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    loc: Res<Localization>,
    game_data: Res<GameData>,
) {
    let select_text = loc.get("select_blind");
    let ante_text = format!("{}: {}", loc.get("ante"), game_data.ante);
    let small_text = loc.get("small_blind");
    let big_text = loc.get("big_blind");
    let boss_text = loc.get("boss_blind");
    let skip_text = loc.get("skip_blind");
    let font = fonts.english.clone();

    let small_chips = get_blind_chips(game_data.ante, &BlindType::SmallBlind);
    let big_chips = get_blind_chips(game_data.ante, &BlindType::BigBlind);
    let boss_chips = get_blind_chips(game_data.ante, &BlindType::BossBlind);

    let root = commands
        .spawn((
            BlindSelectRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(GameColors::bg_dark()),
        ))
        .id();

    // Header
    let header = commands
        .spawn((
            Text::new(select_text),
            TextFont { font: font.clone(), font_size: 48.0, ..default() },
            TextColor(GameColors::text_white()),
            Node { margin: UiRect::bottom(Val::Px(10.0)), ..default() },
        ))
        .id();

    let ante_label = commands
        .spawn((
            Text::new(ante_text),
            TextFont { font: font.clone(), font_size: 24.0, ..default() },
            TextColor(GameColors::accent_gold()),
            Node { margin: UiRect::bottom(Val::Px(30.0)), ..default() },
        ))
        .id();

    // Row container
    let row = commands
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(20.0),
            ..default()
        })
        .id();

    // Blind cards
    let small_card = spawn_blind_card(
        &mut commands, &small_text, small_chips, GameColors::accent_blue(),
        font.clone(), BlindButton::SelectSmall, &skip_text, Some(BlindButton::SkipSmall), "$3",
    );
    let big_card = spawn_blind_card(
        &mut commands, &big_text, big_chips, GameColors::accent_green(),
        font.clone(), BlindButton::SelectBig, &skip_text, Some(BlindButton::SkipBig), "$4",
    );
    let boss_card = spawn_blind_card(
        &mut commands, &boss_text, boss_chips, GameColors::accent_red(),
        font.clone(), BlindButton::SelectBoss, &skip_text, None, "$5",
    );

    commands.entity(row).add_children(&[small_card, big_card, boss_card]);
    commands.entity(root).add_children(&[header, ante_label, row]);
}

fn spawn_blind_card(
    commands: &mut Commands,
    name: &str,
    chips: u64,
    color: Color,
    font: Handle<Font>,
    select_button: BlindButton,
    skip_text: &str,
    skip_button: Option<BlindButton>,
    reward: &str,
) -> Entity {
    let card = commands
        .spawn((
            Node {
                width: Val::Px(220.0),
                min_height: Val::Px(300.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(15.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(GameColors::panel()),
            BorderColor::from(color),
        ))
        .id();

    let name_text = commands.spawn((
        Text::new(name.to_string()),
        TextFont { font: font.clone(), font_size: 28.0, ..default() },
        TextColor(color),
        Node { margin: UiRect::bottom(Val::Px(15.0)), ..default() },
    )).id();

    let chips_text = commands.spawn((
        Text::new(format!("{} chips", chips)),
        TextFont { font: font.clone(), font_size: 20.0, ..default() },
        TextColor(GameColors::text_white()),
        Node { margin: UiRect::bottom(Val::Px(10.0)), ..default() },
    )).id();

    let reward_text = commands.spawn((
        Text::new(format!("Reward: {}", reward)),
        TextFont { font: font.clone(), font_size: 18.0, ..default() },
        TextColor(GameColors::accent_gold()),
        Node { margin: UiRect::bottom(Val::Px(20.0)), ..default() },
    )).id();

    // Select button
    let select_label = commands.spawn((
        Text::new("Select"),
        TextFont { font: font.clone(), font_size: 22.0, ..default() },
        TextColor(GameColors::text_white()),
    )).id();

    let select_btn = commands.spawn((
        Button,
        Node {
            width: Val::Px(160.0),
            height: Val::Px(40.0),
            margin: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(2.0)),
            border_radius: BorderRadius::all(Val::Px(6.0)),
            ..default()
        },
        BackgroundColor(color),
        BorderColor::from(GameColors::text_white()),
        select_button,
    )).id();
    commands.entity(select_btn).add_children(&[select_label]);

    let mut children = vec![name_text, chips_text, reward_text, select_btn];

    // Skip button
    if let Some(skip_btn_comp) = skip_button {
        let skip_label = commands.spawn((
            Text::new(skip_text.to_string()),
            TextFont { font: font.clone(), font_size: 18.0, ..default() },
            TextColor(GameColors::text_dim()),
        )).id();

        let skip_btn = commands.spawn((
            Button,
            Node {
                width: Val::Px(160.0),
                height: Val::Px(35.0),
                margin: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(GameColors::button_normal()),
            skip_btn_comp,
        )).id();
        commands.entity(skip_btn).add_children(&[skip_label]);
        children.push(skip_btn);
    }

    commands.entity(card).add_children(&children);
    card
}

fn handle_blind_buttons(
    mut next_state: ResMut<NextState<AppState>>,
    mut game_data: ResMut<GameData>,
    query: Query<(&Interaction, &BlindButton), Changed<Interaction>>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Pressed {
            let blind = match button {
                BlindButton::SelectSmall => BlindType::SmallBlind,
                BlindButton::SelectBig => BlindType::BigBlind,
                BlindButton::SelectBoss => BlindType::BossBlind,
                BlindButton::SkipSmall | BlindButton::SkipBig => return,
            };
            game_data.current_blind = blind;
            game_data.blind_chips_required = get_blind_chips(game_data.ante, &blind);
            game_data.blind_chips_scored = 0;
            game_data.hands_remaining = 4;
            game_data.discards_remaining = 3;
            game_data.shuffle_deck();
            game_data.draw_hand();
            next_state.set(AppState::Playing);
        }
    }
}

fn cleanup_blind_select(mut commands: Commands, query: Query<Entity, With<BlindSelectRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
