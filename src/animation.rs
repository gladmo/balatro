use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_card_animations, update_fade_animations));
    }
}

/// Card move animation component
#[derive(Component)]
pub struct CardAnimation {
    pub start_pos: Vec2,
    pub end_pos: Vec2,
    pub timer: Timer,
    pub easing: EasingType,
}

/// Fade animation component
#[derive(Component)]
pub struct FadeAnimation {
    pub start_alpha: f32,
    pub end_alpha: f32,
    pub timer: Timer,
}

#[derive(Debug, Clone, Copy)]
pub enum EasingType {
    Linear,
    EaseInOut,
    EaseOut,
    EaseIn,
}

impl EasingType {
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            EasingType::Linear => t,
            EasingType::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            EasingType::EaseOut => 1.0 - (1.0 - t).powi(2),
            EasingType::EaseIn => t * t,
        }
    }
}

fn update_card_animations(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut CardAnimation)>,
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
