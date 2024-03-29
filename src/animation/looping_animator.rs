use std::time::Duration;

use bevy::prelude::*;

use crate::animation::AnimationIndices;

#[derive(Component)]
pub struct LoopingAnimator {
    pub indecies: AnimationIndices,
    pub current: usize,
    pub animation_timer: Timer,
}

pub fn update_looping_animations(
    time: Res<Time>,
    mut query: Query<(&mut LoopingAnimator, &mut TextureAtlasSprite)>,
) {
    for (mut animator, mut atlas) in query.iter_mut() {
        animator
            .animation_timer
            .tick(Duration::from_secs_f32(time.delta_seconds()));
        if animator.animation_timer.just_finished() {
            animator.current = if animator.current >= animator.indecies.last {
                animator.indecies.first
            } else {
                animator.current + 1
            };
            atlas.index = animator.current;
        }
    }
}
impl LoopingAnimator {
    pub fn new(last: usize, frame_length: f32) -> Self {
        return LoopingAnimator {
            current: 0,
            indecies: AnimationIndices::new(0, last),
            animation_timer: Timer::new(
                Duration::from_secs_f32(frame_length),
                TimerMode::Repeating,
            ),
        };
    }
}
