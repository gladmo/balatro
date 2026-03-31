#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;

#[derive(Resource)]
pub struct AudioAssets {
    pub card_play: Option<Handle<AudioSource>>,
    pub card_select: Option<Handle<AudioSource>>,
    pub button_click: Option<Handle<AudioSource>>,
    pub win: Option<Handle<AudioSource>>,
    pub lose: Option<Handle<AudioSource>>,
    pub shop: Option<Handle<AudioSource>>,
}

impl Default for AudioAssets {
    fn default() -> Self {
        AudioAssets {
            card_play: None,
            card_select: None,
            button_click: None,
            win: None,
            lose: None,
            shop: None,
        }
    }
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioAssets>();
        // Audio loading would happen here if audio files exist
    }
}
