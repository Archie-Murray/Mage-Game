use crate::{
    animation::{directional_animator::*, *},
    damage::health::Health,
    entity::stats::Stats,
};
use bevy::{math::Vec2, utils::hashbrown::HashMap};

pub struct EnemyData {
    pub animator: DirectionalAnimator,
    pub sprite_data: SpriteData,
    pub health: Health,
    pub stats: Stats,
}

pub struct SpriteData {
    pub path: &'static str,
    pub tile_size: Vec2,
    pub columns: usize,
    pub rows: usize,
    pub padding: Option<Vec2>,
    pub offset: Option<Vec2>,
}

pub fn orc() -> EnemyData {
    return EnemyData {
        animator: DirectionalAnimator {
            animation_indices: HashMap::from([
                (
                    AnimationType::Idle,
                    HashMap::from([
                        (AnimationDirection::Up, AnimationIndices::new(0, 6)),
                        (AnimationDirection::Left, AnimationIndices::new(9, 15)),
                        (AnimationDirection::Down, AnimationIndices::new(18, 24)),
                        (AnimationDirection::Right, AnimationIndices::new(27, 33)),
                    ]),
                ),
                (
                    AnimationType::Walk,
                    HashMap::from([
                        (AnimationDirection::Up, AnimationIndices::new(36, 44)),
                        (AnimationDirection::Left, AnimationIndices::new(45, 53)),
                        (AnimationDirection::Down, AnimationIndices::new(54, 62)),
                        (AnimationDirection::Right, AnimationIndices::new(63, 71)),
                    ]),
                ),
                (
                    AnimationType::Cast,
                    HashMap::from([
                        (AnimationDirection::Up, AnimationIndices::new(72, 77)),
                        (AnimationDirection::Left, AnimationIndices::new(81, 86)),
                        (AnimationDirection::Down, AnimationIndices::new(90, 95)),
                        (AnimationDirection::Right, AnimationIndices::new(99, 104)),
                    ]),
                ),
            ]),
            animation: AnimationType::Idle,
            direction: AnimationDirection::Up,
            last_update_timer: 0.0
        },
        sprite_data: SpriteData {
            path: "enemy/orc.png",
            tile_size: Vec2::splat(64.0),
            columns: 9,
            rows: 12,
            padding: None,
            offset: None,
        },
        stats: Stats::new(1000.0, 25.0, 5.0, 2.0, 10.0, 0.0),
        health: Health::new(1000.0, 25, 5, crate::damage::health::EntityType::Enemy),
    };
}
