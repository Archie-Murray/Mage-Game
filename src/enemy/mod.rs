use bevy::prelude::*;

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
    }
}

pub fn spawn_spawners(mut commands: Commands) {
    commands.spawn(
        spawner::EnemySpawner::new(EnemyType::ORC, 1.0, 1))
            .insert(Transform::from_xyz(-50.0, 50.0, 1.0));
}

pub mod spawner;
pub mod data;
