#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;
use crate::game_data::GameData;
use crate::shop::{ShopState, BuyItemEvent, SellJokerEvent, RerollShopEvent};
use crate::jokers::OwnedJokers;
use crate::consumables::ConsumableSlots;

#[derive(Component)]
pub struct ShopRoot;

#[derive(Component)]
pub struct ShopBuyButton {
    pub index: usize,
}

#[derive(Component)]
pub struct ShopSellButton {
    pub index: usize,
}

#[derive(Component)]
pub struct ShopRerollButton;

#[derive(Component)]
pub struct ShopContinueButton;

pub fn setup_shop(
    mut commands: Commands,
    mut shop: ResMut<ShopState>,
    game_data: Res<GameData>,
    jokers: Res<OwnedJokers>,
) {
    // Generate shop items
    let mut rng = rand::thread_rng();
    shop.generate(&mut rng, jokers.len(), game_data.joker_slots as usize);

    // Collect data before spawning UI
    let money = game_data.money;
    let blind_name = game_data.blind_name();
    let reward = game_data.blind_reward();
    let interest = game_data.interest();
    let reroll_cost = shop.reroll_cost;

    // Pre-collect shop item data for UI
    let shop_items: Vec<(String, String, i32)> = shop.items.iter().map(|item| {
        match item {
            Some(i) => (i.name(), i.description(), i.cost()),
            None => ("Empty".to_string(), "".to_string(), 0),
        }
    }).collect();

    let joker_data: Vec<(String, String, i32)> = jokers.jokers.iter().map(|j| {
        (j.name().to_string(), j.description().to_string(), j.sell_value)
    }).collect();

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            padding: UiRect::all(Val::Px(20.0)),
            row_gap: Val::Px(16.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.07, 0.07, 0.1)),
        ShopRoot,
    )).with_children(|root| {
        // Title row
        root.spawn((
            Node {
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
        )).with_children(|title_row| {
            title_row.spawn((
                Text::new("SHOP"),
                TextFont { font_size: 48.0, ..default() },
                TextColor(Color::srgb(0.9, 0.8, 0.2)),
            ));

            let info_text = format!("Money: ${} | Reward: ${} | Interest: ${}",
                money, reward, interest);
            title_row.spawn((
                Text::new(info_text),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });

        // After beating blind text
        let beat_text = format!("Beat {} — Earned ${} + ${} interest",
            blind_name, reward, interest);
        root.spawn((
            Text::new(beat_text),
            TextFont { font_size: 18.0, ..default() },
            TextColor(Color::srgb(0.6, 0.9, 0.6)),
        ));

        // Shop items
        root.spawn((
            Text::new("For Sale:"),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
        ));

        root.spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexStart,
                column_gap: Val::Px(16.0),
                ..default()
            },
        )).with_children(|items_row| {
            for (i, (name, desc, cost)) in shop_items.iter().enumerate() {
                if name == "Empty" {
                    continue;
                }
                let can_afford = money >= *cost;
                let bg = if can_afford {
                    Color::srgb(0.12, 0.18, 0.28)
                } else {
                    Color::srgb(0.15, 0.1, 0.1)
                };

                items_row.spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        width: Val::Px(180.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        row_gap: Val::Px(8.0),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(bg),
                    BorderColor(Color::srgb(0.4, 0.4, 0.6)),
                )).with_children(|item_card| {
                    item_card.spawn((
                        Text::new(name.clone()),
                        TextFont { font_size: 18.0, ..default() },
                        TextColor(Color::WHITE),
                    ));

                    item_card.spawn((
                        Text::new(desc.clone()),
                        TextFont { font_size: 13.0, ..default() },
                        TextColor(Color::srgb(0.7, 0.8, 0.9)),
                    ));

                    let price_text = format!("${}", cost);
                    item_card.spawn((
                        Text::new(price_text),
                        TextFont { font_size: 20.0, ..default() },
                        TextColor(if can_afford { Color::srgb(0.9, 0.8, 0.1) } else { Color::srgb(0.6, 0.3, 0.3) }),
                    ));

                    if can_afford {
                        item_card.spawn((
                            Button,
                            Node {
                                width: Val::Px(120.0),
                                height: Val::Px(35.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.15, 0.5, 0.15)),
                            ShopBuyButton { index: i },
                        )).with_children(|btn| {
                            btn.spawn((
                                Text::new("Buy"),
                                TextFont { font_size: 18.0, ..default() },
                                TextColor(Color::WHITE),
                            ));
                        });
                    }
                });
            }
        });

        // Owned jokers section (sell)
        if !joker_data.is_empty() {
            root.spawn((
                Text::new("Your Jokers (click to sell):"),
                TextFont { font_size: 22.0, ..default() },
                TextColor(Color::srgb(0.8, 0.7, 1.0)),
            ));

            root.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(10.0),
                    ..default()
                },
            )).with_children(|joker_row| {
                for (i, (name, desc, sell_val)) in joker_data.iter().enumerate() {
                    joker_row.spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            width: Val::Px(130.0),
                            padding: UiRect::all(Val::Px(8.0)),
                            row_gap: Val::Px(6.0),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.12, 0.3)),
                        BorderColor(Color::srgb(0.6, 0.4, 0.8)),
                    )).with_children(|jcard| {
                        jcard.spawn((
                            Text::new(name.clone()),
                            TextFont { font_size: 14.0, ..default() },
                            TextColor(Color::WHITE),
                        ));

                        jcard.spawn((
                            Text::new(desc.clone()),
                            TextFont { font_size: 11.0, ..default() },
                            TextColor(Color::srgb(0.7, 0.7, 0.9)),
                        ));

                        jcard.spawn((
                            Button,
                            Node {
                                width: Val::Px(100.0),
                                height: Val::Px(28.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.5, 0.15, 0.15)),
                            ShopSellButton { index: i },
                        )).with_children(|btn| {
                            btn.spawn((
                                Text::new(format!("Sell ${}", sell_val)),
                                TextFont { font_size: 14.0, ..default() },
                                TextColor(Color::WHITE),
                            ));
                        });
                    });
                }
            });
        }

        // Bottom buttons row
        root.spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(20.0),
                margin: UiRect::top(Val::Px(10.0)),
                ..default()
            },
        )).with_children(|btns| {
            btns.spawn((
                Button,
                Node {
                    width: Val::Px(160.0),
                    height: Val::Px(45.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.3, 0.25, 0.1)),
                BorderColor(Color::srgb(0.6, 0.5, 0.2)),
                ShopRerollButton,
            )).with_children(|btn| {
                btn.spawn((
                    Text::new(format!("Reroll (${reroll_cost})")),
                    TextFont { font_size: 18.0, ..default() },
                    TextColor(Color::WHITE),
                ));
            });

            btns.spawn((
                Button,
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(45.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.4, 0.6)),
                BorderColor(Color::srgb(0.3, 0.6, 0.9)),
                ShopContinueButton,
            )).with_children(|btn| {
                btn.spawn((
                    Text::new("Next Round"),
                    TextFont { font_size: 20.0, ..default() },
                    TextColor(Color::WHITE),
                ));
            });
        });
    });
}

