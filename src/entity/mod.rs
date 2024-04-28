use bevy::prelude::{Plugin, App, Update};

pub mod stats;
pub mod particles;
pub mod enemy;
pub mod damage;
pub mod player;
pub mod health;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, stats::update_stats)
            .add_plugins(enemy::EnemyPlugin)
            .add_plugins(player::PlayerPlugin)
            .add_plugins(health::HealthPlugin);
    }
}
