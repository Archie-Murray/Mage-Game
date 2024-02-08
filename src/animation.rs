use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize
}

pub const DEFAULT: AnimationIndices = AnimationIndices { first: 0, last: 0 };

impl Default for AnimationIndices {
    fn default() -> Self {
        return AnimationIndices { first: 0, last: 0 }
    }
}

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
pub struct Animations {
    pub animation_indices: HashMap<AnimationType, HashMap<AnimationDirection, AnimationIndices>>,
    pub current: AnimationType,
    pub previous: AnimationType,
    pub current_dir: AnimationDirection,
    pub previous_dir: AnimationDirection,
}

impl Animations {
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

#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
    pub is_animating: bool
}
