use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use crate::animation::directional_animator::{vec2_to_direction, AnimationType, DirectionalAnimator};

pub mod spawner;
pub mod orc;
pub mod data;

#[derive(Component, Reflect)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub enemy_state: EnemyState,
    pub action_timer: Timer,
    pub anim_timer: Timer,
}

#[derive(Debug, Clone, Copy, Reflect)]
pub enum EnemyState { Idle, Wander, Chase, Attack, Death }

#[derive(Debug, Clone, Copy, Reflect)]
pub enum EnemyType { Orc }

impl Enemy {
    pub fn new(enemy_type: EnemyType) -> Self {
        Enemy { enemy_type, enemy_state: EnemyState::Idle, action_timer: Timer::from_seconds(5.0, TimerMode::Once), anim_timer: Timer::from_seconds(0.0, TimerMode::Once) }
    }
}

#[derive(Event)]
pub struct EnemySpawnEvent {
    pub entity: Entity,
    pub enemy_type: EnemyType
}

pub fn enemy_spawn_init(
    mut commands: Commands,
    mut spawned_enemies: EventReader<EnemySpawnEvent>
) {
    for spawned_enemy in spawned_enemies.read() {
        info!("Spawn init!");
        match spawned_enemy.enemy_type {
            EnemyType::Orc => { commands.entity(spawned_enemy.entity).insert(orc::OrcIdle); },
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<spawner::EnemyManager>();
        app.add_event::<EnemySpawnEvent>();
        app.add_systems(Startup, spawn_spawners);
        app.add_systems(FixedUpdate, (spawner::update_spawners, enemy_spawn_init));
        app.add_systems(Update, update_enemy_animations);
    }
}

pub fn spawn_spawners(mut commands: Commands) {
    commands.spawn(
        spawner::EnemySpawner::new(
            EnemyType::Orc, 
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

