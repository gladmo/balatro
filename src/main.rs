use bevy::prelude::*;

mod animation;
mod audio;
mod blinds;
mod cards;
mod game_state;
mod hand_eval;
mod jokers;
mod localization;
mod save;
mod shop;
mod textures;
mod ui;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: bevy::window::WindowResolution::new(1280, 800),
                        title: "Balatro".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: "resources".to_string(),
                    ..default()
                }),
        )
        .init_state::<game_state::AppState>()
        .insert_resource(game_state::GameData::default())
        .add_plugins(localization::LocalizationPlugin)
        .add_plugins(textures::TexturePlugin)
        .add_plugins(audio::GameAudioPlugin)
        .add_plugins(animation::AnimationPlugin)
        .add_plugins(ui::BalatroUiPlugin)
        .add_plugins(shop::ShopPlugin)
        .add_plugins(blinds::BlindsPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
