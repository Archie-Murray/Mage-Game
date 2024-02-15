use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use crate::entity::stat_type::StatType;
#[derive(Component)]
pub struct Stats {
    stats: HashMap<StatType, f32>,
    pub stat_effects: HashMap<u32, StatChangeDuration>
}

#[derive(Clone, Copy)]
pub struct StatChangeDuration {
    pub stat_type: StatType,
    pub amount: f32,
    pub duration: f32
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

    pub fn add_duration_change(&mut self, stat_type: StatType, amount: f32, duration: f32, id: u32) {
        self.stat_effects.insert(id, StatChangeDuration { stat_type, amount, duration });
        self.add_stat(stat_type, -amount);
    }
}

pub fn update_stats(time: Res<Time>, mut stats: Query<&mut Stats>) {
    for mut stat in stats.iter_mut() {
        if stat.stat_effects.len() == 0 {
            return;
        }
        let mut finished: Vec<u32> = Vec::new();
        for (id, mut stat_effect) in stat.stat_effects.iter_mut() {
            stat_effect.duration = (stat_effect.duration - time.delta_seconds()).max(0.0);
            if stat_effect.duration == 0.0 {
                finished.push(*id);
            }
        }
        for id in finished {
            stat.stat_effects.remove(&id);
        }
    }
}

impl Default for Stats {
    fn default() -> Self {
        return Stats { stats: HashMap::from([
            (StatType::Health, 100.0),
            (StatType::Defence, 10.0),
            (StatType::MagicDefence, 10.0),
            (StatType::Speed, 50.0),
            (StatType::Attack, 25.0),
            (StatType::Magic, 20.0)
        ]), stat_effects: HashMap::new()};
    }
}

#[derive(Event)]
pub struct OnStatChangeEvent {
    pub stat_type: StatType,
    pub amount: f32 
}
