use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize
}

#[derive(PartialEq, Eq, Hash)]
pub enum AnimationType { Idle, Walk, Run, Cast, SpecialCast}

#[derive(Component)]
pub struct Animations {
    pub animation_indices: HashMap<AnimationType, AnimationIndices>,
    pub current: AnimationType
}

#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
}
