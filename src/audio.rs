use bevy::prelude::*;
use std::collections::HashMap;

use crate::game_state::AppState;

// ---------------------------------------------------------------------------
// Sound effect identifier – one variant per .ogg file shipped with the game.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SoundEvent {
    // Music
    Music1,
    Music2,
    Music3,
    Music4,
    Music5,
    // Cards
    Card1,
    Card3,
    CardFan2,
    CardSlide1,
    CardSlide2,
    // Chips / score
    Chips1,
    Chips2,
    Multhit1,
    Multhit2,
    // Coins
    Coin1,
    Coin2,
    Coin3,
    Coin4,
    Coin5,
    Coin6,
    Coin7,
    // Edition
    Foil1,
    Foil2,
    Holo1,
    Polychrome1,
    Negative,
    // Glass
    Glass1,
    Glass2,
    Glass3,
    Glass4,
    Glass5,
    Glass6,
    // UI
    Button,
    Cancel,
    Highlight1,
    Highlight2,
    // Paper / crumple
    Crumple1,
    Crumple2,
    Crumple3,
    Crumple4,
    Crumple5,
    CrumpleLong1,
    CrumpleLong2,
    Paper1,
    // Magic
    MagicCrumple,
    MagicCrumple2,
    MagicCrumple3,
    // Explosion
    Explosion1,
    ExplosionBuildup1,
    ExplosionRelease1,
    // Special
    GoldSeal,
    Gong,
    Slice1,
    Tarot1,
    Tarot2,
    Timpani,
    Whoosh,
    Whoosh1,
    Whoosh2,
    WhooshLong,
    Win,
    SplashBuildup,
    Generic1,
    Other1,
    // Ambient
    AmbientFire1,
    AmbientFire2,
    AmbientFire3,
    AmbientOrgan1,
    // Voice / intro
    Voice1,
    Voice2,
    Voice3,
    Voice4,
    Voice5,
    Voice6,
    Voice7,
    Voice8,
    Voice9,
    Voice10,
    Voice11,
    IntroPad1,
}

/// Every variant in declaration order – handy for iteration.
pub const ALL_SOUNDS: &[SoundEvent] = &[
    SoundEvent::Music1,
    SoundEvent::Music2,
    SoundEvent::Music3,
    SoundEvent::Music4,
    SoundEvent::Music5,
    SoundEvent::Card1,
    SoundEvent::Card3,
    SoundEvent::CardFan2,
    SoundEvent::CardSlide1,
    SoundEvent::CardSlide2,
    SoundEvent::Chips1,
    SoundEvent::Chips2,
    SoundEvent::Multhit1,
    SoundEvent::Multhit2,
    SoundEvent::Coin1,
    SoundEvent::Coin2,
    SoundEvent::Coin3,
    SoundEvent::Coin4,
    SoundEvent::Coin5,
    SoundEvent::Coin6,
    SoundEvent::Coin7,
    SoundEvent::Foil1,
    SoundEvent::Foil2,
    SoundEvent::Holo1,
    SoundEvent::Polychrome1,
    SoundEvent::Negative,
    SoundEvent::Glass1,
    SoundEvent::Glass2,
    SoundEvent::Glass3,
    SoundEvent::Glass4,
    SoundEvent::Glass5,
    SoundEvent::Glass6,
    SoundEvent::Button,
    SoundEvent::Cancel,
    SoundEvent::Highlight1,
    SoundEvent::Highlight2,
    SoundEvent::Crumple1,
    SoundEvent::Crumple2,
    SoundEvent::Crumple3,
    SoundEvent::Crumple4,
    SoundEvent::Crumple5,
    SoundEvent::CrumpleLong1,
    SoundEvent::CrumpleLong2,
    SoundEvent::Paper1,
    SoundEvent::MagicCrumple,
    SoundEvent::MagicCrumple2,
    SoundEvent::MagicCrumple3,
    SoundEvent::Explosion1,
    SoundEvent::ExplosionBuildup1,
    SoundEvent::ExplosionRelease1,
    SoundEvent::GoldSeal,
    SoundEvent::Gong,
    SoundEvent::Slice1,
    SoundEvent::Tarot1,
    SoundEvent::Tarot2,
    SoundEvent::Timpani,
    SoundEvent::Whoosh,
    SoundEvent::Whoosh1,
    SoundEvent::Whoosh2,
    SoundEvent::WhooshLong,
    SoundEvent::Win,
    SoundEvent::SplashBuildup,
    SoundEvent::Generic1,
    SoundEvent::Other1,
    SoundEvent::AmbientFire1,
    SoundEvent::AmbientFire2,
    SoundEvent::AmbientFire3,
    SoundEvent::AmbientOrgan1,
    SoundEvent::Voice1,
    SoundEvent::Voice2,
    SoundEvent::Voice3,
    SoundEvent::Voice4,
    SoundEvent::Voice5,
    SoundEvent::Voice6,
    SoundEvent::Voice7,
    SoundEvent::Voice8,
    SoundEvent::Voice9,
    SoundEvent::Voice10,
    SoundEvent::Voice11,
    SoundEvent::IntroPad1,
];

