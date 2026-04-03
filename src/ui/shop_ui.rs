use bevy::prelude::*;

use crate::game_state::{AppState, GameData};
use crate::localization::Localization;
use crate::shop::ShopInventory;
use super::{FontAssets, GameColors};

pub struct ShopUiPlugin;

impl Plugin for ShopUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Shop), setup_shop_ui)
            .add_systems(
                Update,
                handle_shop_buttons.run_if(in_state(AppState::Shop)),
            )
            .add_systems(OnExit(AppState::Shop), cleanup_shop_ui);
    }
}

#[derive(Component)]
struct ShopUiRoot;

#[derive(Component)]
enum ShopButton {
    BuyItem(usize),
    Reroll,
    NextRound,
}

fn setup_shop_ui(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    loc: Res<Localization>,
    game_data: Res<GameData>,
    inventory: Res<ShopInventory>,
) {
    let shop_title = loc.get("shop");
    let reroll_text = loc.get("reroll");
    let next_round_text = loc.get("next_round");
    let money_text = format!("${}", game_data.money);
    let font = fonts.english.clone();

    let root = commands.spawn((
        ShopUiRoot,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        BackgroundColor(GameColors::bg_dark()),
    )).id();

    // Header
    let header = commands.spawn(Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Center,
        margin: UiRect::bottom(Val::Px(20.0)),
        ..default()
    }).id();

    let title = commands.spawn((
        Text::new(shop_title),
        TextFont { font: font.clone(), font_size: 48.0, ..default() },
        TextColor(GameColors::accent_gold()),
    )).id();

    let money = commands.spawn((
        Text::new(money_text),
        TextFont { font: font.clone(), font_size: 36.0, ..default() },
        TextColor(GameColors::accent_gold()),
    )).id();

    commands.entity(header).add_children(&[title, money]);

    // Items row
    let items_row = commands.spawn(Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        column_gap: Val::Px(15.0),
        flex_grow: 1.0,
        ..default()
    }).id();

    let mut item_cards = Vec::new();
    for (i, item) in inventory.items.iter().enumerate() {
        let card = commands.spawn((
            Node {
                width: Val::Px(160.0),
                min_height: Val::Px(200.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(GameColors::panel()),
            BorderColor::from(GameColors::accent_blue()),
        )).id();

        let name_label = commands.spawn((
            Text::new(item.display_name()),
            TextFont { font: font.clone(), font_size: 18.0, ..default() },
            TextColor(GameColors::text_white()),
            Node { margin: UiRect::bottom(Val::Px(10.0)), ..default() },
        )).id();

        let cost_label = commands.spawn((
            Text::new(format!("${}", item.cost())),
            TextFont { font: font.clone(), font_size: 22.0, ..default() },
            TextColor(GameColors::accent_gold()),
            Node { margin: UiRect::bottom(Val::Px(10.0)), ..default() },
        )).id();

        let buy_label = commands.spawn((
            Text::new("Buy"),
            TextFont { font: font.clone(), font_size: 18.0, ..default() },
            TextColor(GameColors::text_white()),
        )).id();

        let buy_btn = commands.spawn((
            Button,
            Node {
                width: Val::Px(100.0),
                height: Val::Px(35.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(GameColors::accent_green()),
            BorderColor::from(GameColors::text_white()),
            ShopButton::BuyItem(i),
        )).id();
        commands.entity(buy_btn).add_children(&[buy_label]);
        commands.entity(card).add_children(&[name_label, cost_label, buy_btn]);
        item_cards.push(card);
    }
    commands.entity(items_row).add_children(&item_cards);

    // Bottom buttons
    let bottom = commands.spawn(Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        column_gap: Val::Px(20.0),
        margin: UiRect::top(Val::Px(20.0)),
        ..default()
    }).id();

    let reroll_label = commands.spawn((
        Text::new(format!("{} (${})", reroll_text, game_data.reroll_cost)),
        TextFont { font: font.clone(), font_size: 22.0, ..default() },
        TextColor(GameColors::text_white()),
    )).id();
    let reroll_btn = commands.spawn((
        Button,
        Node {
            width: Val::Px(180.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(2.0)),
            border_radius: BorderRadius::all(Val::Px(8.0)),
            ..default()
        },
        BackgroundColor(GameColors::accent_purple()),
        BorderColor::from(GameColors::text_white()),
        ShopButton::Reroll,
    )).id();
    commands.entity(reroll_btn).add_children(&[reroll_label]);

    let next_label = commands.spawn((
        Text::new(next_round_text),
        TextFont { font: font.clone(), font_size: 22.0, ..default() },
        TextColor(GameColors::text_white()),
    )).id();
    let next_btn = commands.spawn((
        Button,
        Node {
            width: Val::Px(200.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(2.0)),
            border_radius: BorderRadius::all(Val::Px(8.0)),
            ..default()
        },
        BackgroundColor(GameColors::accent_blue()),
        BorderColor::from(GameColors::text_white()),
        ShopButton::NextRound,
    )).id();
    commands.entity(next_btn).add_children(&[next_label]);

    commands.entity(bottom).add_children(&[reroll_btn, next_btn]);
    commands.entity(root).add_children(&[header, items_row, bottom]);
}

fn handle_shop_buttons(
    mut next_state: ResMut<NextState<AppState>>,
    mut game_data: ResMut<GameData>,
    inventory: Res<ShopInventory>,
    query: Query<(&Interaction, &ShopButton), Changed<Interaction>>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Pressed {
            match button {
                ShopButton::BuyItem(idx) => {
                    if let Some(item) = inventory.items.get(*idx) {
                        crate::shop::try_purchase(&mut game_data, item);
                    }
                }
                ShopButton::Reroll => {
                    let cost = game_data.reroll_cost as i32;
                    if game_data.money >= cost {
                        game_data.money -= cost;
                    }
                }
                ShopButton::NextRound => {
                    game_data.round += 1;
                    next_state.set(AppState::BlindSelect);
                }
            }
        }
    }
}

fn cleanup_shop_ui(mut commands: Commands, query: Query<Entity, With<ShopUiRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
