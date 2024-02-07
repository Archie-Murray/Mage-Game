use bevy::prelude::*;
use crate::damage::damagetype::DamageType;
use crate::damage;
#[derive(Component)]
pub struct Health {
    current_health: f32,
    max_health: f32,
    magical_defence: i32,
    phyiscal_defence: i32,
    dead: bool,
    is_invulnerable: bool,
    damage_timer: DamageTimer,
    entity_type: EntityType
}

pub struct DamageTimer {
    pub timer: Timer,
    pub amount: f32
}

pub enum EntityType { Player, Enemy, Boss }

impl Clone for EntityType {
    fn clone(&self) -> Self {
        return match self {
            Self::Player => EntityType::Player,
            Self::Enemy => EntityType::Enemy,
            Self::Boss => EntityType::Boss
        };
    }
}

#[derive(Event)]
pub struct HealthDamageEvent {
    entity: Entity, 
    entity_type: EntityType, 
    amount: f32
}
#[derive(Event)]
pub struct HealthDeathEvent {
    entity: Entity, 
    entity_type: EntityType
}

pub fn on_health_damage(reader: &mut EventReader<HealthDamageEvent>) {}
pub fn on_health_death(reader: &mut EventReader<HealthDamageEvent>) {}
pub fn health_update(
    time: Res<Time>, 
    mut ev_damage: EventWriter<HealthDamageEvent>, 
    mut ev_death: EventWriter<HealthDeathEvent>, 
    mut query: Query<(&mut Health, Entity)> // Adapt to use health UI later
) {
    for (mut health, entity) in query.iter_mut() {
        if !health.damage_timer.timer.finished() {
            ev_damage.send(HealthDamageEvent { entity,  entity_type: health.entity_type.clone(), amount: health.damage_timer.amount });
            if health.dead {
                ev_death.send( HealthDeathEvent { entity, entity_type: health.entity_type.clone() });
            }
        }
    }
}

impl Health {
    pub fn new(health: f32, physical_defence: i32, magical_defence: i32, entity_type: EntityType) -> Health {
        return Self { current_health: health, max_health: health, magical_defence, phyiscal_defence: physical_defence, dead: false, is_invulnerable: false, damage_timer: DamageTimer { timer: Timer::from_seconds(0.25, TimerMode::Once), amount: 0.0 }, entity_type }
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

    fn defence_multiplier(&mut self, damage_type: DamageType) -> f32 {
        return match damage_type {
            DamageType::PHYSICAL => damage::multiplier_from_defence(self.phyiscal_defence),
            DamageType::MAGICAL => damage::multiplier_from_defence(self.magical_defence),
            _ => 1.0
        };
    }
}
