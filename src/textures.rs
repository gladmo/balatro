#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;
use bevy::image::TextureAtlasLayout;
use crate::cards::{Suit, Rank};
use crate::jokers::JokerId;
use crate::consumables::{TarotCard, PlanetCard};

/// Texture scale: 1 for standard displays, 2 for HiDPI/Retina.
/// Mirrors the original Lua project's `texture_scaling` setting (default 2).
#[derive(Resource, Clone, Copy, PartialEq, Eq, Debug)]
pub struct TextureScale(pub u32);

impl Default for TextureScale {
    fn default() -> Self {
        TextureScale(2)
    }
}

/// All loaded texture atlases, mirroring the original project's ASSET_ATLAS.
/// Scale-appropriate images from resources/textures/1x/ or resources/textures/2x/ are loaded.
#[derive(Resource)]
pub struct GameTextures {
    pub scale: u32,

    // Playing card faces  (8BitDeck.png): 13 cols × 4 rows (suits), 71×95 px/sprite @ 1x
    pub cards: Handle<Image>,
    pub cards_layout: Handle<TextureAtlasLayout>,

    // Joker cards (Jokers.png): 10 cols × 16 rows, 71×95 px/sprite @ 1x
    pub jokers: Handle<Image>,
    pub jokers_layout: Handle<TextureAtlasLayout>,

    // Tarot + Planet cards (Tarots.png): 10 cols × 6 rows, 71×95 px/sprite @ 1x
    pub tarots: Handle<Image>,
    pub tarots_layout: Handle<TextureAtlasLayout>,

    // Card enhancements / backs (Enhancers.png): 7 cols × 5 rows, 71×95 px/sprite @ 1x
    pub enhancers: Handle<Image>,
    pub enhancers_layout: Handle<TextureAtlasLayout>,

    // UI icons (ui_assets.png): 4 cols × 2 rows, 18×18 px/sprite @ 1x
    pub ui_assets: Handle<Image>,
    pub ui_assets_layout: Handle<TextureAtlasLayout>,

    // Scoring chips (chips.png): 5 cols × 2 rows, 29×29 px/sprite @ 1x
    pub chips: Handle<Image>,
    pub chips_layout: Handle<TextureAtlasLayout>,

    // Blind chip animation (BlindChips.png): 21 cols × 31 rows, 34×34 px/sprite @ 1x
    pub blind_chips: Handle<Image>,
    pub blind_chips_layout: Handle<TextureAtlasLayout>,

    // Balatro logo (balatro.png): single image
    pub balatro_logo: Handle<Image>,
}

impl GameTextures {
    /// Atlas sprite index for a playing card in `8BitDeck.png`.
    /// Layout (cols=13, rows=4): row=suit, col=rank (2..A).
    pub fn card_sprite_index(suit: Suit, rank: Rank) -> usize {
        let row: usize = match suit {
            Suit::Hearts   => 0,
            Suit::Clubs    => 1,
            Suit::Diamonds => 2,
            Suit::Spades   => 3,
        };
        let col: usize = match rank {
            Rank::Two   => 0,
            Rank::Three => 1,
            Rank::Four  => 2,
            Rank::Five  => 3,
            Rank::Six   => 4,
            Rank::Seven => 5,
            Rank::Eight => 6,
            Rank::Nine  => 7,
            Rank::Ten   => 8,
            Rank::Jack  => 9,
            Rank::Queen => 10,
            Rank::King  => 11,
            Rank::Ace   => 12,
        };
        row * 13 + col
    }

