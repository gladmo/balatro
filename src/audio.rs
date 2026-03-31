#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;

/// Handles for every sound effect and music track used in the game.
/// Mirrors the original project's audio resource approach.
#[derive(Resource)]
pub struct AudioAssets {
    // Music tracks (music1-5.ogg)
    pub music: Vec<Handle<AudioSource>>,

    // Card SFX
    pub card_select:  Handle<AudioSource>,  // highlight1.ogg — card toggled (selected/deselected)
    pub card_flip:    Handle<AudioSource>,  // cardSlide1.ogg — card moved to hand
    pub card_play:    Handle<AudioSource>,  // card1.ogg      — hand played
    pub card_discard: Handle<AudioSource>,  // crumple1.ogg   — card discarded

    // UI SFX
    pub button_click: Handle<AudioSource>,  // button.ogg
    pub cancel:       Handle<AudioSource>,  // cancel.ogg

    // Shop / score SFX
    pub coin:         Handle<AudioSource>,  // coin1.ogg
    pub chips:        Handle<AudioSource>,  // chips1.ogg
}

/// Marker component placed on the background music entity so we can find it.
#[derive(Component)]
pub struct BackgroundMusic;

/// Startup system: load all audio assets from resources/sounds/ and begin
/// playing background music, mirroring the original project's audio setup.
pub fn load_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    let music_handles: Vec<Handle<AudioSource>> = (1..=5)
        .map(|i| asset_server.load(format!("sounds/music{}.ogg", i)))
        .collect();

    // Start playing the first music track looped
    let first_track = music_handles[0].clone();
    commands.spawn((
        AudioPlayer::new(first_track),
        PlaybackSettings::LOOP,
        BackgroundMusic,
    ));

    commands.insert_resource(AudioAssets {
        music: music_handles,
        card_select:  asset_server.load("sounds/highlight1.ogg"),
        card_flip:    asset_server.load("sounds/cardSlide1.ogg"),
        card_play:    asset_server.load("sounds/card1.ogg"),
        card_discard: asset_server.load("sounds/crumple1.ogg"),
        button_click: asset_server.load("sounds/button.ogg"),
        cancel:       asset_server.load("sounds/cancel.ogg"),
        coin:         asset_server.load("sounds/coin1.ogg"),
        chips:        asset_server.load("sounds/chips1.ogg"),
    });
}

/// Helper: play a one-shot sound effect by spawning a temporary audio entity.
pub fn play_sfx(commands: &mut Commands, handle: Handle<AudioSource>) {
    commands.spawn((
        AudioPlayer::new(handle),
        PlaybackSettings::DESPAWN,
    ));
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, load_audio);
    }
}
