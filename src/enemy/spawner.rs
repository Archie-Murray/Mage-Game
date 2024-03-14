use std::time::Duration;

use bevy::prelude::*;
use crate::enemy::*;

use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct EnemySpawner {
    pub enemy_type: EnemyType,
    pub spawn_timer: Timer,
    pub spawn_delay: f32,
    pub spawn_count: usize,
    pub max_spawns: usize,
}

impl EnemySpawner {
    pub fn new(enemy_type: EnemyType, spawn_delay: f32, max_spawns: usize) -> Self {
        return EnemySpawner { enemy_type, spawn_delay, spawn_timer: Timer::from_seconds(spawn_delay, TimerMode::Repeating), spawn_count: 0, max_spawns };
    }
}

impl Default for EnemyManager {
    fn default() -> Self {
        return EnemyManager { enemies: Vec::new() };
    }
}

#[derive(Resource)]
pub struct EnemyManager {
    pub enemies: Vec<u32>
}

pub fn update_spawners(
    time: Res<Time>,
    mut enemies: ResMut<EnemyManager>,
    mut spawners: Query<(&mut EnemySpawner, &Transform, Entity)>,
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>

) {
    for (mut spawner, transform, entity) in spawners.iter_mut() {
        if spawner.spawn_count == spawner.max_spawns {
            commands.entity(entity).despawn();
            return;
        }
        spawner.spawn_timer.tick(Duration::from_secs_f32(time.delta_seconds()));
        if spawner.spawn_timer.just_finished() {
            let enemy = Enemy::new(spawner.enemy_type);
            let enemy_transform = Transform::from_xyz(transform.translation.x, transform.translation.y, 1.0);
            match spawner.enemy_type {
                EnemyType::ORC => {
                    let orc = data::orc();
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
                    enemies.enemies.push(commands.spawn(enemy)
                        .insert(sprite_bundle)
                        .insert(animator)
                        .insert(health)
                        .insert(stats)
                        .insert(Collider::ball(16.0))
                        .insert(RigidBody::Dynamic)
                        .insert(Velocity::default())
                        .insert(LockedAxes::ROTATION_LOCKED)
                    .id().index());
                },
            }
            spawner.spawn_count += 1;
        };
    }
}
