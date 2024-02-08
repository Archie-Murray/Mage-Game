use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use crate::entity::stat_type::StatType;
#[derive(Component)]
pub struct Stats {
    stats: HashMap<StatType, f32>
}

impl Stats {
    pub fn add_stat(&mut self, stat_type: StatType, amount: f32) {
        let stat = self.stats.get_mut(&stat_type);
        if let Some(stat) = stat {        
            *stat += amount;
        }
    }

    pub fn get_stat(&self, stat_type: StatType) -> Option<&f32> {
        return self.stats.get(&stat_type);
    }
}

impl Default for Stats {
    fn default() -> Self {
        return Stats { stats: HashMap::from([
            (StatType::Health, 100.0),
            (StatType::Defence, 10.0),
            (StatType::MagicDefence, 10.0),
            (StatType::Speed, 100.0),
            (StatType::Attack, 25.0),
            (StatType::Magic, 20.0)
        ])}
    }
}

#[derive(Event)]
pub struct OnStatChangeEvent {
    pub stat_type: StatType,
    pub amount: f32 
}
