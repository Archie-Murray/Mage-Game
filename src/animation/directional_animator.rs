use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use super::*;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum AnimationType { Idle, Walk, Run, Cast, SpecialCast }
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum AnimationDirection { Up, Down, Left, Right }


pub fn vec2_to_direction(vector: &Vec2) -> AnimationDirection {
    if vector.x.abs() > 0.0 || vector.y.abs() == 0.0 {
        return if vector.x > 0.0 { AnimationDirection::Right } else { AnimationDirection::Left };
    } else {
        return if vector.y > 0.0 { AnimationDirection::Up } else { AnimationDirection::Down };
    }
}

#[derive(Component)]
pub struct DirectionalAnimator {
    pub animation_indices: HashMap<AnimationType, HashMap<AnimationDirection, AnimationIndices>>,
    pub current: AnimationType,
    pub previous: AnimationType,
    pub current_dir: AnimationDirection,
    pub previous_dir: AnimationDirection,
}

impl DirectionalAnimator {
    pub fn update_animation(&mut self, animation: AnimationType) {
        self.previous = self.current;
        self.current = animation;
    }

    pub fn update_direction(&mut self, direction: AnimationDirection) {
        self.previous_dir = self.current_dir;
        self.current_dir = direction;
    }

    pub fn get_animation(&self) -> &AnimationIndices {
        if let Some(current) = self.animation_indices.get(&self.current) {
            if let Some(indices) = current.get(&self.current_dir) {
                return indices;
            }
        }
        return &DEFAULT;
    }
}