    /// Atlas sprite index for a joker in `Jokers.png` (10 cols × 16 rows).
    /// Positions taken directly from the original game's P_CENTERS data.
    pub fn joker_sprite_index(id: JokerId) -> usize {
        // index = row * 10 + col
        match id {
            JokerId::Joker          => 0 * 10 + 0,   // {x=0,y=0}
            JokerId::JollyJoker     => 0 * 10 + 2,   // {x=2,y=0}
            JokerId::ZanyJoker      => 0 * 10 + 3,   // {x=3,y=0}
            JokerId::MadJoker       => 0 * 10 + 4,   // {x=4,y=0}
            JokerId::CrazyJoker     => 0 * 10 + 5,   // {x=5,y=0}
            JokerId::DrollJoker     => 0 * 10 + 6,   // {x=6,y=0}
            JokerId::HalfJoker      => 0 * 10 + 7,   // {x=7,y=0}
            JokerId::StoneJoker     => 0 * 10 + 9,   // {x=9,y=0}
            JokerId::Juggler        => 1 * 10 + 0,   // {x=0,y=1}
            JokerId::Drunkard       => 1 * 10 + 1,   // {x=1,y=1}
            JokerId::GreedyJoker    => 1 * 10 + 6,   // {x=6,y=1}
            JokerId::LustyJoker     => 1 * 10 + 7,   // {x=7,y=1}
            JokerId::WrathfulJoker  => 1 * 10 + 8,   // {x=8,y=1}
            JokerId::GluttonousJoker=> 1 * 10 + 9,   // {x=9,y=1}
            JokerId::Banner         => 2 * 10 + 1,   // {x=1,y=2}
            JokerId::Misprint       => 2 * 10 + 6,   // {x=6,y=2}
            JokerId::GoldenJoker    => 2 * 10 + 9,   // {x=9,y=2}
            JokerId::ScaryFace      => 3 * 10 + 2,   // {x=2,y=3}
            JokerId::AbstractJoker  => 3 * 10 + 3,   // {x=3,y=3}
            JokerId::Scholar        => 4 * 10 + 0,   // {x=0,y=4}
            JokerId::BusinessCard   => 4 * 10 + 1,   // {x=1,y=4}
            JokerId::Supernova      => 4 * 10 + 2,   // {x=2,y=4}
            JokerId::Fibonacci      => 5 * 10 + 1,   // {x=1,y=5}
            JokerId::SlyJoker       => 14 * 10 + 0,  // {x=0,y=14}
            JokerId::BlueJoker      => 10 * 10 + 7,  // {x=7,y=10}
        }
    }

    /// Atlas sprite index for a Tarot card in `Tarots.png` (10 cols × 6 rows).
    /// Rows 0-2: Tarot cards (22 cards total); rows 2-5: Planet cards.
    pub fn tarot_sprite_index(tarot: TarotCard) -> usize {
        // index = row * 10 + col; matches original P_CENTERS positions
        match tarot {
            TarotCard::Magician         => 0 * 10 + 1,   // {x=1,y=0}
            TarotCard::HighPriestess    => 0 * 10 + 2,   // {x=2,y=0}
            TarotCard::Empress          => 0 * 10 + 3,   // {x=3,y=0}
            TarotCard::Emperor          => 0 * 10 + 4,   // {x=4,y=0}
            TarotCard::Hierophant       => 0 * 10 + 5,   // {x=5,y=0}
            TarotCard::Lovers           => 0 * 10 + 6,   // {x=6,y=0}
            TarotCard::Chariot          => 0 * 10 + 7,   // {x=7,y=0}
            TarotCard::Justice          => 0 * 10 + 8,   // {x=8,y=0}
            TarotCard::Hermit           => 0 * 10 + 9,   // {x=9,y=0}
            TarotCard::Strength         => 1 * 10 + 1,   // {x=1,y=1}
            TarotCard::HangedMan        => 1 * 10 + 2,   // {x=2,y=1}
            TarotCard::Death            => 1 * 10 + 3,   // {x=3,y=1}
            TarotCard::Devil            => 1 * 10 + 5,   // {x=5,y=1}
            TarotCard::Tower            => 1 * 10 + 6,   // {x=6,y=1}
            TarotCard::Judgement        => 2 * 10 + 0,   // {x=0,y=2}
        }
    }

    /// Atlas sprite index for a Planet card in `Tarots.png` (10 cols × 6 rows).
    pub fn planet_sprite_index(planet: PlanetCard) -> usize {
        match planet {
            PlanetCard::Mercury  => 3 * 10 + 0,   // {x=0,y=3}
            PlanetCard::Venus    => 3 * 10 + 1,   // {x=1,y=3}
            PlanetCard::Earth    => 3 * 10 + 2,   // {x=2,y=3}
            PlanetCard::Mars     => 3 * 10 + 3,   // {x=3,y=3}
            PlanetCard::Jupiter  => 3 * 10 + 4,   // {x=4,y=3}
            PlanetCard::Saturn   => 3 * 10 + 5,   // {x=5,y=3}
            PlanetCard::Uranus   => 3 * 10 + 6,   // {x=6,y=3}
            PlanetCard::Neptune  => 3 * 10 + 7,   // {x=7,y=3}
            PlanetCard::Pluto    => 3 * 10 + 8,   // {x=8,y=3}
            PlanetCard::PlanetX  => 2 * 10 + 9,   // {x=9,y=2}
            PlanetCard::Ceres    => 4 * 10 + 0,   // {x=0,y=4}
            PlanetCard::Eris     => 4 * 10 + 1,   // {x=1,y=4}
        }
    }
}

