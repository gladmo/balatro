#![allow(dead_code)]
#![allow(unused)]

use bevy::prelude::*;

// ─── Card Hover Animation ────────────────────────────────────────────────────

/// Applied to a card button entity to make it rise on hover.
/// Uses a spring-physics model (critically-damped) for a natural feel,
/// matching the original Balatro card hover animation.
#[derive(Component)]
pub struct CardHoverAnim {
    /// Current upward offset (px). 0 = resting, negative = lifted.
    pub offset: f32,
    /// Current velocity (px/sec) — spring physics.
    pub velocity: f32,
    /// Target offset to settle at.
    pub target: f32,
    /// Spring stiffness (angular frequency squared).
    pub stiffness: f32,
    /// Damping ratio (1.0 = critically damped, no oscillation).
    pub damping: f32,
}

impl CardHoverAnim {
    pub fn new() -> Self {
        // Critically-damped spring: damping = 2 * sqrt(stiffness) gives zero overshoot
        let stiffness = 1200.0f32;
        let damping = 2.0 * stiffness.sqrt(); // critical damping coefficient
        CardHoverAnim { offset: 0.0, velocity: 0.0, target: 0.0, stiffness, damping }
    }
}

/// Spring-physics card hover lift (rises smoothly, settles without overshoot).
pub fn animate_card_hover(
    time: Res<Time>,
    mut query: Query<(&mut Node, &mut CardHoverAnim, &Interaction)>,
) {
    let dt = time.delta_secs();
    for (mut node, mut anim, interaction) in &mut query {
        anim.target = match interaction {
            Interaction::Hovered | Interaction::Pressed => -20.0,
            _ => 0.0,
        };

        // Critically-damped spring integration (semi-implicit Euler)
        let displacement = anim.offset - anim.target;
        let spring_force = -anim.stiffness * displacement;
        let damping_force = -anim.damping * anim.velocity;
        let acceleration = spring_force + damping_force;

        anim.velocity += acceleration * dt;
        anim.offset += anim.velocity * dt;

        // Snap to rest if very close
        if displacement.abs() < 0.05 && anim.velocity.abs() < 0.5 {
            anim.offset = anim.target;
            anim.velocity = 0.0;
        }

        node.top = Val::Px(anim.offset);
    }
}

// ─── Card Selection Lift + Bounce Animation ──────────────────────────────────

/// Drives the "lifted" position for selected cards using spring physics,
/// producing a satisfying bounce on selection and deselection.
/// Applied via `margin.bottom` to avoid conflicting with `Node.top` (hover).
#[derive(Component)]
pub struct CardSelectAnim {
    pub selected_offset: f32,
    pub current: f32,
    pub velocity: f32,
    pub stiffness: f32,
    pub damping: f32,
}

impl CardSelectAnim {
    pub fn new() -> Self {
        // Underdamped spring (ratio < 1.0) produces a small, satisfying bounce on card select.
        // damping_ratio = 0.65 → ~10% overshoot before settling.
        // damping = 2 * omega * damping_ratio  (standard damped oscillator formula)
        const OMEGA: f32 = 35.0;           // natural angular frequency (rad/s)
        const DAMPING_RATIO: f32 = 0.65;   // < 1.0 = underdamped (slight bounce)
        CardSelectAnim {
            selected_offset: 0.0,
            current: 0.0,
            velocity: 0.0,
            stiffness: OMEGA * OMEGA,
            damping: 2.0 * OMEGA * DAMPING_RATIO,
        }
    }
}

pub fn animate_card_select(
    time: Res<Time>,
    mut query: Query<(&mut Node, &mut CardSelectAnim)>,
) {
    let dt = time.delta_secs();
    for (mut node, mut anim) in &mut query {
        let displacement = anim.current - anim.selected_offset;
        let spring_force = -anim.stiffness * displacement;
        let damping_force = -anim.damping * anim.velocity;
        anim.velocity += (spring_force + damping_force) * dt;
        anim.current += anim.velocity * dt;

        if displacement.abs() < 0.05 && anim.velocity.abs() < 0.5 {
            anim.current = anim.selected_offset;
            anim.velocity = 0.0;
        }

        // `Node.top` is owned by CardHoverAnim; use margin.bottom for selection lift.
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