impl SoundEvent {
    /// Asset path relative to the Bevy `AssetPlugin::file_path` root.
    pub fn asset_path(self) -> &'static str {
        match self {
            Self::Music1 => "sounds/music1.ogg",
            Self::Music2 => "sounds/music2.ogg",
            Self::Music3 => "sounds/music3.ogg",
            Self::Music4 => "sounds/music4.ogg",
            Self::Music5 => "sounds/music5.ogg",
            Self::Card1 => "sounds/card1.ogg",
            Self::Card3 => "sounds/card3.ogg",
            Self::CardFan2 => "sounds/cardFan2.ogg",
            Self::CardSlide1 => "sounds/cardSlide1.ogg",
            Self::CardSlide2 => "sounds/cardSlide2.ogg",
            Self::Chips1 => "sounds/chips1.ogg",
            Self::Chips2 => "sounds/chips2.ogg",
            Self::Multhit1 => "sounds/multhit1.ogg",
            Self::Multhit2 => "sounds/multhit2.ogg",
            Self::Coin1 => "sounds/coin1.ogg",
            Self::Coin2 => "sounds/coin2.ogg",
            Self::Coin3 => "sounds/coin3.ogg",
            Self::Coin4 => "sounds/coin4.ogg",
            Self::Coin5 => "sounds/coin5.ogg",
            Self::Coin6 => "sounds/coin6.ogg",
            Self::Coin7 => "sounds/coin7.ogg",
            Self::Foil1 => "sounds/foil1.ogg",
            Self::Foil2 => "sounds/foil2.ogg",
            Self::Holo1 => "sounds/holo1.ogg",
            Self::Polychrome1 => "sounds/polychrome1.ogg",
            Self::Negative => "sounds/negative.ogg",
            Self::Glass1 => "sounds/glass1.ogg",
            Self::Glass2 => "sounds/glass2.ogg",
            Self::Glass3 => "sounds/glass3.ogg",
            Self::Glass4 => "sounds/glass4.ogg",
            Self::Glass5 => "sounds/glass5.ogg",
            Self::Glass6 => "sounds/glass6.ogg",
            Self::Button => "sounds/button.ogg",
            Self::Cancel => "sounds/cancel.ogg",
            Self::Highlight1 => "sounds/highlight1.ogg",
            Self::Highlight2 => "sounds/highlight2.ogg",
            Self::Crumple1 => "sounds/crumple1.ogg",
            Self::Crumple2 => "sounds/crumple2.ogg",
            Self::Crumple3 => "sounds/crumple3.ogg",
            Self::Crumple4 => "sounds/crumple4.ogg",
            Self::Crumple5 => "sounds/crumple5.ogg",
            Self::CrumpleLong1 => "sounds/crumpleLong1.ogg",
            Self::CrumpleLong2 => "sounds/crumpleLong2.ogg",
            Self::Paper1 => "sounds/paper1.ogg",
            Self::MagicCrumple => "sounds/magic_crumple.ogg",
            Self::MagicCrumple2 => "sounds/magic_crumple2.ogg",
            Self::MagicCrumple3 => "sounds/magic_crumple3.ogg",
            Self::Explosion1 => "sounds/explosion1.ogg",
            Self::ExplosionBuildup1 => "sounds/explosion_buildup1.ogg",
            Self::ExplosionRelease1 => "sounds/explosion_release1.ogg",
            Self::GoldSeal => "sounds/gold_seal.ogg",
            Self::Gong => "sounds/gong.ogg",
            Self::Slice1 => "sounds/slice1.ogg",
            Self::Tarot1 => "sounds/tarot1.ogg",
            Self::Tarot2 => "sounds/tarot2.ogg",
            Self::Timpani => "sounds/timpani.ogg",
            Self::Whoosh => "sounds/whoosh.ogg",
            Self::Whoosh1 => "sounds/whoosh1.ogg",
            Self::Whoosh2 => "sounds/whoosh2.ogg",
            Self::WhooshLong => "sounds/whoosh_long.ogg",
            Self::Win => "sounds/win.ogg",
            Self::SplashBuildup => "sounds/splash_buildup.ogg",
            Self::Generic1 => "sounds/generic1.ogg",
            Self::Other1 => "sounds/other1.ogg",
            Self::AmbientFire1 => "sounds/ambientFire1.ogg",
            Self::AmbientFire2 => "sounds/ambientFire2.ogg",
            Self::AmbientFire3 => "sounds/ambientFire3.ogg",
            Self::AmbientOrgan1 => "sounds/ambientOrgan1.ogg",
            Self::Voice1 => "sounds/voice1.ogg",
            Self::Voice2 => "sounds/voice2.ogg",
            Self::Voice3 => "sounds/voice3.ogg",
            Self::Voice4 => "sounds/voice4.ogg",
            Self::Voice5 => "sounds/voice5.ogg",
            Self::Voice6 => "sounds/voice6.ogg",
            Self::Voice7 => "sounds/voice7.ogg",
            Self::Voice8 => "sounds/voice8.ogg",
            Self::Voice9 => "sounds/voice9.ogg",
            Self::Voice10 => "sounds/voice10.ogg",
            Self::Voice11 => "sounds/voice11.ogg",
            Self::IntroPad1 => "sounds/introPad1.ogg",
        }
    }

    /// True for music tracks that should loop and crossfade.
    pub fn is_music(self) -> bool {
        matches!(
            self,
            Self::Music1 | Self::Music2 | Self::Music3 | Self::Music4 | Self::Music5
        )
    }

    /// True for ambient sounds (looping, managed separately).
    pub fn is_ambient(self) -> bool {
        matches!(
            self,
            Self::AmbientFire1 | Self::AmbientFire2 | Self::AmbientFire3 | Self::AmbientOrgan1
        )
    }
}

