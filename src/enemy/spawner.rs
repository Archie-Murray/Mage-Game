use std::time::Duration;

use rand::{thread_rng, Rng};

use bevy::prelude::*;
use crate::{damage::healthbar::HealthBarBundle, enemy::*, pathfinding::AITarget};

use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct EnemySpawner {
    pub enemy_type: EnemyType,
    pub spawn_timer: Timer,
    pub spawn_delay: f32,
    pub spawn_count: usize,
    pub max_spawns: usize,
    pub spawn_points: Vec<Vec2>
}

impl EnemySpawner {
    pub fn new(enemy_type: EnemyType, spawn_delay: f32, max_spawns: usize, spawn_points: Vec<Vec2>) -> Self {
        EnemySpawner { enemy_type, spawn_delay, spawn_timer: Timer::from_seconds(spawn_delay, TimerMode::Repeating), spawn_count: 0, max_spawns, spawn_points }
    }
}

impl Default for EnemyManager {
    fn default() -> Self {
        EnemyManager { enemies: Vec::new() }
    }
}

#[derive(Resource)]
pub struct EnemyManager {
    pub enemies: Vec<u32>
}

pub fn update_spawners(
    time: Res<Time>,
    mut enemies: ResMut<EnemyManager>,
    mut spawners: Query<(&mut EnemySpawner, Entity)>,
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>

) {
    for (mut spawner, entity) in spawners.iter_mut() {
        if spawner.spawn_count == spawner.max_spawns {
            commands.entity(entity).despawn();
            return;
        }
        spawner.spawn_timer.tick(Duration::from_secs_f32(time.delta_seconds()));
        if spawner.spawn_timer.just_finished() {
            let enemy = Enemy::new(spawner.enemy_type);
            let position_index = rand::thread_rng().gen_range(0..spawner.spawn_points.len());
            let enemy_transform = Transform::from_xyz(spawner.spawn_points[position_index].x, spawner.spawn_points[position_index].y, 1.0);
            match spawner.enemy_type {
                EnemyType::ORC => {
                    let orc = data::orc_data();
                    let sprite_bundle: SpriteSheetBundle = SpriteSheetBundle { 
                        sprite: TextureAtlasSprite::new(0), 
                        texture_atlas: textures.add(TextureAtlas::from_grid(
                                assets.load(orc.sprite_data.path), orc.sprite_data.tile_size, orc.sprite_data.columns, orc.sprite_data.rows, orc.sprite_data.padding, orc.sprite_data.offset)
                        ), 
                        transform: enemy_transform,
                        ..default()
                    };

                    let animator = orc.animator;
                    let health = orc.health;
                    let stats = orc.stats;
                    let enemy = commands.spawn(enemy)
                        .insert(sprite_bundle)
                        .insert(animator)
                        .insert(health)
                        .insert(stats)
                        .insert(Collider::ball(16.0))
                        .insert(RigidBody::Dynamic)
                        .insert(Velocity::default())
                        .insert(LockedAxes::ROTATION_LOCKED)
                        .insert(AITarget::new(256.0))
                    .id();
                    let health_bar = commands.spawn(HealthBarBundle::new(data::orc_data().health.get_max(), assets.load("ui/health_bar.png"))).id();
                    commands.entity(enemy).push_children(&[health_bar]);
                    enemies.enemies.push(enemy.index());
                },
            }
            spawner.spawn_count += 1;
        };
    }
}
