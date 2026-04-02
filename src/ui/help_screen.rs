use bevy::prelude::*;
use crate::localization::Localization;

#[derive(Component)]
pub struct HelpRoot;

#[derive(Component)]
pub struct HelpCloseButton;

/// (i18n key, base chips, base mult)
const HAND_TABLE: &[(&str, u32, u32)] = &[
    ("hand.high_card",         5,   1),
    ("hand.pair",             10,   2),
    ("hand.two_pair",         20,   2),
    ("hand.three_of_a_kind",  30,   3),
    ("hand.straight",         30,   4),
    ("hand.flush",            35,   4),
    ("hand.full_house",       40,   4),
    ("hand.four_of_a_kind",   60,   7),
    ("hand.straight_flush",  100,   8),
    ("hand.five_of_a_kind",  120,  12),
    ("hand.flush_house",     140,  14),
    ("hand.flush_five",      160,  16),
];

const EDITION_TABLE: &[(&str, &str)] = &[
    ("Foil",         "+50 Chips"),
    ("Holographic",  "+10 Mult"),
    ("Polychrome",   "×1.5 Mult"),
    ("Negative",     "+1 Joker slot"),
];

const ENHANCEMENT_TABLE: &[(&str, &str)] = &[
    ("Bonus Card",    "+30 Chips"),
    ("Mult Card",     "+4 Mult"),
    ("Wild Card",     "Any suit"),
    ("Glass Card",    "×2 Mult (1-in-4 breaks)"),
    ("Steel Card",    "×1.5 Mult while in hand"),
    ("Stone Card",    "+50 Chips, no suit/rank"),
    ("Gold Card",     "+3$ at end of round"),
    ("Lucky Card",    "1-in-5: +20 Mult / 1-in-15: +20$"),
];

const SEAL_TABLE: &[(&str, &str)] = &[
    ("Gold Seal",   "+3$ when scored"),
    ("Red Seal",    "Retrigger card once"),
    ("Blue Seal",   "Create Planet card after hand"),
    ("Purple Seal", "Create Tarot card when discarded"),
];

