use bevy::prelude::*;

pub mod looping_animator;
pub mod directional_animator;

#[derive(Clone, Copy)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
    pub frame_length: f32,
}
impl AnimationIndices {
    pub fn new(first: usize, last: usize) -> Self {
        AnimationIndices {first, last, frame_length: 1.0 / (last as f32 - first as f32).abs() }
    }

    pub fn with_frame_length(first: usize, last: usize, frame_length: f32) -> Self {
        AnimationIndices {first, last, frame_length }
    }
}

pub const DEFAULT: AnimationIndices = AnimationIndices { first: 0, last: 0, frame_length: f32::MAX };

impl Default for AnimationIndices {
    fn default() -> Self {
        return AnimationIndices { first: 0, last: 0, frame_length: f32::MAX }
    }

    
}

pub struct AnimatorPlugin;

impl Plugin for AnimatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (looping_animator::update_looping_animations, directional_animator::animate_directional));
    }
}
