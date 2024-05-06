use bevy::prelude::*;
use bevy_hanabi::{ParticleEffect, ParticleEffectBundle};
use crate::abilities::abilities::AutoDestroy;
use crate::entity::particles::ParticleType;
use crate::entity::particles::Particles;
use super::damage::*;
use bevy::utils::hashbrown::HashMap;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<HealthDamageEvent>()
            .add_event::<HealthDeathEvent>()
            .register_type::<Health>()
            .add_systems(Update, (health_update, on_damage));
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
    entity_type: EntityType,
    incoming_damage: Vec<DamageInstance>,
    #[reflect(ignore)]
    dots: HashMap<u32, DOT>
}

#[derive(Debug, Reflect, Clone, Copy)]
pub struct DamageInstance {
    amount: f32, 
    damage_type: DamageType,
    spawn_damage_particles: bool
}

impl DamageInstance {
    pub fn new(amount: f32, damage_type: DamageType, spawn_damage_particles: bool) -> Self {
        DamageInstance { amount, damage_type, spawn_damage_particles }
    }
}

#[derive(Clone, Reflect, Copy)]
pub struct DOT {
    pub tick_damage: f32, 
    pub duration: f32, 
    pub damage_type: DamageType,
    pub finished: bool
}

#[derive(Reflect, PartialEq, Eq)]
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
    pos: Vec2,
    amount: f32
}

fn on_damage(mut commands: Commands, mut evr: EventReader<HealthDamageEvent>, particles: Res<Particles>) {
    let Some(effect) = particles.effects.get(&ParticleType::Hit) else { return; };
    for event in evr.read() {
        info!("{} was damaged!", event.entity.index());
        commands.spawn(ParticleEffectBundle {
            effect: ParticleEffect::new(effect.clone()),
            transform: Transform::from_translation(event.pos.extend(10.0)).with_scale(Vec3::splat(10.0)),
            ..Default::default()
        }).insert(AutoDestroy::new(10.0));
    }
}

#[allow(dead_code)]
#[derive(Event)]
pub struct HealthDeathEvent {
    pub entity: Entity, 
    pub entity_type: EntityType
}

pub fn health_update(
    time: Res<Time>, 
    mut ev_damage: EventWriter<HealthDamageEvent>, 
    mut ev_death: EventWriter<HealthDeathEvent>, 
    mut query: Query<(&mut Health, Entity, &Transform), Changed<Health>> // Adapt to use health UI later
) {
    let mut damage_instances = Vec::<DamageInstance>::new();
    for (mut health, entity, transform) in query.iter_mut() {
        let en_type = health.entity_type.clone();
        for damage_instance in &health.incoming_damage {
            if damage_instance.spawn_damage_particles {
                ev_damage.send(HealthDamageEvent { entity, entity_type: en_type.clone(), amount: damage_instance.amount, pos: transform.translation.truncate() });
            }
            damage_instances.push(damage_instance.clone());
        }
        for damage_instance in damage_instances.iter() {
            health.damage(damage_instance.amount, damage_instance.damage_type);
        }
        health.incoming_damage.clear();
        damage_instances.clear();
        if health.dead {
            ev_death.send( HealthDeathEvent { entity, entity_type: health.entity_type.clone() });
        }

        if health.dots.len() == 0 {
            return;
        }

        let mut finished: Vec<u32> = Vec::new();

        for (entity, dot) in health.dots.iter_mut() {
            dot.duration = (dot.duration - time.delta_seconds()).max(0.0);
            damage_instances.push(DamageInstance::new(dot.tick_damage * time.delta_seconds(), dot.damage_type, false));
            if dot.duration == 0.0 {
                finished.push(*entity);
            }
        }
        
        for damage_instance in damage_instances.iter() {
            health.incoming_damage.push(*damage_instance);
        }

        for id in finished {
            health.dots.remove(&id);
        }
    }
}

impl Health {
    pub fn new(health: f32, physical_defence: i32, magical_defence: i32, entity_type: EntityType) -> Health {
        Self { current_health: health, max_health: health, magical_defence, physical_defence, dead: false, is_invulnerable: false, incoming_damage: Vec::new(), entity_type, dots: HashMap::new() }
    }

    pub fn push_damage(&mut self, amount: f32, damage_type: DamageType) {
        self.incoming_damage.push(DamageInstance::new(amount, damage_type, true));
    }

    fn damage(&mut self, mut amount: f32, damage_type: DamageType) {
        if !self.dead && !self.is_invulnerable {
            amount *= self.defence_multiplier(damage_type);
            self.current_health = f32::max(0.0, self.current_health - amount);
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
            DamageType::PHYSICAL => multiplier_from_defence(self.physical_defence),
            DamageType::MAGICAL => multiplier_from_defence(self.magical_defence),
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