// ---------------------------------------------------------------------------
// Bevy event – triggered via `commands.trigger(PlaySound::new(..))`.
// Observers registered by the plugin handle playback.
// ---------------------------------------------------------------------------

#[derive(Event, Debug, Clone)]
pub struct PlaySound {
    pub sound: SoundEvent,
    /// Per-instance volume multiplier (default 1.0).
    pub volume: f32,
    /// Playback speed / pitch multiplier (default 1.0).
    pub pitch: f32,
}

impl PlaySound {
    pub fn new(sound: SoundEvent) -> Self {
        Self {
            sound,
            volume: 1.0,
            pitch: 1.0,
        }
    }

    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume;
        self
    }

    pub fn with_pitch(mut self, pitch: f32) -> Self {
        self.pitch = pitch;
        self
    }
}

// ---------------------------------------------------------------------------
// Resources
// ---------------------------------------------------------------------------

/// Pre-loaded handles for every sound file.
#[derive(Resource)]
pub struct SoundAssets {
    pub handles: HashMap<SoundEvent, Handle<AudioSource>>,
}

/// Mirrors the original game's `SETTINGS.SOUND` table.
#[derive(Resource)]
pub struct SoundSettings {
    /// Master volume 0–100 (maps to `volume` in the Lua code).
    pub master_volume: f32,
    /// Music volume 0–100.
    pub music_volume: f32,
    /// Game / SFX volume 0–100.
    pub game_sounds_volume: f32,
}

impl Default for SoundSettings {
    fn default() -> Self {
        Self {
            master_volume: 100.0,
            music_volume: 100.0,
            game_sounds_volume: 100.0,
        }
    }
}

/// State that drives music cross-fading (mirrors the Lua `SET_SFX` logic for
/// music tracks).
#[derive(Resource)]
pub struct MusicState {
    /// The track we want to hear at full volume.
    pub desired_track: Option<SoundEvent>,
    /// Entity + current fade-level for every spawned music track.
    pub tracks: HashMap<SoundEvent, MusicTrackInfo>,
    /// Pitch modifier applied to all music (Lua `pitch_mod`).
    pub pitch_mod: f32,
}

impl Default for MusicState {
    fn default() -> Self {
        Self {
            desired_track: None,
            tracks: HashMap::new(),
            pitch_mod: 1.0,
        }
    }
}

