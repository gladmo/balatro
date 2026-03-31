#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;
use crate::cards::Edition;

// ─── Color-based Holographic / Foil Shimmer ──────────────────────────────────
// These systems drive animated color tints on ImageNode components for cards
// with Holographic, Polychrome, or Foil editions, replicating the visual intent
// of the original holo.fs / foil.fs shaders using Bevy's built-in color system.

/// Marks a UI image node as having a holographic edition effect.
/// The system will animate its `ImageNode.color` each frame.
#[derive(Component)]
pub struct HoloEffect {
    pub phase: f32,
}

/// Marks a UI image node as having a foil edition effect.
#[derive(Component)]
pub struct FoilEffect {
    pub phase: f32,
}

/// Drives the holographic rainbow shimmer by cycling ImageNode hue each frame.
pub fn animate_holo_effect(
    time: Res<Time>,
    mut query: Query<(&mut ImageNode, &mut HoloEffect)>,
) {
    let t = time.elapsed_secs();
    for (mut image_node, mut fx) in &mut query {
        fx.phase = t * 0.4;
        // Cycle through hue: produce a rainbow shimmer with slight brightness boost
        let hue = (fx.phase * 360.0) % 360.0;
        let hsl = Hsla::new(hue, 0.6, 0.75, 0.85);
        image_node.color = Color::from(hsl);
    }
}

/// Drives the foil effect: blue-shifted specular flash oscillation.
pub fn animate_foil_effect(
    time: Res<Time>,
    mut query: Query<(&mut ImageNode, &mut FoilEffect)>,
) {
    let t = time.elapsed_secs();
    for (mut image_node, mut fx) in &mut query {
        fx.phase = t * 0.6;
        // Oscillate between normal and a cool blue-silver tint
        let intensity = (fx.phase.sin() * 0.5 + 0.5) * 0.7 + 0.3;
        image_node.color = Color::srgba(
            intensity * 0.7,
            intensity * 0.85,
            intensity * 1.0 + 0.15,
            1.0,
        );
    }
}

/// On entering the Playing state, scan the hand for cards with editions and
/// attach the appropriate shader effect component.
pub fn attach_edition_effects(
    mut commands: Commands,
    card_query: Query<(Entity, &crate::ui::game_ui::HandCardButton)>,
    hand: Res<crate::deck::Hand>,
) {
    for (entity, btn) in &card_query {
        let Some(card) = hand.cards.get(btn.index) else { continue };
        match card.edition {
            Edition::Holographic | Edition::Polychrome => {
                commands.entity(entity).insert(HoloEffect { phase: 0.0 });
            }
            Edition::Foil => {
                commands.entity(entity).insert(FoilEffect { phase: 0.0 });
            }
            _ => {}
        }
    }
}

pub struct CardShaderPlugin;

impl Plugin for CardShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (animate_holo_effect, animate_foil_effect));
        // Re-attach edition effects whenever the Playing state is entered
        app.add_systems(
            OnEnter(crate::GameState::Playing),
            attach_edition_effects.after(crate::ui::game_ui::setup_game_ui),
        );
    }
}
