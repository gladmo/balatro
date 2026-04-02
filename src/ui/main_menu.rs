#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;
use bevy::ui::widget::ImageNode;
use crate::textures::GameTextures;
use crate::localization::{Localization, Language};

#[derive(Component)]
pub struct MainMenuRoot;

#[derive(Component)]
pub struct NewRunButton;

#[derive(Component)]
pub struct ContinueButton;

#[derive(Component)]
pub struct QuitButton;

#[derive(Component)]
pub struct HowToPlayButton;

#[derive(Component)]
pub struct LanguageToggleButton;

pub fn setup_main_menu(
    mut commands: Commands,
    textures: Option<Res<GameTextures>>,
    loc: Res<Localization>,
    fonts: Res<crate::ui::FontAssets>,
) {
    let lang = loc.language();
    let lang_label = match loc.language() {
        Language::English => "中文",
        Language::Chinese => "English",
    };

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(18.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.08, 0.08, 0.12)),
        MainMenuRoot,
    )).with_children(|parent| {
        // Logo: use balatro.png if textures loaded, otherwise text fallback
        if let Some(ref tex) = textures {
            parent.spawn((
                Node {
                    width: Val::Px(333.0),
                    height: Val::Px(216.0),
                    ..default()
                },
                ImageNode::new(tex.balatro_logo.clone()),
            ));
        } else {
            parent.spawn((
                Text::new(loc.get("menu.title")),
                TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 80.0, ..default() },
                TextColor(Color::srgb(0.9, 0.7, 0.1)),
            ));
        }

        parent.spawn((
            Text::new("A Poker Roguelite"),
            TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 22.0, ..default() },
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
        ));

        parent.spawn((Node { height: Val::Px(16.0), ..default() },));

        // New Run button
        parent.spawn((
            Button,
            Node {
                width: Val::Px(240.0),
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
                Text::new(loc.get("menu.new_run")),
                TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 26.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });

        // How to Play button
        parent.spawn((
            Button,
            Node {
                width: Val::Px(240.0),
                height: Val::Px(50.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.25, 0.45)),
            BorderColor::from(Color::srgb(0.3, 0.5, 0.8)),
            HowToPlayButton,
        )).with_children(|btn| {
            btn.spawn((
                Text::new(loc.get("menu.help")),
                TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 22.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });

        // Language toggle button
        parent.spawn((
            Button,
            Node {
                width: Val::Px(240.0),
                height: Val::Px(44.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.25, 0.2, 0.35)),
            BorderColor::from(Color::srgb(0.5, 0.4, 0.7)),
            LanguageToggleButton,
        )).with_children(|btn| {
            // Show the name of the opposite language so the user knows what they're switching to
            let label = format!("{} / {}", lang_label, loc.get("menu.language"));
            btn.spawn((
                Text::new(label),
                TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 18.0, ..default() },
                TextColor(Color::srgb(0.9, 0.85, 1.0)),
            ));
        });

        // Quit button
        parent.spawn((
            Button,
            Node {
                width: Val::Px(240.0),
                height: Val::Px(44.0),
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
                Text::new(loc.get("menu.quit")),
                TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 22.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
    });
}

pub fn main_menu_buttons(
    new_run_query: Query<&Interaction, (Changed<Interaction>, With<NewRunButton>)>,
    quit_query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    help_query: Query<&Interaction, (Changed<Interaction>, With<HowToPlayButton>)>,
    lang_query: Query<&Interaction, (Changed<Interaction>, With<LanguageToggleButton>)>,
    mut next_state: ResMut<NextState<crate::GameState>>,
    mut game_data: ResMut<crate::game_data::GameData>,
    mut deck: ResMut<crate::deck::Deck>,
    mut hand: ResMut<crate::deck::Hand>,
    mut selected: ResMut<crate::deck::SelectedCards>,
    mut discard_pile: ResMut<crate::deck::DiscardPile>,
    mut jokers: ResMut<crate::jokers::OwnedJokers>,
    mut hand_levels: ResMut<crate::hand_eval::HandLevels>,
    mut shop: ResMut<crate::shop::ShopState>,
    mut loc: ResMut<Localization>,
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

    for interaction in &help_query {
        if *interaction == Interaction::Pressed {
            next_state.set(crate::GameState::HelpScreen);
        }
    }

    for interaction in &lang_query {
        if *interaction == Interaction::Pressed {
            loc.toggle_language();
            // Reload the main menu to reflect the new language
            next_state.set(crate::GameState::MainMenu);
        }
    }

    for interaction in &quit_query {
        if *interaction == Interaction::Pressed {
            std::process::exit(0);
        }
    }
}