pub struct MusicTrackInfo {
    pub entity: Entity,
    /// Current crossfade volume in 0.0–1.0 (lerped each frame).
    pub current_volume: f32,
    /// The "original volume" the track was started with (Lua `original_volume`).
    pub original_volume: f32,
    /// The "original pitch" the track was started with (Lua `original_pitch`).
    pub original_pitch: f32,
}

/// Marker component so we can query music entities.
#[derive(Component)]
pub struct MusicTrack {
    pub sound: SoundEvent,
}

/// State for per-ambient-sound volume control (Lua `ambient_control`).
#[derive(Resource, Default)]
pub struct AmbientControl {
    pub layers: HashMap<SoundEvent, AmbientLayer>,
}

pub struct AmbientLayer {
    pub volume: f32,
    pub pitch: f32,
    pub entity: Option<Entity>,
}

// Defaults matching the Lua `RESTART_MUSIC` call: per=0.7, vol=0.6
const MUSIC_DEFAULT_PITCH: f32 = 0.7;
const MUSIC_DEFAULT_VOLUME: f32 = 0.6;

// ---------------------------------------------------------------------------
// Plugin
// ---------------------------------------------------------------------------

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SoundSettings>()
            .init_resource::<MusicState>()
            .init_resource::<AmbientControl>()
            .add_systems(Startup, load_sound_assets)
            .add_systems(OnEnter(AppState::MainMenu), start_menu_music)
            .add_observer(on_play_sound)
            .add_systems(
                Update,
                (update_music_crossfade, update_ambient_sounds),
            );
    }
}

// ---------------------------------------------------------------------------
// Systems
// ---------------------------------------------------------------------------

/// Pre-load every sound file into the asset server at startup.
fn load_sound_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut handles = HashMap::new();
    for &sound in ALL_SOUNDS {
        let handle: Handle<AudioSource> = asset_server.load(sound.asset_path());
        handles.insert(sound, handle);
    }
    commands.insert_resource(SoundAssets { handles });
}

/// Kick off the main-menu music (matches the original game's default).
fn start_menu_music(mut music_state: ResMut<MusicState>) {
    music_state.desired_track = Some(SoundEvent::Music1);
}

/// Observer that fires for every [`PlaySound`] trigger.  Music events update
/// [`MusicState`]; everything else spawns a one-shot audio entity.
fn on_play_sound(
    ev: On<PlaySound>,
    mut commands: Commands,
    assets: Option<Res<SoundAssets>>,
    settings: Res<SoundSettings>,
    mut music_state: ResMut<MusicState>,
) {
    let Some(assets) = assets else { return };

    // Music: just change the desired track – crossfade handles the rest.
    if ev.sound.is_music() {
        music_state.desired_track = Some(ev.sound);
        return;
    }

    // Ambient sounds are managed by update_ambient_sounds.
    if ev.sound.is_ambient() {
        return;
    }

    let Some(handle) = assets.handles.get(&ev.sound) else {
        return;
    };

    let vol = ev.volume
        * (settings.master_volume / 100.0)
        * (settings.game_sounds_volume / 100.0);

    if vol <= 0.0 {
        return;
    }

    commands.spawn((
        AudioPlayer::new(handle.clone()),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Despawn,
            volume: bevy::audio::Volume::Linear(vol),
            speed: ev.pitch,
            ..default()
        },
    ));
}

