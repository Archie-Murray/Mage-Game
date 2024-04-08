use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use crate::animation::directional_animator::{vec2_to_direction, AnimationType, DirectionalAnimator};

#[derive(Component)]
pub struct Enemy {
    pub enemy_type: EnemyType
}

#[derive(Debug, Clone, Copy, Reflect)]
pub enum EnemyType { ORC }

impl Enemy {
    pub fn new(enemy_type: EnemyType) -> Self {
        return Enemy { enemy_type };
   }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<spawner::EnemyManager>();
        app.add_systems(Startup, spawn_spawners);
        app.add_systems(FixedUpdate, spawner::update_spawners);
        app.add_systems(Update, update_enemy_animations);
    }
}

pub fn spawn_spawners(mut commands: Commands) {
    commands.spawn(
        spawner::EnemySpawner::new(
            EnemyType::ORC, 
            1.0, 
            1, 
            vec!(Vec2::new(100.0, 10.0))
        )).insert(Transform::from_xyz(-50.0, 50.0, 1.0));
}

pub fn update_enemy_animations(
    mut enemies: Query<(&Velocity, &mut DirectionalAnimator), With<Enemy>>
) {
    for (enemy_vel, mut enemy_anim) in enemies.iter_mut() {
        if enemy_vel.linvel.length_squared() >= 0.01 {
            enemy_anim.update_animation(AnimationType::Walk);
        }
        enemy_anim.update_direction(vec2_to_direction(&enemy_vel.linvel));
    }
}

pub mod spawner;
pub mod data;
