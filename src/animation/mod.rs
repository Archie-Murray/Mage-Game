use bevy::prelude::*;

pub mod looping_animator;
pub mod directional_animator;

pub struct AnimationIndices {
    pub first: usize,
    pub last: usize
}
impl AnimationIndices {
    fn new(first: usize, last: usize) -> Self {
        return AnimationIndices {first, last};
    }
}

pub const DEFAULT: AnimationIndices = AnimationIndices { first: 0, last: 0 };

impl Default for AnimationIndices {
    fn default() -> Self {
        return AnimationIndices { first: 0, last: 0 }
    }
}
pub struct AnimatorPlugin;

impl Plugin for AnimatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, looping_animator::update_looping_animations);
    }
}


#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
    pub is_animating: bool
}
