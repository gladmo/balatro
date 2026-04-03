#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;
use bevy::ui::widget::ImageNode;
use bevy::image::TextureAtlas;
use crate::game_data::GameData;
use crate::shop::{ShopState, ShopItem};
use crate::jokers::OwnedJokers;
use crate::consumables::{ConsumableSlots, Consumable};
use crate::textures::GameTextures;

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
    textures: Option<Res<GameTextures>>,
    fonts: Res<crate::ui::FontAssets>,
    loc: Res<crate::localization::Localization>,
) {
    let lang = loc.language();
    // Generate shop items
    let mut rng = rand::thread_rng();
    shop.generate(&mut rng, jokers.len(), game_data.joker_slots as usize);

    // Collect data before spawning UI
    let money = game_data.money;
    let blind_name = game_data.blind_name();
    let reward = game_data.blind_reward();
    let interest = game_data.interest();
    let reroll_cost = shop.reroll_cost;

    // Pre-collect shop item data + sprite indices
    struct ShopItemUiData {
        name: String,
        desc: String,
        cost: i32,
        joker_sprite: Option<usize>,   // index into Jokers atlas
        tarot_sprite: Option<usize>,   // index into Tarots atlas
    }
    let shop_items: Vec<ShopItemUiData> = shop.items.iter().map(|item| {
        match item {
            Some(ShopItem::JokerItem(j)) => ShopItemUiData {
                name: j.name().to_string(),
                desc: j.description().to_string(),
                cost: j.cost,
                joker_sprite: Some(GameTextures::joker_sprite_index(j.id)),
                tarot_sprite: None,
            },
            Some(ShopItem::ConsumableItem(Consumable::Tarot(t))) => ShopItemUiData {
                name: t.name().to_string(),
                desc: t.description().to_string(),
                cost: Consumable::Tarot(*t).cost(),
                joker_sprite: None,
                tarot_sprite: Some(GameTextures::tarot_sprite_index(*t)),
            },
            Some(ShopItem::ConsumableItem(Consumable::Planet(p))) => ShopItemUiData {
                name: p.name().to_string(),
                desc: p.description().to_string(),
                cost: Consumable::Planet(*p).cost(),
                joker_sprite: None,
                tarot_sprite: Some(GameTextures::planet_sprite_index(*p)),
            },
            None => ShopItemUiData {
                name: "Empty".to_string(),
                desc: "".to_string(),
                cost: 0,
                joker_sprite: None,
                tarot_sprite: None,
            },
        }
    }).collect();

    // Owned joker data + sprite indices
    let joker_data: Vec<(String, String, i32, usize)> = jokers.jokers.iter().map(|j| {
        (j.name().to_string(), j.description().to_string(), j.sell_value, GameTextures::joker_sprite_index(j.id))
    }).collect();

    // Pre-collect all localized strings before spawning (avoid borrow conflict with commands)
    let shop_title = loc.get("shop.title").to_string();
    let info_text = format!("{}: ${} | {}: ${} | {}: ${}",
        loc.get("shop.money_label"), money,
        loc.get("blind.reward"), reward,
        loc.get("shop.interest_label"), interest);
    let beat_text = format!("{} {} — {} ${} + ${} {}",
        loc.get("shop.beat_blind"), blind_name,
        loc.get("shop.earned"), reward, interest,
        loc.get("shop.interest_label"));
    let for_sale_str = loc.get("shop.for_sale").to_string();
    let buy_str = loc.get("shop.buy").to_string();
    let your_jokers_str = loc.get("shop.your_jokers").to_string();
    let sell_btn_str = loc.get("shop.sell_btn").to_string();
    let reroll_str = format!("{} (${})", loc.get("shop.reroll_btn"), reroll_cost);
    let continue_str = loc.get("shop.continue").to_string();

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
                Text::new(shop_title),
                TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 48.0, ..default() },
                TextColor(Color::srgb(0.9, 0.8, 0.2)),
            ));

            title_row.spawn((
                Text::new(info_text),
                TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 18.0, ..default() },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });

        // After beating blind text
        root.spawn((
            Text::new(beat_text),
            TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 18.0, ..default() },
            TextColor(Color::srgb(0.6, 0.9, 0.6)),
        ));

        // Shop items
        root.spawn((
            Text::new(for_sale_str),
            TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 24.0, ..default() },
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
            for (i, item) in shop_items.iter().enumerate() {
                if item.name == "Empty" {
                    continue;
                }
                let can_afford = money >= item.cost;
                let bg = if can_afford {
                    Color::srgb(0.12, 0.18, 0.28)
                } else {
                    Color::srgb(0.15, 0.1, 0.1)
                };

                items_row.spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        width: Val::Px(100.0),
                        padding: UiRect::all(Val::Px(8.0)),
                        row_gap: Val::Px(6.0),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(bg),
                    BorderColor::from(Color::srgb(0.4, 0.4, 0.6)),
                )).with_children(|item_card| {
                    // Show sprite if textures are available
                    if let Some(ref tex) = textures {
                        if let Some(idx) = item.joker_sprite {
                            item_card.spawn((
                                Node { width: Val::Px(71.0), height: Val::Px(95.0), ..default() },
                                ImageNode::from_atlas_image(
                                    tex.jokers.clone(),
                                    TextureAtlas { layout: tex.jokers_layout.clone(), index: idx },
                                ),
                            ));
                        } else if let Some(idx) = item.tarot_sprite {
                            item_card.spawn((
                                Node { width: Val::Px(71.0), height: Val::Px(95.0), ..default() },
                                ImageNode::from_atlas_image(
                                    tex.tarots.clone(),
                                    TextureAtlas { layout: tex.tarots_layout.clone(), index: idx },
                                ),
                            ));
                        }
                    } else {
                        item_card.spawn((
                            Text::new(item.name.clone()),
                            TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 14.0, ..default() },
                            TextColor(Color::WHITE),
                        ));
                        item_card.spawn((
                            Text::new(item.desc.clone()),
                            TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 11.0, ..default() },
                            TextColor(Color::srgb(0.7, 0.8, 0.9)),
                        ));
                    }

                    let price_text = format!("${}", item.cost);
                    item_card.spawn((
                        Text::new(price_text),
                        TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 18.0, ..default() },
                        TextColor(if can_afford { Color::srgb(0.9, 0.8, 0.1) } else { Color::srgb(0.6, 0.3, 0.3) }),
                    ));

                    if can_afford {
                        item_card.spawn((
                            Button,
                            Node {
                                width: Val::Px(80.0),
                                height: Val::Px(30.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.15, 0.5, 0.15)),
                            ShopBuyButton { index: i },
                        )).with_children(|btn| {
                            btn.spawn((
                                Text::new(buy_str.clone()),
                                TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 16.0, ..default() },
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
                Text::new(your_jokers_str),
                TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 22.0, ..default() },
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
                for (i, (name, desc, sell_val, sprite_idx)) in joker_data.iter().enumerate() {
                    joker_row.spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            width: Val::Px(100.0),
                            padding: UiRect::all(Val::Px(6.0)),
                            row_gap: Val::Px(4.0),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.12, 0.3)),
                        BorderColor::from(Color::srgb(0.6, 0.4, 0.8)),
                    )).with_children(|jcard| {
                        if let Some(ref tex) = textures {
                            jcard.spawn((
                                Node { width: Val::Px(71.0), height: Val::Px(95.0), ..default() },
                                ImageNode::from_atlas_image(
                                    tex.jokers.clone(),
                                    TextureAtlas { layout: tex.jokers_layout.clone(), index: *sprite_idx },
                                ),
                            ));
                        } else {
                            jcard.spawn((
                                Text::new(name.clone()),
                                TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 12.0, ..default() },
                                TextColor(Color::WHITE),
                            ));
                        }

                        jcard.spawn((
                            Button,
                            Node {
                                width: Val::Px(85.0),
                                height: Val::Px(26.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.5, 0.15, 0.15)),
                            ShopSellButton { index: i },
                        )).with_children(|btn| {
                            btn.spawn((
                                Text::new(format!("{} ${}", sell_btn_str, sell_val)),
                                TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 13.0, ..default() },
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
                BorderColor::from(Color::srgb(0.6, 0.5, 0.2)),
                ShopRerollButton,
            )).with_children(|btn| {
                btn.spawn((
                    Text::new(reroll_str),
                    TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 18.0, ..default() },
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
                BorderColor::from(Color::srgb(0.3, 0.6, 0.9)),
                ShopContinueButton,
            )).with_children(|btn| {
                btn.spawn((
                    Text::new(continue_str),
                    TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 20.0, ..default() },
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
    mut shop: ResMut<ShopState>,
    mut game_data: ResMut<GameData>,
    mut jokers: ResMut<OwnedJokers>,
    mut consumables: ResMut<ConsumableSlots>,
    mut next_state: ResMut<NextState<crate::GameState>>,
) {
    for (interaction, btn) in &buy_query {
        if *interaction == Interaction::Pressed {
            crate::shop::execute_buy(&mut shop, &mut game_data, &mut jokers, &mut consumables, btn.index);
        }
    }

    for (interaction, btn) in &sell_query {
        if *interaction == Interaction::Pressed {
            crate::shop::execute_sell(&mut jokers, &mut game_data, btn.index);
        }
    }

    for interaction in &reroll_query {
        if *interaction == Interaction::Pressed {
            crate::shop::execute_reroll(&mut shop, &mut game_data, &jokers);
        }
    }

    for interaction in &continue_query {
        if *interaction == Interaction::Pressed {
            let reward = game_data.blind_reward();
            let interest = game_data.interest();
            game_data.money += reward + interest;

            let golden_count = jokers.jokers.iter().filter(|j| j.id == crate::jokers::JokerId::GoldenJoker).count();
            game_data.money += golden_count as i32 * 4;

            game_data.advance_blind();

            if game_data.is_game_won() {
                next_state.set(crate::GameState::Victory);
            } else {
                next_state.set(crate::GameState::BlindSelect);
            }
        }
    }
}
