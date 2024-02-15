use bevy::prelude::{Plugin, App, Update};

pub mod stats;
pub mod stat_type;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, stats::update_stats);
    }
}
