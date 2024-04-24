use bevy::prelude::*;
use crate::damage::damagetype::DamageType;
use crate::damage;
use bevy::utils::hashbrown::HashMap;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<HealthDamageEvent>()
            .add_event::<HealthDeathEvent>()
            .register_type::<Health>()
            .add_systems(Update, health_update);
    }
}

#[derive(Component, Reflect, Clone)]
pub struct Health {
    current_health: f32,
    max_health: f32,
    magical_defence: i32,
    physical_defence: i32,
    dead: bool,
    is_invulnerable: bool,
    damage_timer: DamageTimer,
    entity_type: EntityType,
    #[reflect(ignore)]
    dots: HashMap<u32, DOT>
}

#[derive(Clone, Reflect, Copy)]
pub struct DOT {
    pub tick_damage: f32, 
    pub duration: f32, 
    pub damage_type: DamageType,
    pub finished: bool
}

#[derive(Reflect, Clone)]
pub struct DamageTimer {
    pub timer: Timer,
    pub amount: f32
}

#[derive(Reflect)]
pub enum EntityType { Player, Enemy, Boss }

impl Clone for EntityType {
    fn clone(&self) -> Self {
        match self {
            Self::Player => EntityType::Player,
            Self::Enemy => EntityType::Enemy,
            Self::Boss => EntityType::Boss
        }
    }
}

#[allow(dead_code)]
#[derive(Event)]
pub struct HealthDamageEvent {
    entity: Entity, 
    entity_type: EntityType, 
    amount: f32
}
#[allow(dead_code)]
#[derive(Event)]
pub struct HealthDeathEvent {
    entity: Entity, 
    entity_type: EntityType
}

pub fn health_update(
    time: Res<Time>, 
    mut ev_damage: EventWriter<HealthDamageEvent>, 
    mut ev_death: EventWriter<HealthDeathEvent>, 
    mut query: Query<(&mut Health, Entity)> // Adapt to use health UI later
) {
    for (mut health, entity) in query.iter_mut() {
        if health.damage_timer.timer.fraction() <= 0.0 {
            ev_damage.send(HealthDamageEvent { entity,  entity_type: health.entity_type.clone(), amount: health.damage_timer.amount });
            if health.dead {
                ev_death.send( HealthDeathEvent { entity, entity_type: health.entity_type.clone() });
            }
        }

        if health.dots.len() == 0 {
            return;
        }

        let mut finished: Vec<u32> = Vec::new();

        for (entity, dot) in health.dots.iter_mut() {
            dot.duration = (dot.duration - time.delta_seconds()).max(0.0);
            if dot.duration == 0.0 {
                finished.push(*entity);
            }
        }

        for id in finished {
            health.dots.remove(&id);
        }
    }
}

impl Health {
    pub fn new(health: f32, physical_defence: i32, magical_defence: i32, entity_type: EntityType) -> Health {
        Self { current_health: health, max_health: health, magical_defence, physical_defence, dead: false, is_invulnerable: false, damage_timer: DamageTimer { timer: Timer::from_seconds(0.25, TimerMode::Once), amount: 0.0 }, entity_type, dots: HashMap::new() }
    }

    pub fn damage(&mut self, mut amount: f32, damage_type: DamageType) {
        if !self.dead && !self.is_invulnerable {
            amount *= self.defence_multiplier(damage_type);
            self.current_health = f32::max(0.0, self.current_health - amount);
            self.damage_timer.timer.reset();
            self.damage_timer.amount = amount;
            if self.current_health == 0.0 {
                self.dead = true;
            }
        }
    }

    pub fn heal(&mut self, mut amount: f32) {
        amount = amount.max(0.0);
        self.current_health = self.max_health.min(self.current_health + amount);
    }

    pub fn add_dot(&mut self, damage_per_second: f32, duration: f32, damage_type: DamageType, entity_id: u32) {
        self.dots.insert(entity_id, DOT { tick_damage: damage_per_second, duration, damage_type, finished: false });
    }

    fn defence_multiplier(&mut self, damage_type: DamageType) -> f32 {
        match damage_type {
            DamageType::PHYSICAL => damage::multiplier_from_defence(self.physical_defence),
            DamageType::MAGICAL => damage::multiplier_from_defence(self.magical_defence),
            _ => 1.0
        }
    }

    pub fn get_percent(&self) -> f32 {
        f32::clamp(self.current_health / self.max_health, 0.0, 1.0)
    }

    pub fn get_max(&self) -> f32 {
        self.max_health
    }

    #[allow(dead_code)]
    pub fn get_current(&self) -> f32 {
        self.current_health
    }
}
