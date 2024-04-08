use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use bevy_inspector_egui::InspectorOptions;
use crate::entity::stat_type::StatType;
#[derive(Component, Clone, Reflect, InspectorOptions)]
pub struct Stats {
    pub stats: [Stat; 6],
    #[reflect(ignore)]
    pub stat_effects: HashMap<u32, StatChangeDuration>,
}

#[derive(Clone, Copy, Reflect)]
pub struct Stat {
    pub stat_type: StatType,
    pub stat_value: f32
}
impl Stat {
    fn new(stat_type: StatType, stat_value: f32) -> Self {
        Stat { stat_type, stat_value }
    }
}

#[derive(Clone, Copy)]
pub struct StatChangeDuration {
    pub stat_type: StatType,
    pub amount: f32,
    pub duration: f32
}

impl Stats {
    pub fn add_stat(&mut self, stat_type: StatType, amount: f32) {
        let Some(stat) = self.stats.iter_mut().find(|iter_stat| iter_stat.stat_type == stat_type) else { return; };
        stat.stat_value += amount;
    }

    pub fn get_stat(&self, stat_type: StatType) -> Option<&f32> {
        let stat = self.stats.iter().find(|stat| stat.stat_type == stat_type);
        if stat.is_some() {
            Some(&stat.unwrap().stat_value)
        } else {
            None
        }
    }

    pub fn get_stat_mut(&mut self, stat_type: StatType) -> Option<&mut f32> {
        let stat = self.stats.iter_mut().find(|stat| stat.stat_type == stat_type);
        if stat.is_some() {
            Some(&mut stat.unwrap().stat_value)
        } else {
            None
        }
    }

    pub fn add_duration_change(&mut self, stat_type: StatType, amount: f32, duration: f32, id: u32, can_change_sign: bool) {
        if self.stat_effects.contains_key(&id) {
            return;
        }
        
        info!("Added stat change for {:?} from {} for duration {}", stat_type, id, duration);
        let amount = if !can_change_sign {
            let Some(current_value) = self.get_stat(stat_type) else { return; };
            if (current_value + amount).signum() == current_value.signum() { amount } else { 0.0 }
        } else {
            amount
        };
        self.add_stat(stat_type, amount);
        self.stat_effects.insert(id, StatChangeDuration { stat_type, amount, duration });
        assert!(self.stat_effects.len() > 0);
        info!("Stat effect count for not {} is {}", id, self.stat_effects.len());
    }

    pub fn new(health: f32, defence: f32, mag_def: f32, speed: f32, attack: f32, magic: f32) -> Self {
        Stats {
            stats: [
                Stat::new(StatType::Health, health),
                Stat::new(StatType::Defence, defence),
                Stat::new(StatType::MagicDefence, mag_def),
                Stat::new(StatType::Speed, speed),
                Stat::new(StatType::Attack, attack),
                Stat::new(StatType::Magic, magic),
            ],
            stat_effects: HashMap::new(),
        }
    }
}

pub fn update_stats(time: Res<Time>, mut stats: Query<&mut Stats>) {
    for mut stat in stats.iter_mut() {
        if stat.stat_effects.keys().len() == 0 {
            continue;
        }
        let mut finished: Vec<u32> = Vec::new();
        for (id, stat_effect) in stat.stat_effects.iter_mut() {
            stat_effect.duration = (stat_effect.duration - time.delta_seconds()).max(0.0);
            info!("Updated stat duration to: {}", stat_effect.duration);
            if stat_effect.duration == 0.0 {
                info!("Stat effect {:?} is finished", stat_effect.stat_type);
                finished.push(*id);
            }
        }
        for id in finished {
            let value = *stat.stat_effects.get(&id).unwrap();
            stat.add_stat(value.stat_type, -value.amount);
            stat.stat_effects.remove(&id);
        }
    }
}

impl Default for Stats {
    fn default() -> Self {
        Stats { stats: [
            Stat::new(StatType::Health, 100.0),
            Stat::new(StatType::Defence, 10.0),
            Stat::new(StatType::MagicDefence, 10.0),
            Stat::new(StatType::Speed, 50.0),
            Stat::new(StatType::Attack, 25.0),
            Stat::new(StatType::Magic, 20.0)
        ], stat_effects: HashMap::new() }
    }
}

#[derive(Event)]
pub struct OnStatChangeEvent {
    pub stat_type: StatType,
    pub amount: f32 
}
