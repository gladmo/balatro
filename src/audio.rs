use bevy::prelude::*;

use crate::game_state::AppState;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), play_menu_music);
    }
}

#[derive(Component)]
pub struct MenuMusic;

fn play_menu_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/music1.ogg")),
        PlaybackSettings::LOOP,
        MenuMusic,
    ));
}
