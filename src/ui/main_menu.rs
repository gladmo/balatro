use bevy::prelude::*;

use crate::game_state::{AppState, GameData};
use crate::localization::Localization;
use crate::save;
use super::{FontAssets, GameColors};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                handle_menu_buttons.run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), cleanup_main_menu);
    }
}

#[derive(Component)]
struct MainMenuRoot;

#[derive(Component)]
enum MenuButton {
    NewRun,
    Continue,
    Help,
    Quit,
}

fn setup_main_menu(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    loc: Res<Localization>,
) {
    let title_text = loc.get("title");
    let new_run_text = loc.get("new_run");
    let continue_text = loc.get("continue_run");
    let help_text = loc.get("help");
    let quit_text = loc.get("quit");
    let font = fonts.english.clone();

    let root = commands
        .spawn((
            MainMenuRoot,
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

    // Title
    let title = commands
        .spawn((
            Text::new(title_text),
            TextFont {
                font: font.clone(),
                font_size: 96.0,
                ..default()
            },
            TextColor(GameColors::accent_gold()),
            Node {
                margin: UiRect::bottom(Val::Px(60.0)),
                ..default()
            },
        ))
        .id();
    commands.entity(root).add_children(&[title]);

    // Version
    let version = commands
        .spawn((
            Text::new("v0.1.0 - Bevy Edition"),
            TextFont {
                font: font.clone(),
                font_size: 18.0,
                ..default()
            },
            TextColor(GameColors::text_dim()),
            Node {
                margin: UiRect::bottom(Val::Px(40.0)),
                ..default()
            },
        ))
        .id();
    commands.entity(root).add_children(&[version]);

    // Buttons
    let new_run_btn = spawn_menu_button(&mut commands, &new_run_text, font.clone(), MenuButton::NewRun);
    commands.entity(root).add_children(&[new_run_btn]);

    if save::has_save() {
        let cont_btn = spawn_menu_button(&mut commands, &continue_text, font.clone(), MenuButton::Continue);
        commands.entity(root).add_children(&[cont_btn]);
    }

    let help_btn = spawn_menu_button(&mut commands, &help_text, font.clone(), MenuButton::Help);
    let quit_btn = spawn_menu_button(&mut commands, &quit_text, font.clone(), MenuButton::Quit);
    commands.entity(root).add_children(&[help_btn, quit_btn]);
}

fn spawn_menu_button(commands: &mut Commands, text: &str, font: Handle<Font>, button: MenuButton) -> Entity {
    let text_entity = commands
        .spawn((
            Text::new(text.to_string()),
            TextFont {
                font,
                font_size: 28.0,
                ..default()
            },
            TextColor(GameColors::text_white()),
        ))
        .id();

    let btn = commands
        .spawn((
            Button,
            Node {
                width: Val::Px(280.0),
                height: Val::Px(55.0),
                margin: UiRect::all(Val::Px(8.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(GameColors::button_normal()),
            BorderColor::from(GameColors::accent_blue()),
            button,
        ))
        .id();
    commands.entity(btn).add_children(&[text_entity]);
    btn
}

fn handle_menu_buttons(
    mut next_state: ResMut<NextState<AppState>>,
    mut game_data: ResMut<GameData>,
    query: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Pressed {
            match button {
                MenuButton::NewRun => {
                    game_data.new_run();
                    next_state.set(AppState::BlindSelect);
                }
                MenuButton::Continue => {
                    if let Ok(data) = save::load_game() {
                        *game_data = data;
                        next_state.set(AppState::Playing);
                    }
                }
                MenuButton::Help => {}
                MenuButton::Quit => {
                    exit.write(AppExit::Success);
                }
            }
        }
    }
}

fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