/// DPI scale threshold: displays with scale_factor >= this value use 2x textures.
/// Matches the original Lua project's two-tier texture_scaling approach.
const HIDPI_SCALE_THRESHOLD: f32 = 1.5;

/// Startup system: detect window DPI scale and load all texture atlases from
/// `resources/textures/Nx/` (where N = 1 or 2), matching the original project's
/// `set_render_settings()` approach. Runs in PostStartup so the window is ready.
pub fn load_game_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    // Choose scale: 1x for standard displays, 2x for HiDPI (scale_factor >= 1.5).
    // Default matches the original game's default of 2x.
    let scale: u32 = if let Ok(win) = windows.single() {
        if win.scale_factor() >= HIDPI_SCALE_THRESHOLD { 2 } else { 1 }
    } else {
        2
    };

    let s = scale; // sprite pixel multiplier
    let p = format!("textures/{}x/", scale); // asset-relative path prefix

    // Playing cards: 71×95 px per sprite @ 1x, 13 cols (ranks 2-A) × 4 rows (suits)
    let card_size = UVec2::new(71 * s, 95 * s);
    let cards_layout = atlas_layouts.add(
        TextureAtlasLayout::from_grid(card_size, 13, 4, None, None)
    );

    // Jokers: same sprite size, 10 cols × 16 rows
    let jokers_layout = atlas_layouts.add(
        TextureAtlasLayout::from_grid(card_size, 10, 16, None, None)
    );

    // Tarots (includes Planet cards): 10 cols × 6 rows
    let tarots_layout = atlas_layouts.add(
        TextureAtlasLayout::from_grid(card_size, 10, 6, None, None)
    );

    // Enhancers / card backs: 7 cols × 5 rows
    let enhancers_layout = atlas_layouts.add(
        TextureAtlasLayout::from_grid(card_size, 7, 5, None, None)
    );

    // UI assets: 18×18 px per sprite @ 1x, 4 cols × 2 rows
    let ui_size = UVec2::new(18 * s, 18 * s);
    let ui_assets_layout = atlas_layouts.add(
        TextureAtlasLayout::from_grid(ui_size, 4, 2, None, None)
    );

    // Scoring chips: 29×29 px per sprite @ 1x, 5 cols × 2 rows
    let chip_size = UVec2::new(29 * s, 29 * s);
    let chips_layout = atlas_layouts.add(
        TextureAtlasLayout::from_grid(chip_size, 5, 2, None, None)
    );

    // Blind chip animation: 34×34 px per sprite @ 1x, 21 cols × 31 rows
    let blind_size = UVec2::new(34 * s, 34 * s);
    let blind_chips_layout = atlas_layouts.add(
        TextureAtlasLayout::from_grid(blind_size, 21, 31, None, None)
    );

    commands.insert_resource(GameTextures {
        scale,
        cards:             asset_server.load(format!("{}8BitDeck.png", p)),
        cards_layout,
        jokers:            asset_server.load(format!("{}Jokers.png", p)),
        jokers_layout,
        tarots:            asset_server.load(format!("{}Tarots.png", p)),
        tarots_layout,
        enhancers:         asset_server.load(format!("{}Enhancers.png", p)),
        enhancers_layout,
        ui_assets:         asset_server.load(format!("{}ui_assets.png", p)),
        ui_assets_layout,
        chips:             asset_server.load(format!("{}chips.png", p)),
        chips_layout,
        blind_chips:       asset_server.load(format!("{}BlindChips.png", p)),
        blind_chips_layout,
        balatro_logo:      asset_server.load(format!("{}balatro.png", p)),
    });

    commands.insert_resource(TextureScale(scale));
}