pub fn shop_buttons(
    buy_query: Query<(&Interaction, &ShopBuyButton), Changed<Interaction>>,
    sell_query: Query<(&Interaction, &ShopSellButton), Changed<Interaction>>,
    reroll_query: Query<&Interaction, (Changed<Interaction>, With<ShopRerollButton>)>,
    continue_query: Query<&Interaction, (Changed<Interaction>, With<ShopContinueButton>)>,
    mut buy_events: EventWriter<BuyItemEvent>,
    mut sell_events: EventWriter<SellJokerEvent>,
    mut reroll_events: EventWriter<RerollShopEvent>,
    mut next_state: ResMut<NextState<crate::GameState>>,
    mut game_data: ResMut<GameData>,
    jokers: Res<OwnedJokers>,
) {
    for (interaction, btn) in &buy_query {
        if *interaction == Interaction::Pressed {
            buy_events.send(BuyItemEvent { index: btn.index });
        }
    }

    for (interaction, btn) in &sell_query {
        if *interaction == Interaction::Pressed {
            sell_events.send(SellJokerEvent { index: btn.index });
        }
    }

    for interaction in &reroll_query {
        if *interaction == Interaction::Pressed {
            reroll_events.send(RerollShopEvent);
        }
    }

    for interaction in &continue_query {
        if *interaction == Interaction::Pressed {
            // Award money for beating the blind
            let reward = game_data.blind_reward();
            let interest = game_data.interest();
            game_data.money += reward + interest;

            // Golden joker bonus
            let golden_count = jokers.jokers.iter().filter(|j| j.id == crate::jokers::JokerId::GoldenJoker).count();
            game_data.money += golden_count as i32 * 4;

            // Advance to next blind
            game_data.advance_blind();

            if game_data.is_game_won() {
                next_state.set(crate::GameState::Victory);
            } else {
                next_state.set(crate::GameState::BlindSelect);
            }
        }
    }
}
