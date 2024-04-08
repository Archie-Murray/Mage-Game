use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use super::*;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Reflect)]
pub enum AnimationType { Idle, Walk, Run, Cast, SpecialCast }
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Reflect)]
pub enum AnimationDirection { Up, Down, Left, Right }

pub fn vec2_to_direction(vector: &Vec2) -> AnimationDirection {
    if vector.x.abs() > 0.5 || vector.y.abs() == 0.5 {
        return if vector.x > 0.0 { AnimationDirection::Right } else { AnimationDirection::Left };
    } else {
        return if vector.y > 0.0 { AnimationDirection::Up } else { AnimationDirection::Down };
    }
}

#[derive(Component, Reflect, Clone)]
pub struct DirectionalAnimator {
    #[reflect(ignore)]
    pub animation_indices: HashMap<AnimationType, HashMap<AnimationDirection, AnimationIndices>>,
    pub animation: AnimationType,
    pub direction: AnimationDirection,
    pub last_update_timer: f32,
}

impl DirectionalAnimator {
    pub fn update_animation(&mut self, animation: AnimationType) {
        self.animation = animation;
    }

    pub fn update_direction(&mut self, direction: AnimationDirection) {
        self.direction = direction;
    }

    pub fn get_animation(&mut self) -> &AnimationIndices {
        if let Some(current) = self.animation_indices.get(&self.animation) {
            if let Some(indices) = current.get(&self.direction) {
                return indices;
            }
        }
        return &DEFAULT;
    }
}

pub fn animate_directional(
    time: Res<Time>,
    mut animators: Query<(&mut DirectionalAnimator, &mut TextureAtlasSprite)>
) {
    for (mut animator, mut sprite) in animators.iter_mut() {
        let animation_indices = *animator.get_animation();
        animator.last_update_timer += time.delta_seconds();
        while animator.last_update_timer > animation_indices.frame_length {
            animator.last_update_timer -= animation_indices.frame_length;
            if sprite.index > animation_indices.last || sprite.index < animation_indices.first {
                sprite.index = animation_indices.first
            } else {
                sprite.index = if sprite.index + 1 > animation_indices.last { animation_indices.first } else { sprite.index + 1 }
            };
        }
    }
}
