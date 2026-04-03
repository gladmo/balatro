use std::collections::VecDeque;

use bevy::prelude::*;

// ────────────────────────────────────────────────────────────────────────────────
// Plugin
// ────────────────────────────────────────────────────────────────────────────────

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EventManager>()
            .init_resource::<MoveableSettings>()
            .add_systems(
                Update,
                (
                    update_moveable_system,
                    update_juice_system,
                    update_card_animations,
                    update_fade_animations,
                    update_flip_animations,
                    update_dissolve_animations,
                    update_materialize_animations,
                    update_event_manager,
                ),
            );
    }
}

// ────────────────────────────────────────────────────────────────────────────────
// Easing helpers (matches Lua: lerp, elastic, quad + extras)
// ────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EasingType {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Lerp,
    Elastic,
    Quad,
}

impl EasingType {
    /// Map `t` in 0..1 through the easing curve.
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            EasingType::Linear | EasingType::Lerp => t,
            EasingType::EaseIn | EasingType::Quad => t * t,
            EasingType::EaseOut => 1.0 - (1.0 - t).powi(2),
            EasingType::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            // Matches Lua: -2^(10p-10) * sin((10p-10.75)*2π/3)
            EasingType::Elastic => {
                if t <= 0.0 {
                    0.0
                } else if t >= 1.0 {
                    1.0
                } else {
                    let p = t;
                    let raw = -(2.0_f32).powf(10.0 * p - 10.0)
                        * ((p * 10.0 - 10.75) * std::f32::consts::TAU / 3.0).sin();
                    // Lua applies this as a remaining‐fraction; we invert to get 0→1 progress.
                    1.0 - (raw * (1.0 - t) + t * 0.0 - (1.0 - t) * 0.0)
                        .min(1.0)
                        .max(0.0)
                }
            }
        }
    }
}

