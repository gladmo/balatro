use bevy::prelude::*;

pub mod blind_select;
pub mod game_ui;
pub mod help_screen;
pub mod main_menu;
pub mod shop_ui;

pub struct BalatroUiPlugin;

impl Plugin for BalatroUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FontAssets>()
            .add_systems(Startup, load_fonts)
            .add_plugins(main_menu::MainMenuPlugin)
            .add_plugins(game_ui::GameUiPlugin)
            .add_plugins(shop_ui::ShopUiPlugin)
            .add_plugins(blind_select::BlindSelectPlugin)
            .add_plugins(help_screen::HelpScreenPlugin);
    }
}

/// Game color palette matching Balatro's aesthetic
pub struct GameColors;

impl GameColors {
    pub fn bg_dark() -> Color {
        Color::srgb(0.07, 0.09, 0.13)
    }
    pub fn bg_medium() -> Color {
        Color::srgb(0.12, 0.15, 0.22)
    }
    pub fn panel() -> Color {
        Color::srgb(0.15, 0.18, 0.27)
    }
    pub fn accent_red() -> Color {
        Color::srgb(0.9, 0.2, 0.25)
    }
    pub fn accent_blue() -> Color {
        Color::srgb(0.2, 0.4, 0.85)
    }
    pub fn accent_green() -> Color {
        Color::srgb(0.15, 0.7, 0.35)
    }
    pub fn accent_gold() -> Color {
        Color::srgb(0.95, 0.75, 0.2)
    }
    pub fn accent_purple() -> Color {
        Color::srgb(0.6, 0.3, 0.8)
    }
    pub fn text_white() -> Color {
        Color::srgb(0.95, 0.95, 0.95)
    }
    pub fn text_dim() -> Color {
        Color::srgb(0.6, 0.6, 0.7)
    }
    pub fn card_bg() -> Color {
        Color::srgb(0.95, 0.93, 0.88)
    }
    pub fn button_normal() -> Color {
        Color::srgb(0.25, 0.3, 0.42)
    }
    pub fn button_hover() -> Color {
        Color::srgb(0.35, 0.4, 0.55)
    }
    pub fn button_pressed() -> Color {
        Color::srgb(0.18, 0.22, 0.32)
    }
}

#[derive(Resource, Default)]
pub struct FontAssets {
    pub english: Handle<Font>,
    pub chinese: Handle<Font>,
}

fn load_fonts(mut fonts: ResMut<FontAssets>, asset_server: Res<AssetServer>) {
    fonts.english = asset_server.load("fonts/m6x11plus.ttf");
    fonts.chinese = asset_server.load("fonts/GoNotoCJKCore.ttf");
}

/// Component to mark cleanup for state changes
#[derive(Component)]
pub struct CleanupOnStateChange;
