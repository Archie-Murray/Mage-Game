use bevy::prelude::*;

#[derive(Component)]
pub struct Animation {
    pub first: usize,
    pub last: usize
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);