/// Interpolate between `start` and `end_val` using the Lua ease convention.
/// `remaining` goes from 1.0 → 0.0 as time progresses; we convert to 0→1 progress.
pub fn ease_value(ease_type: EasingType, remaining: f32, start: f32, end_val: f32) -> f32 {
    match ease_type {
        EasingType::Lerp | EasingType::Linear => remaining * start + (1.0 - remaining) * end_val,
        EasingType::Elastic => {
            let p = remaining;
            let adjusted = if p <= 0.0 {
                0.0
            } else if p >= 1.0 {
                1.0
            } else {
                -(2.0_f32).powf(10.0 * p - 10.0)
                    * ((p * 10.0 - 10.75) * std::f32::consts::TAU / 3.0).sin()
            };
            adjusted * start + (1.0 - adjusted) * end_val
        }
        EasingType::Quad => {
            let adjusted = remaining * remaining;
            adjusted * start + (1.0 - adjusted) * end_val
        }
        _ => {
            let progress = 1.0 - remaining;
            let t = EasingType::apply(&ease_type, progress);
            start + (end_val - start) * t
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────────
// Moveable component  (mirrors Lua T / VT / velocity system)
// ────────────────────────────────────────────────────────────────────────────────

/// Global tuning knobs that mirror `G.exp_times` in the Lua code.
#[derive(Resource)]
pub struct MoveableSettings {
    /// Damping factor for XY velocity (Lua `G.exp_times.xy`, default ~0.9).
    pub xy_damping: f32,
    /// Multiplier for XY approach speed (Lua uses 35).
    pub xy_speed: f32,
    /// Maximum velocity magnitude.
    pub max_vel: f32,
    /// Damping factor for rotation velocity.
    pub r_damping: f32,
    /// Damping factor for scale velocity.
    pub scale_damping: f32,
    /// Speed multiplier for width/height pinch easing (Lua uses 8).
    pub wh_speed: f32,
}

impl Default for MoveableSettings {
    fn default() -> Self {
        Self {
            xy_damping: 0.9,
            xy_speed: 35.0,
            max_vel: 40.0,
            r_damping: 0.9,
            scale_damping: 0.9,
            wh_speed: 8.0,
        }
    }
}

/// Transform target + visible‐transform pair that eases over time, matching Lua `Moveable`.
///
/// Attach this component to any entity whose `Transform` should smoothly track a target.
/// Each frame the system drives `vt_*` toward `t_*` using velocity, then writes the result
/// into the entity's Bevy `Transform`.
#[derive(Component)]
pub struct Moveable {
    // ── Target transform (T) ────────────────────────────────────────────
    pub t_x: f32,
    pub t_y: f32,
    pub t_w: f32,
    pub t_h: f32,
    pub t_r: f32,
    pub t_scale: f32,

    // ── Visible transform (VT) ──────────────────────────────────────────
    pub vt_x: f32,
    pub vt_y: f32,
    pub vt_w: f32,
    pub vt_h: f32,
    pub vt_r: f32,
    pub vt_scale: f32,

    // ── Velocity ────────────────────────────────────────────────────────
    pub vel_x: f32,
    pub vel_y: f32,
    pub vel_r: f32,
    pub vel_scale: f32,

    // ── Pinch (width/height ease to 0 when true) ───────────────────────
    pub pinch_x: bool,
    pub pinch_y: bool,

    // ── Role / alignment ────────────────────────────────────────────────
    pub role: MoveableRole,
    pub alignment: Alignment,
}

impl Moveable {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            t_x: x,
            t_y: y,
            t_w: w,
            t_h: h,
            t_r: 0.0,
            t_scale: 1.0,
            vt_x: x,
            vt_y: y,
            vt_w: w,
            vt_h: h,
            vt_r: 0.0,
            vt_scale: 1.0,
            vel_x: 0.0,
            vel_y: 0.0,
            vel_r: 0.0,
            vel_scale: 0.0,
            pinch_x: false,
            pinch_y: false,
            role: MoveableRole::Major,
            alignment: Alignment::default(),
        }
    }

    /// Instantly snap VT to T and zero velocity.
    pub fn hard_set_vt(&mut self) {
        self.vt_x = self.t_x;
        self.vt_y = self.t_y;
        self.vt_w = self.t_w;
        self.vt_h = self.t_h;
        self.vt_r = self.t_r;
        self.vt_scale = self.t_scale;
        self.vel_x = 0.0;
        self.vel_y = 0.0;
        self.vel_r = 0.0;
        self.vel_scale = 0.0;
    }

    /// Set a new target and snap VT.
    pub fn hard_set_t(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.t_x = x;
        self.t_y = y;
        self.t_w = w;
        self.t_h = h;
        self.hard_set_vt();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoveableRole {
    Major,
    /// Attached to a parent entity with an offset; bonds control which axes are inherited.
    Minor {
        major: Entity,
        offset: Vec2,
        xy_bond: Bond,
        r_bond: Bond,
        scale_bond: Bond,
        wh_bond: Bond,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bond {
    /// VT directly copies the major's VT + offset (no independent easing).
    Strong,
    /// VT eases independently toward its own T.
    Weak,
}

/// Alignment descriptor (matches Lua `alignment` table).
#[derive(Debug, Clone)]
pub struct Alignment {
    pub kind: AlignmentKind,
    pub offset: Vec2,
}

impl Default for Alignment {
    fn default() -> Self {
        Self {
            kind: AlignmentKind::Absolute,
            offset: Vec2::ZERO,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignmentKind {
    /// No special alignment; position is absolute.
    Absolute,
    /// Center within parent.
    MiddleCenter,
    /// Align to top-left (inner).
    TopLeft,
    /// Align to bottom-right (inner).
    BottomRight,
    /// Align to bottom-center.
    BottomCenter,
    /// Align to top-center.
    TopCenter,
}

// ────────────────────────────────────────────────────────────────────────────────
// Moveable update system
// ────────────────────────────────────────────────────────────────────────────────

fn update_moveable_system(
    time: Res<Time>,
    settings: Res<MoveableSettings>,
    mut query: Query<(&mut Moveable, &mut Transform), Without<FlipAnimation>>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    for (mut mv, mut transform) in query.iter_mut() {
        move_xy(&mut mv, dt, &settings);
        move_r(&mut mv, dt, &settings);
        move_scale(&mut mv, dt, &settings);
        move_wh(&mut mv, dt, &settings);

        // Write VT into Bevy Transform
        transform.translation.x = mv.vt_x;
        transform.translation.y = mv.vt_y;
        transform.rotation = Quat::from_rotation_z(mv.vt_r);
        let s = mv.vt_scale.max(0.001);
        transform.scale = Vec3::new(s, s, 1.0);
    }
}

fn move_xy(mv: &mut Moveable, dt: f32, s: &MoveableSettings) {
    let dx = mv.t_x - mv.vt_x;
    let dy = mv.t_y - mv.vt_y;
    if dx.abs() < 0.01 && dy.abs() < 0.01 && mv.vel_x.abs() < 0.01 && mv.vel_y.abs() < 0.01 {
        mv.vt_x = mv.t_x;
        mv.vt_y = mv.t_y;
        mv.vel_x = 0.0;
        mv.vel_y = 0.0;
        return;
    }
    mv.vel_x = s.xy_damping * mv.vel_x + (1.0 - s.xy_damping) * dx * s.xy_speed * dt;
    mv.vel_y = s.xy_damping * mv.vel_y + (1.0 - s.xy_damping) * dy * s.xy_speed * dt;

    // Clamp velocity magnitude
    let mag_sq = mv.vel_x * mv.vel_x + mv.vel_y * mv.vel_y;
    if mag_sq > s.max_vel * s.max_vel {
        let mag = mag_sq.sqrt();
        mv.vel_x = s.max_vel * mv.vel_x / mag;
        mv.vel_y = s.max_vel * mv.vel_y / mag;
    }

    mv.vt_x += mv.vel_x;
    mv.vt_y += mv.vel_y;

    if (mv.vt_x - mv.t_x).abs() < 0.01 && mv.vel_x.abs() < 0.01 {
        mv.vt_x = mv.t_x;
        mv.vel_x = 0.0;
    }
    if (mv.vt_y - mv.t_y).abs() < 0.01 && mv.vel_y.abs() < 0.01 {
        mv.vt_y = mv.t_y;
        mv.vel_y = 0.0;
    }
}

fn move_r(mv: &mut Moveable, _dt: f32, s: &MoveableSettings) {
    let des_r = mv.t_r + 0.015 * mv.vel_x;
    let diff = des_r - mv.vt_r;
    if diff.abs() < 0.001 && mv.vel_r.abs() < 0.001 {
        mv.vt_r = mv.t_r;
        mv.vel_r = 0.0;
        return;
    }
    mv.vel_r = s.r_damping * mv.vel_r + (1.0 - s.r_damping) * diff;
    mv.vt_r += mv.vel_r;
    if (mv.vt_r - mv.t_r).abs() < 0.001 && mv.vel_r.abs() < 0.001 {
        mv.vt_r = mv.t_r;
        mv.vel_r = 0.0;
    }
}

fn move_scale(mv: &mut Moveable, _dt: f32, s: &MoveableSettings) {
    let des = mv.t_scale;
    let diff = des - mv.vt_scale;
    if diff.abs() < 0.001 && mv.vel_scale.abs() < 0.001 {
        return;
    }
    mv.vel_scale = s.scale_damping * mv.vel_scale + (1.0 - s.scale_damping) * diff;
    mv.vt_scale += mv.vel_scale;
}

fn move_wh(mv: &mut Moveable, dt: f32, s: &MoveableSettings) {
    let target_w = if mv.pinch_x { 0.0 } else { mv.t_w };
    let target_h = if mv.pinch_y { 0.0 } else { mv.t_h };
    let dir_w = if mv.pinch_x { -1.0 } else { 1.0 };
    let dir_h = if mv.pinch_y { -1.0 } else { 1.0 };

    if (mv.vt_w - target_w).abs() > 0.01 {
        mv.vt_w += s.wh_speed * dt * dir_w * mv.t_w;
        mv.vt_w = mv.vt_w.clamp(0.0, mv.t_w);
    }
    if (mv.vt_h - target_h).abs() > 0.01 {
        mv.vt_h += s.wh_speed * dt * dir_h * mv.t_h;
        mv.vt_h = mv.vt_h.clamp(0.0, mv.t_h);
    }
}

// ────────────────────────────────────────────────────────────────────────────────
// JuiceUp component  (brief scale bump + rotation wobble)
// ────────────────────────────────────────────────────────────────────────────────

/// Transient "juice" effect: the entity's scale and rotation oscillate then settle.
/// Mirrors Lua `Moveable:juice_up(amount, rot_amt)`.
#[derive(Component)]
pub struct JuiceUp {
    pub scale_amt: f32,
    pub rot_amt: f32,
    pub start_time: f32,
    pub end_time: f32,
    /// Accumulated real time (driven by `Time`).
    elapsed: f32,
    duration: f32,
}

impl JuiceUp {
    /// Create a juice effect. `scale` defaults to 0.4, `rot` defaults to ±0.6*scale.
    pub fn new(scale: Option<f32>, rot: Option<f32>) -> Self {
        let s = scale.unwrap_or(0.4);
        let r = rot.unwrap_or(0.6 * s);
        let duration = 0.4;
        Self {
            scale_amt: s,
            rot_amt: r,
            start_time: 0.0,
            end_time: duration,
            elapsed: 0.0,
            duration,
        }
    }
}

fn update_juice_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut JuiceUp, &mut Transform)>,
) {
    let dt = time.delta_secs();
    for (entity, mut juice, mut transform) in query.iter_mut() {
        juice.elapsed += dt;
        if juice.elapsed >= juice.duration {
            commands.entity(entity).remove::<JuiceUp>();
            continue;
        }
        let t = juice.elapsed;
        let remaining = ((juice.duration - juice.elapsed) / juice.duration).max(0.0);

        // Decaying sine oscillation matching Lua formula
        let scale_offset =
            juice.scale_amt * (50.8 * t).sin() * remaining.powi(3);
        let rot_offset =
            juice.rot_amt * (40.8 * t).sin() * remaining.powi(2);

        // Apply as additive offsets; the moveable system (if present) sets the base.
        transform.scale.x += scale_offset;
        transform.scale.y += scale_offset;
        transform.rotation *= Quat::from_rotation_z(rot_offset);
    }
}

// ────────────────────────────────────────────────────────────────────────────────
// Card flip animation  (pinch width → 0, swap sprite, pinch back)
// ────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlipPhase {
    /// Pinching width to 0.
    Closing,
    /// Width expanding back to full.
    Opening,
}

/// Attach to an entity to run a card‐flip animation. When `Closing` finishes the
/// `on_midpoint` flag is set so external systems can swap the sprite. The component
/// auto-removes when complete.
#[derive(Component)]
pub struct FlipAnimation {
    pub phase: FlipPhase,
    pub timer: Timer,
    pub original_scale_x: f32,
    /// Set to `true` for exactly one frame at the midpoint so consumers can swap the sprite.
    pub on_midpoint: bool,
}

impl FlipAnimation {
    /// Create a flip animation. `half_duration` is the time for each half (close + open).
    pub fn new(half_duration: f32, original_scale_x: f32) -> Self {
        Self {
            phase: FlipPhase::Closing,
            timer: Timer::from_seconds(half_duration, TimerMode::Once),
            original_scale_x,
            on_midpoint: false,
        }
    }
}

fn update_flip_animations(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut FlipAnimation)>,
) {
    for (entity, mut transform, mut flip) in query.iter_mut() {
        flip.timer.tick(time.delta());
        flip.on_midpoint = false;

        let frac = flip.timer.fraction();
        match flip.phase {
            FlipPhase::Closing => {
                transform.scale.x = flip.original_scale_x * (1.0 - frac);
                if flip.timer.just_finished() {
                    transform.scale.x = 0.0;
                    flip.phase = FlipPhase::Opening;
                    flip.timer.reset();
                    flip.on_midpoint = true;
                }
            }
            FlipPhase::Opening => {
                transform.scale.x = flip.original_scale_x * frac;
                if flip.timer.just_finished() {
                    transform.scale.x = flip.original_scale_x;
                    commands.entity(entity).remove::<FlipAnimation>();
                }
            }
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────────
// Dissolve / Materialize animations
// ────────────────────────────────────────────────────────────────────────────────

/// Dissolve effect: alpha fades from 1 → 0 over `duration`.
/// When finished, the entity is despawned (matching Lua `start_dissolve`).
#[derive(Component)]
pub struct DissolveAnimation {
    pub timer: Timer,
    pub despawn_on_complete: bool,
}

impl DissolveAnimation {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
            despawn_on_complete: true,
        }
    }
}

fn update_dissolve_animations(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DissolveAnimation, &mut Transform)>,
) {
    for (entity, mut diss, mut transform) in query.iter_mut() {
        diss.timer.tick(time.delta());
        let alpha = 1.0 - diss.timer.fraction();
        // Encode dissolve progress into scale so shaders / sprite alpha can pick it up.
        transform.scale = Vec3::splat(alpha.max(0.0));

        if diss.timer.just_finished() {
            if diss.despawn_on_complete {
                commands.entity(entity).despawn();
            } else {
                commands.entity(entity).remove::<DissolveAnimation>();
            }
        }
    }
}

/// Materialize effect: reverse dissolve (alpha 0 → 1).
#[derive(Component)]
pub struct MaterializeAnimation {
    pub timer: Timer,
}

impl MaterializeAnimation {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }
}

fn update_materialize_animations(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut MaterializeAnimation, &mut Transform), Without<DissolveAnimation>>,
) {
    for (entity, mut mat, mut transform) in query.iter_mut() {
        mat.timer.tick(time.delta());
        let alpha = mat.timer.fraction();
        transform.scale = Vec3::splat(alpha.max(0.001));

        if mat.timer.just_finished() {
            transform.scale = Vec3::ONE;
            commands.entity(entity).remove::<MaterializeAnimation>();
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────────
// Legacy CardAnimation / FadeAnimation  (kept for backward compat)
// ────────────────────────────────────────────────────────────────────────────────

/// Timer-driven positional animation with easing (original simple system).
#[derive(Component)]
pub struct CardAnimation {
    pub start_pos: Vec2,
    pub end_pos: Vec2,
    pub timer: Timer,
    pub easing: EasingType,
}

/// Timer-driven alpha fade on `BackgroundColor`.
#[derive(Component)]
pub struct FadeAnimation {
    pub start_alpha: f32,
    pub end_alpha: f32,
    pub timer: Timer,
}

fn update_card_animations(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut CardAnimation), Without<Moveable>>,
) {
    for (entity, mut transform, mut anim) in query.iter_mut() {
        anim.timer.tick(time.delta());
        let t = anim.easing.apply(anim.timer.fraction());
        let pos = anim.start_pos.lerp(anim.end_pos, t);
        transform.translation.x = pos.x;
        transform.translation.y = pos.y;

        if anim.timer.just_finished() {
            commands.entity(entity).remove::<CardAnimation>();
        }
    }
}

fn update_fade_animations(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut BackgroundColor, &mut FadeAnimation)>,
) {
    for (entity, mut bg, mut anim) in query.iter_mut() {
        anim.timer.tick(time.delta());
        let t = anim.timer.fraction();
        let alpha = anim.start_alpha + (anim.end_alpha - anim.start_alpha) * t;

        let current = bg.0;
        bg.0 = current.with_alpha(alpha);

        if anim.timer.just_finished() {
            commands.entity(entity).remove::<FadeAnimation>();
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────────
// Event system  (mirrors Lua Event / EventManager)
// ────────────────────────────────────────────────────────────────────────────────

/// What causes an event to fire.
#[derive(Debug, Clone)]
pub enum EventTrigger {
    /// Fires immediately on first process.
    Immediate,
    /// Fires after `delay` seconds have elapsed.
    After { delay: f32 },
    /// Smoothly interpolates a value over `duration` seconds.
    Ease {
        ease_type: EasingType,
        start_val: f32,
        end_val: f32,
        duration: f32,
        /// Accumulated time since the ease started.
        elapsed: f32,
    },
    /// Fires every tick until `condition_fn` returns true.
    Condition,
    /// Fires the function immediately, then waits `delay` before completing.
    Before { delay: f32 },
}

/// A single event in the queue.  
///
/// The `func` callback receives the current ease value (for `Ease` triggers) and returns
/// `true` when the event's work is done.
pub struct GameEvent {
    pub trigger: EventTrigger,
    /// If `true`, subsequent blockable events in the same queue wait for this one.
    pub blocking: bool,
    /// If `true`, this event is blocked by prior blocking events.
    pub blockable: bool,
    /// Protected from `clear_queue` calls.
    pub no_delete: bool,
    /// Callback. Receives optional ease value; returns `true` when logically complete.
    pub func: Box<dyn FnMut(Option<f32>) -> bool + Send + Sync>,
    /// Internal: tracks elapsed time for After/Before triggers.
    elapsed: f32,
    /// Has the func already returned true?
    complete: bool,
    /// Has the timer requirement been met?
    time_done: bool,
    /// Has the func been invoked at least once? (for Before trigger)
    func_invoked: bool,
}

impl GameEvent {
    pub fn immediate(func: impl FnMut(Option<f32>) -> bool + Send + Sync + 'static) -> Self {
        Self {
            trigger: EventTrigger::Immediate,
            blocking: true,
            blockable: true,
            no_delete: false,
            func: Box::new(func),
            elapsed: 0.0,
            complete: false,
            time_done: false,
            func_invoked: false,
        }
    }

    pub fn after(
        delay: f32,
        func: impl FnMut(Option<f32>) -> bool + Send + Sync + 'static,
    ) -> Self {
        Self {
            trigger: EventTrigger::After { delay },
            blocking: true,
            blockable: true,
            no_delete: false,
            func: Box::new(func),
            elapsed: 0.0,
            complete: false,
            time_done: false,
            func_invoked: false,
        }
    }

    pub fn ease(
        ease_type: EasingType,
        start_val: f32,
        end_val: f32,
        duration: f32,
        func: impl FnMut(Option<f32>) -> bool + Send + Sync + 'static,
    ) -> Self {
        Self {
            trigger: EventTrigger::Ease {
                ease_type,
                start_val,
                end_val,
                duration,
                elapsed: 0.0,
            },
            blocking: true,
            blockable: true,
            no_delete: false,
            func: Box::new(func),
            elapsed: 0.0,
            complete: false,
            time_done: false,
            func_invoked: false,
        }
    }

    pub fn condition(
        func: impl FnMut(Option<f32>) -> bool + Send + Sync + 'static,
    ) -> Self {
        Self {
            trigger: EventTrigger::Condition,
            blocking: true,
            blockable: true,
            no_delete: false,
            func: Box::new(func),
            elapsed: 0.0,
            complete: false,
            time_done: false,
            func_invoked: false,
        }
    }

    pub fn before(
        delay: f32,
        func: impl FnMut(Option<f32>) -> bool + Send + Sync + 'static,
    ) -> Self {
        Self {
            trigger: EventTrigger::Before { delay },
            blocking: true,
            blockable: true,
            no_delete: false,
            func: Box::new(func),
            elapsed: 0.0,
            complete: false,
            time_done: false,
            func_invoked: false,
        }
    }

    /// Builder: set blocking flag.
    pub fn with_blocking(mut self, blocking: bool) -> Self {
        self.blocking = blocking;
        self
    }

    /// Builder: set blockable flag.
    pub fn with_blockable(mut self, blockable: bool) -> Self {
        self.blockable = blockable;
        self
    }

    /// Builder: mark undeletable.
    pub fn with_no_delete(mut self, no_delete: bool) -> Self {
        self.no_delete = no_delete;
        self
    }
}

/// Named event queues (mirrors Lua's five queues).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventQueue {
    Base,
    Unlock,
    Tutorial,
    Achievement,
    Other,
}

/// Central event manager resource. Holds per-queue VecDeques of `GameEvent`s.
#[derive(Resource)]
pub struct EventManager {
    pub queues: [(EventQueue, VecDeque<GameEvent>); 5],
}

impl Default for EventManager {
    fn default() -> Self {
        Self {
            queues: [
                (EventQueue::Unlock, VecDeque::new()),
                (EventQueue::Base, VecDeque::new()),
                (EventQueue::Tutorial, VecDeque::new()),
                (EventQueue::Achievement, VecDeque::new()),
                (EventQueue::Other, VecDeque::new()),
            ],
        }
    }
}

impl EventManager {
    /// Push an event to the back (or front) of the named queue.
    pub fn add_event(&mut self, event: GameEvent, queue: EventQueue, front: bool) {
        let q = self.get_queue_mut(queue);
        if front {
            q.push_front(event);
        } else {
            q.push_back(event);
        }
    }

    /// Convenience: push to `Base` queue at the back.
    pub fn add(&mut self, event: GameEvent) {
        self.add_event(event, EventQueue::Base, false);
    }

    /// Clear a specific queue (respects `no_delete`).
    pub fn clear_queue(&mut self, queue: EventQueue) {
        let q = self.get_queue_mut(queue);
        q.retain(|e| e.no_delete);
    }

    /// Clear all queues except `exception` (respects `no_delete`).
    pub fn clear_all_except(&mut self, exception: Option<EventQueue>) {
        for (name, q) in self.queues.iter_mut() {
            if Some(*name) != exception {
                q.retain(|e| e.no_delete);
            }
        }
    }

    /// Clear every queue.
    pub fn clear_all(&mut self) {
        self.clear_all_except(None);
    }

    fn get_queue_mut(&mut self, queue: EventQueue) -> &mut VecDeque<GameEvent> {
        for (name, q) in self.queues.iter_mut() {
            if *name == queue {
                return q;
            }
        }
        unreachable!("all queues are pre-allocated");
    }
}

/// System that processes the event manager each frame (mirrors Lua `EventManager:update`).
fn update_event_manager(time: Res<Time>, mut manager: ResMut<EventManager>) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    for (_name, queue) in manager.queues.iter_mut() {
        let mut blocked = false;
        let mut i = 0;
        while i < queue.len() {
            let event = &mut queue[i];

            // Skip if blocked and this event is blockable
            if blocked && event.blockable {
                i += 1;
                continue;
            }

            handle_event(event, dt);

            if event.blocking {
                blocked = true;
            }

            if event.complete && event.time_done {
                queue.remove(i);
                // don't increment i; the next element slid into this slot
            } else {
                i += 1;
            }
        }
    }
}

fn handle_event(event: &mut GameEvent, dt: f32) {
    match &mut event.trigger {
        EventTrigger::Immediate => {
            event.complete = (event.func)(None);
            event.time_done = true;
        }
        EventTrigger::After { delay } => {
            event.elapsed += dt;
            if event.elapsed >= *delay {
                event.time_done = true;
                if !event.complete {
                    event.complete = (event.func)(None);
                }
            }
        }
        EventTrigger::Ease {
            ease_type,
            start_val,
            end_val,
            duration,
            elapsed,
        } => {
            *elapsed += dt;
            if *elapsed >= *duration {
                // Final value
                event.complete = (event.func)(Some(*end_val));
                event.time_done = true;
            } else {
                let remaining = (*duration - *elapsed) / *duration;
                let val = ease_value(*ease_type, remaining, *start_val, *end_val);
                (event.func)(Some(val));
            }
        }
        EventTrigger::Condition => {
            if !event.complete {
                event.complete = (event.func)(None);
            }
            event.time_done = true;
        }
        EventTrigger::Before { delay } => {
            if !event.func_invoked {
                event.complete = (event.func)(None);
                event.func_invoked = true;
            }
            event.elapsed += dt;
            if event.elapsed >= *delay {
                event.time_done = true;
            }
        }
    }
}
