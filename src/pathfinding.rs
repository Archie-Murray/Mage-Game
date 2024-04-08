use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::entity::{stat_type::StatType, stats::Stats};

#[derive(Component)]
pub struct AITarget {
    follow_range: f32,
}

impl AITarget {
    pub fn new(follow_range: f32) -> Self {
        return AITarget { follow_range };
    }
} 

pub struct PathfindingPlugin;

pub fn update_ai_paths(
    mut ai_pathfinders: Query<(&mut Velocity, &Transform, &Stats, &AITarget)>,
    player_query: Query<&Transform, (With<crate::player::Player>, Without<AITarget>)>,
) {
    let Ok(player_transform) = player_query.get_single() else { info!("No player!"); return; };

    for (mut pathfinder, transform, stats, target) in ai_pathfinders.iter_mut() {
        let speed = *(stats.get_stat(StatType::Speed).unwrap_or(&100.0));
        if Vec2::distance_squared(player_transform.translation.truncate(), transform.translation.truncate()) <= target.follow_range.powf(2.0) {
            pathfinder.linvel = (player_transform.translation.truncate() - transform.translation.truncate()).normalize() * speed;
        }
    }
}

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_ai_paths);
    }
}