/// Every frame, lerp each music track's volume toward its target (1.0 for the
/// desired track, 0.0 for all others) and spawn missing tracks.
///
/// Matches the Lua: `current = target*(dt*3) + (1-dt*3)*current`
fn update_music_crossfade(
    mut commands: Commands,
    time: Res<Time>,
    assets: Option<Res<SoundAssets>>,
    settings: Res<SoundSettings>,
    mut music_state: ResMut<MusicState>,
    mut sinks: Query<&mut AudioSink>,
) {
    let Some(assets) = assets else { return };
    let Some(desired) = music_state.desired_track else {
        return;
    };

    let dt = time.delta_secs();
    // Crossfade rate matching the original: `dt * 3`
    let alpha = (dt * 3.0).clamp(0.0, 1.0);

    let master = settings.master_volume / 100.0;
    let music_vol = settings.music_volume / 100.0;

    // Ensure the desired track entity exists.
    if !music_state.tracks.contains_key(&desired) {
        if let Some(handle) = assets.handles.get(&desired) {
            let entity = commands
                .spawn((
                    AudioPlayer::new(handle.clone()),
                    PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Loop,
                        volume: bevy::audio::Volume::Linear(0.0),
                        speed: MUSIC_DEFAULT_PITCH * music_state.pitch_mod,
                        ..default()
                    },
                    MusicTrack { sound: desired },
                ))
                .id();

            music_state.tracks.insert(
                desired,
                MusicTrackInfo {
                    entity,
                    current_volume: 0.0,
                    original_volume: MUSIC_DEFAULT_VOLUME,
                    original_pitch: MUSIC_DEFAULT_PITCH,
                },
            );
        }
    }

    // Crossfade all active tracks.
    let mut despawn_list: Vec<SoundEvent> = Vec::new();
    let pitch_mod = music_state.pitch_mod;

    for (&sound, info) in music_state.tracks.iter_mut() {
        let target = if sound == desired { 1.0 } else { 0.0 };
        info.current_volume = target * alpha + info.current_volume * (1.0 - alpha);

        let effective = info.current_volume * info.original_volume * master * music_vol;

        if let Ok(mut sink) = sinks.get_mut(info.entity) {
            sink.set_volume(bevy::audio::Volume::Linear(effective));
            sink.set_speed(info.original_pitch * pitch_mod);
        }

        // If a non-desired track has faded out far enough, despawn it.
        if sound != desired && info.current_volume < 0.001 {
            despawn_list.push(sound);
        }
    }

    for sound in despawn_list {
        if let Some(info) = music_state.tracks.remove(&sound) {
            if let Ok(sink) = sinks.get(info.entity) {
                sink.stop();
            }
            commands.entity(info.entity).despawn();
        }
    }
}

/// Manage ambient sound layers: start / adjust volume per frame.
fn update_ambient_sounds(
    mut commands: Commands,
    assets: Option<Res<SoundAssets>>,
    settings: Res<SoundSettings>,
    mut ambient: ResMut<AmbientControl>,
    mut sinks: Query<&mut AudioSink>,
) {
    let Some(assets) = assets else { return };

    let master = settings.master_volume / 100.0;
    let sfx_vol = settings.game_sounds_volume / 100.0;

    for (&sound, layer) in ambient.layers.iter_mut() {
        let effective = layer.volume * master * sfx_vol;

        if let Some(entity) = layer.entity {
            if let Ok(mut sink) = sinks.get_mut(entity) {
                if effective <= 0.0 {
                    sink.stop();
                    commands.entity(entity).despawn();
                    layer.entity = None;
                } else {
                    sink.set_volume(bevy::audio::Volume::Linear(effective));
                }
            } else {
                // Entity gone (despawned externally).
                layer.entity = None;
            }
        } else if effective > 0.0 {
            if let Some(handle) = assets.handles.get(&sound) {
                let entity = commands
                    .spawn((
                        AudioPlayer::new(handle.clone()),
                        PlaybackSettings {
                            mode: bevy::audio::PlaybackMode::Loop,
                            volume: bevy::audio::Volume::Linear(effective),
                            speed: layer.pitch,
                            ..default()
                        },
                    ))
                    .id();
                layer.entity = Some(entity);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Public helpers
// ---------------------------------------------------------------------------

/// Convenience: stop all music and ambient by clearing state.
pub fn stop_all_audio(
    mut commands: Commands,
    mut music_state: ResMut<MusicState>,
    mut ambient: ResMut<AmbientControl>,
    sinks: Query<&AudioSink>,
) {
    for (_, info) in music_state.tracks.drain() {
        if let Ok(sink) = sinks.get(info.entity) {
            sink.stop();
        }
        commands.entity(info.entity).despawn();
    }
    music_state.desired_track = None;

    for (_, layer) in ambient.layers.iter_mut() {
        if let Some(entity) = layer.entity.take() {
            if let Ok(sink) = sinks.get(entity) {
                sink.stop();
            }
            commands.entity(entity).despawn();
        }
    }
}

/// Convenience: restart all music at reduced pitch/volume (matches
/// `RESTART_MUSIC` in the Lua code: pitch 0.7, volume 0.6).
pub fn restart_music(music_state: &mut MusicState) {
    music_state.pitch_mod = 1.0;
    for info in music_state.tracks.values_mut() {
        info.original_pitch = MUSIC_DEFAULT_PITCH;
        info.original_volume = MUSIC_DEFAULT_VOLUME;
        info.current_volume = 0.0;
    }
}