pub fn setup_help_screen(
    mut commands: Commands,
    loc: Res<Localization>,
    fonts: Res<crate::ui::FontAssets>,
) {
    let lang = loc.language();
    let header_color = Color::srgb(0.95, 0.85, 0.2);
    let label_color  = Color::srgb(0.85, 0.85, 0.85);
    let value_color  = Color::WHITE;
    let section_bg   = Color::srgba(0.0, 0.0, 0.0, 0.35);
    let chip_color   = Color::srgb(0.4, 0.7, 1.0);
    let mult_color   = Color::srgb(1.0, 0.4, 0.4);

    // Collect all strings before spawning (avoid borrow through commands)
    let title_str   = loc.get("help.title").to_string();
    let scoring_str = loc.get("help.scoring").to_string();
    let close_str   = loc.get("help.close").to_string();

    let mut hand_names: Vec<String> = HAND_TABLE.iter()
        .map(|(k, _, _)| loc.get(k).to_string())
        .collect();

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            overflow: Overflow::scroll_y(),
            padding: UiRect::all(Val::Px(24.0)),
            row_gap: Val::Px(18.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.06, 0.07, 0.14)),
        HelpRoot,
    )).with_children(|root| {

        // ── Top close button (always visible on entry) ───────────────────────
        root.spawn((
            Button,
            Node {
                width: Val::Px(160.0),
                height: Val::Px(44.0),
                align_self: AlignSelf::FlexEnd,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.5, 0.15, 0.15)),
            BorderColor::from(Color::srgb(0.9, 0.3, 0.3)),
            HelpCloseButton,
        )).with_children(|btn| {
            btn.spawn((
                Text::new(close_str.clone()),
                TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 20.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });

        // Title
        root.spawn((
            Text::new(title_str),
            TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 40.0, ..default() },
            TextColor(header_color),
        ));

        // ── Scoring formula ──────────────────────────────────────────────────
        root.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(12.0)),
                row_gap: Val::Px(6.0),
                ..default()
            },
            BackgroundColor(section_bg),
        )).with_children(|section| {
            section.spawn((
                Text::new(scoring_str),
                TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 20.0, ..default() },
                TextColor(header_color),
            ));
            section.spawn((
                Text::new("Score = (Base Chips + Card Chips) × (Base Mult + Bonuses)"),
                TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 15.0, ..default() },
                TextColor(value_color),
            ));
            section.spawn((
                Text::new("Each played card adds its chip value to the chip pool.\nJokers can add flat Chips, flat Mult, or ×Mult multipliers."),
                TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 13.0, ..default() },
                TextColor(label_color),
            ));
        });

        // ── Hand type reference table ────────────────────────────────────────
        root.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(12.0)),
                row_gap: Val::Px(5.0),
                ..default()
            },
            BackgroundColor(section_bg),
        )).with_children(|section| {
            section.spawn((
                Text::new("Hand Types  (Base Chips × Base Mult)"),
                TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 19.0, ..default() },
                TextColor(header_color),
            ));
            // Header row
            section.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    border: UiRect::bottom(Val::Px(1.0)),
                    ..default()
                },
                BorderColor::from(Color::srgb(0.3, 0.3, 0.5)),
            )).with_children(|row| {
                row.spawn((Text::new("Hand"), TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 13.0, ..default() }, TextColor(label_color), Node { width: Val::Percent(60.0), ..default() }));
                row.spawn((Text::new("Chips"), TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 13.0, ..default() }, TextColor(chip_color), Node { width: Val::Percent(20.0), ..default() }));
                row.spawn((Text::new("Mult"),  TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 13.0, ..default() }, TextColor(mult_color), Node { width: Val::Percent(20.0), ..default() }));
            });
            // Data rows
            for ((_, chips, mult), name) in HAND_TABLE.iter().zip(hand_names.drain(..)) {
                let chips_str = chips.to_string();
                let mult_str = mult.to_string();
                section.spawn((
                    Node { width: Val::Percent(100.0), flex_direction: FlexDirection::Row, justify_content: JustifyContent::SpaceBetween, ..default() },
                )).with_children(|row| {
                    row.spawn((Text::new(name), TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 13.0, ..default() }, TextColor(value_color), Node { width: Val::Percent(60.0), ..default() }));
                    row.spawn((Text::new(chips_str), TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 13.0, ..default() }, TextColor(chip_color), Node { width: Val::Percent(20.0), ..default() }));
                    row.spawn((Text::new(mult_str),  TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 13.0, ..default() }, TextColor(mult_color), Node { width: Val::Percent(20.0), ..default() }));
                });
            }
        });

        // ── Card Editions ────────────────────────────────────────────────────
        root.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(12.0)),
                row_gap: Val::Px(5.0),
                ..default()
            },
            BackgroundColor(section_bg),
        )).with_children(|section| {
            section.spawn((Text::new("Card Editions"), TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 19.0, ..default() }, TextColor(header_color)));
            for (name, effect) in EDITION_TABLE {
                let row_text = format!("{:<18}→  {}", name, effect);
                section.spawn((Text::new(row_text), TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 13.0, ..default() }, TextColor(value_color)));
            }
        });

        // ── Card Enhancements ────────────────────────────────────────────────
        root.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(12.0)),
                row_gap: Val::Px(5.0),
                ..default()
            },
            BackgroundColor(section_bg),
        )).with_children(|section| {
            section.spawn((Text::new("Card Enhancements"), TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 19.0, ..default() }, TextColor(header_color)));
            for (name, effect) in ENHANCEMENT_TABLE {
                let row_text = format!("{:<18}→  {}", name, effect);
                section.spawn((Text::new(row_text), TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 13.0, ..default() }, TextColor(value_color)));
            }
        });

        // ── Card Seals ───────────────────────────────────────────────────────
        root.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(12.0)),
                row_gap: Val::Px(5.0),
                ..default()
            },
            BackgroundColor(section_bg),
        )).with_children(|section| {
            section.spawn((Text::new("Card Seals"), TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 19.0, ..default() }, TextColor(header_color)));
            for (name, effect) in SEAL_TABLE {
                let row_text = format!("{:<18}→  {}", name, effect);
                section.spawn((Text::new(row_text), TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 13.0, ..default() }, TextColor(value_color)));
            }
        });

        // ── Close button ─────────────────────────────────────────────────────
        root.spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.5)),
            BorderColor::from(Color::srgb(0.4, 0.4, 0.9)),
            HelpCloseButton,
        )).with_children(|btn| {
            btn.spawn((
                Text::new(close_str),
                TextFont { font: crate::ui::current_font(lang, &fonts), font_size: 22.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
    });
}

pub fn help_close_button(
    query: Query<&Interaction, (Changed<Interaction>, With<HelpCloseButton>)>,
    mut next_state: ResMut<NextState<crate::GameState>>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            next_state.set(crate::GameState::MainMenu);
        }
    }
}
