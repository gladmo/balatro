use bevy::prelude::*;

use crate::game_state::AppState;

pub struct TexturePlugin;

impl Plugin for TexturePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameTextures>()
            .add_systems(OnEnter(AppState::Loading), load_textures)
            .add_systems(
                Update,
                check_textures_loaded.run_if(in_state(AppState::Loading)),
            );
    }
}

#[derive(Resource, Default)]
pub struct GameTextures {
    pub jokers: Handle<Image>,
    pub tarots: Handle<Image>,
    pub vouchers: Handle<Image>,
    pub boosters: Handle<Image>,
    pub blind_chips: Handle<Image>,
    pub ui_assets: Handle<Image>,
    pub balatro_logo: Handle<Image>,
    pub loaded: bool,
}

fn load_textures(mut textures: ResMut<GameTextures>, asset_server: Res<AssetServer>) {
    textures.jokers = asset_server.load("textures/1x/Jokers.png");
    textures.tarots = asset_server.load("textures/1x/Tarots.png");
    textures.vouchers = asset_server.load("textures/1x/Vouchers.png");
    textures.boosters = asset_server.load("textures/1x/boosters.png");
    textures.blind_chips = asset_server.load("textures/1x/BlindChips.png");
    textures.ui_assets = asset_server.load("textures/1x/ui_assets.png");
    textures.balatro_logo = asset_server.load("textures/1x/balatro.png");
}

fn check_textures_loaded(
    mut next_state: ResMut<NextState<AppState>>,
    mut textures: ResMut<GameTextures>,
    asset_server: Res<AssetServer>,
) {
    use bevy::asset::RecursiveDependencyLoadState;
    let id1 = textures.jokers.id().untyped();
    let id2 = textures.balatro_logo.id().untyped();
    let handles = [id1, id2];

    let all_loaded = handles.iter().all(|id| {
        matches!(
            asset_server.get_recursive_dependency_load_state(*id),
            Some(RecursiveDependencyLoadState::Loaded)
        )
    });

    if all_loaded {
        textures.loaded = true;
        next_state.set(AppState::MainMenu);
    }
}
