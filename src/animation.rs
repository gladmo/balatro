#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;

// ─── Card Hover Animation ────────────────────────────────────────────────────

/// Applied to a card button entity to make it rise on hover.
/// The card moves up (negative top offset) while hovered, mimicking the
/// original Balatro hover animation.
#[derive(Component)]
pub struct CardHoverAnim {
    /// Current upward offset in pixels (0.0 = neutral, -20.0 = lifted).
    pub offset: f32,
    /// Target offset to animate toward.
    pub target: f32,
    /// Animation speed (px/sec).
    pub speed: f32,
}

impl CardHoverAnim {
    pub fn new() -> Self {
        CardHoverAnim { offset: 0.0, target: 0.0, speed: 300.0 }
    }
}

/// Drives the smooth lift/lower animation for card hover.
pub fn animate_card_hover(
    time: Res<Time>,
    mut query: Query<(&mut Node, &mut CardHoverAnim, &Interaction)>,
) {
    for (mut node, mut anim, interaction) in &mut query {
        // Set target based on interaction state
        anim.target = match interaction {
            Interaction::Hovered | Interaction::Pressed => -18.0,
            _ => 0.0,
        };

        // Smooth interpolation toward target
        let delta = anim.target - anim.offset;
        let step = anim.speed * time.delta_secs();
        if delta.abs() < step {
            anim.offset = anim.target;
        } else {
            anim.offset += step * delta.signum();
        }

        node.top = Val::Px(anim.offset);
    }
}

// ─── Card Selection Lift Animation ──────────────────────────────────────────

/// Drives the "lifted" position for selected cards (offset applied in addition
/// to the hover offset, so selected+hovered cards are even higher).
#[derive(Component)]
pub struct CardSelectAnim {
    pub selected_offset: f32,
    pub current: f32,
    pub speed: f32,
}

impl CardSelectAnim {
    pub fn new() -> Self {
        CardSelectAnim { selected_offset: 0.0, current: 0.0, speed: 280.0 }
    }
}

pub fn animate_card_select(
    time: Res<Time>,
    mut query: Query<(&mut Node, &mut CardSelectAnim, &mut BorderColor)>,
) {
    for (mut node, mut anim, mut border) in &mut query {
        let step = anim.speed * time.delta_secs();
        let delta = anim.selected_offset - anim.current;
        if delta.abs() < step {
            anim.current = anim.selected_offset;
        } else {
            anim.current += step * delta.signum();
        }
        // `CardHoverAnim` already drives `Node.top` for the hover-lift.
        // To avoid conflicting writes to the same field, selection lift is
        // applied via `margin.bottom` with a negative value, which pushes
        // the card visually upward within the flex container without
        // interfering with the `top` property used by hover.
        node.margin.bottom = Val::Px(-anim.current);
    }
}

// ─── Button Flash Effect ─────────────────────────────────────────────────────

/// Added to an action button (Play/Discard) to flash it briefly when pressed.
#[derive(Component)]
pub struct ButtonFlash {
    pub timer: Timer,
    pub flash_color: Color,
    pub base_color: Color,
    pub active: bool,
}

impl ButtonFlash {
    pub fn new(flash_color: Color, base_color: Color) -> Self {
        ButtonFlash {
            timer: Timer::from_seconds(0.25, TimerMode::Once),
            flash_color,
            base_color,
            active: false,
        }
    }
}

pub fn animate_button_flash(
    time: Res<Time>,
    mut query: Query<(&mut BackgroundColor, &mut ButtonFlash)>,
) {
    for (mut bg, mut flash) in &mut query {
        if !flash.active {
            continue;
        }
        flash.timer.tick(time.delta());
        if flash.timer.just_finished() {
            bg.0 = flash.base_color;
            flash.active = false;
        } else {
            // Lerp from flash_color back toward base_color
            let t = flash.timer.fraction();
            let fc = flash.flash_color.to_linear();
            let bc = flash.base_color.to_linear();
            let lerped = LinearRgba::new(
                fc.red   + (bc.red   - fc.red)   * t,
                fc.green + (bc.green - fc.green) * t,
                fc.blue  + (bc.blue  - fc.blue)  * t,
                fc.alpha + (bc.alpha - fc.alpha) * t,
            );
            bg.0 = Color::from(lerped);
        }
    }
}

/// Trigger a flash on a button by resetting its timer.
pub fn trigger_flash(flash: &mut ButtonFlash, bg: &mut BackgroundColor) {
    flash.timer.reset();
    flash.active = true;
    bg.0 = flash.flash_color;
}

// ─── Plugin ──────────────────────────────────────────────────────────────────

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            animate_card_hover,
            animate_card_select,
            animate_button_flash,
        ));
    }
}
