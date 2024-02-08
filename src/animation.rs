use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize
}

#[derive(PartialEq, Eq, Hash)]
pub enum AnimationType { Idle, Walk, Run, Cast, SpecialCast }
#[derive(PartialEq, Eq, Hash)]
pub enum AnimationDirection { Up, Down, Left, Right }

pub fn vec2_to_direction(vector: &Vec2) -> AnimationDirection {
    if vector.x.abs() > 0.0 {
        return if vector.x > 0.0 { AnimationDirection::Right } else { AnimationDirection::Left };
    } else {
        return if vector.y > 0.0 { AnimationDirection::Down } else { AnimationDirection::Up };
    }
}

#[derive(Component)]
pub struct Animations {
    pub animation_indices: HashMap<AnimationType, HashMap<AnimationDirection, AnimationIndices>>,
    pub current: AnimationType
}

#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
    pub is_animating: bool
}
