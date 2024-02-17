use std::time::Duration;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy::utils::hashbrown::HashMap;

use bevy_hanabi::prelude::*;

use bevy_rapier2d::prelude::*;

use crate::entity::stat_type::StatType;
use crate::input::Mouse;

use crate::animation::looping_animator::LoopingAnimator;

use crate::damage::{health::Health, damagetype::DamageType};
use crate::entity::stats::Stats;

use crate::player::Player;

use crate::abilities::ability_particles::{AbilityParticle, ParticleType};

#[derive(Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub enum EffectType { Slow, Damage, Heal, Stun }

#[derive(Reflect, InspectorOptions, Hash, PartialEq, Eq, Debug, Copy, Clone)]
#[reflect(InspectorOptions)]
pub enum AbilityType { FireBall, IceStorm, HealOrb }

#[derive(Reflect)]
pub struct AbilityData {
    pub ability_type: AbilityType,
    pub cooldown: f32,
    pub magnitude: f32,
    pub speed: f32
}

impl AbilityData {
    pub fn from_type(ability_type: AbilityType) -> Self {
        return match ability_type {
            AbilityType::FireBall => AbilityData { ability_type: AbilityType::FireBall, cooldown: 2.0, magnitude: 5.0, speed: 100.0 },
            AbilityType::IceStorm => AbilityData { ability_type: AbilityType::IceStorm, cooldown: 5.0, magnitude: 1.0, speed: 25.0 },
            AbilityType::HealOrb => AbilityData { ability_type: AbilityType::HealOrb, cooldown: 10.0, magnitude: 10.0, speed: 0.0 }
        };
    }
}

#[derive(Component, Reflect)]
pub struct AbilitySystem {
    pub abilities: Vec<Ability>
}

#[derive(Resource)]
pub struct AbilityBundle {
    pub sprites: HashMap<AbilityType, SpriteSheetBundle>
}

impl Default for AbilityBundle {
    fn default() -> Self {
        return AbilityBundle { sprites: HashMap::new() };
    }
}

#[derive(Reflect)]
pub struct Ability {
    pub ability_data: AbilityData,
    pub cooldown_timer: Timer,
    pub done: bool
}

impl Ability {
    fn new(ability_type: AbilityType) -> Self {
        let data = AbilityData::from_type(ability_type);
        return Ability { cooldown_timer: Timer::new(Duration::from_secs_f32(data.cooldown), TimerMode::Once), ability_data: data, done: true };
    }

    pub fn can_use(&self) -> bool {
        return self.cooldown_timer.finished();
    }

    fn update_ability(&mut self, delta_time: f32) {
        self.cooldown_timer.tick(Duration::from_secs_f32(delta_time));
        if self.can_use() {
            self.done = true;
        }
    }
}

impl AbilitySystem {
    pub fn get_ability(&mut self, slot: usize) -> Option<&mut Ability> {
        return self.abilities.get_mut(slot);
    }
}

pub struct AbilitySystemPlugin;

impl Plugin for AbilitySystemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AbilityBundle>();
        app.add_systems(Startup, init_abilites);
        app.add_systems(Update, (update_abilities, cast_ability, player_heal, player_dot, player_damage, player_slow, auto_destroy_abilities, auto_destroy_entities));
    }
}

fn init_abilites(
    mut abilities: ResMut<AbilityBundle>, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    abilities.sprites = HashMap::from([
        (AbilityType::FireBall, SpriteSheetBundle { 
            texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("abilities/fire_ball.png"), Vec2::splat(32.0), 5, 1, None, None)),
            sprite: TextureAtlasSprite::new(0), 
            transform: Transform::from_scale(Vec3::splat(1.0)), 
            .. default()
        }),
        (AbilityType::IceStorm, SpriteSheetBundle { 
            texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("abilities/ice_storm.png"), Vec2::splat(64.0), 1, 1, None, None)),
            sprite: TextureAtlasSprite::new(0), 
            transform: Transform::from_scale(Vec3::splat(1.0)),
            .. default()
        }),        
        (AbilityType::HealOrb, SpriteSheetBundle { 
            texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("abilities/heal_orb.png"), Vec2::splat(32.0), 5, 1, None, None)),
            sprite: TextureAtlasSprite::new(0), 
            transform: Transform::from_scale(Vec3::splat(1.0)),
            .. default()
        }) 
    ]);
}
pub fn update_abilities(mut query: Query<&mut AbilitySystem>, time: Res<Time>) {
    let mut system = query.single_mut();
    for ability in system.abilities.iter_mut() {
        ability.update_ability(time.delta_seconds());
    }
}

fn is_ability_key(key_code: KeyCode) -> bool {
    return key_code == KeyCode::Q || key_code == KeyCode::E || key_code == KeyCode::R;
}

fn get_ability_slot(key_code: &KeyCode) -> Option<usize> {
    return match key_code {
        KeyCode::Q => Some(0),
        KeyCode::E => Some(1),
        KeyCode::R => Some(2),
        _ => None
    };
}

pub fn cast_ability(
    commands: Commands,
    ability_sprites: ResMut<AbilityBundle>,
    ability_particles: ResMut<AbilityParticle>,
    mut query: Query<(&mut AbilitySystem, &Transform)>,
    mouse: Res<Mouse>,
    keyboard: Res<Input<KeyCode>>,
) {
    let (mut ability_system, transform) = query.single_mut();
    let Some(slot) = get_ability_slot(
        keyboard.get_just_pressed().filter(|key_code| is_ability_key(**key_code)).next()
        .unwrap_or(&KeyCode::NoConvert)) else { return; };
    let Some(ability ) = ability_system.get_ability(slot) else { return; };
    let mouse_diff = (mouse.world_position - Vec2::new(transform.translation.x, transform.translation.y)).normalize();
    if ability.can_use() {
        let rotation = Quat::from_axis_angle(
            Vec3::new(0.0, 0.0, -1.0), 
            Vec2::angle_between(mouse_diff, Vec2::new(0.0, -1.0)) + std::f32::consts::FRAC_PI_2
        );
        use_ability(ability, transform, rotation, commands, ability_sprites, ability_particles);
    }

}

fn use_ability(ability: &mut Ability, origin: &Transform, rotation: Quat, mut commands: Commands, mut ability_sprites: ResMut<AbilityBundle>, ability_particles: ResMut<AbilityParticle>) {
    ability.cooldown_timer.set_duration(Duration::from_secs_f32(ability.ability_data.cooldown));
    ability.cooldown_timer.reset();
    if let Some(mut ability_sprite) = ability_sprites.sprites.get_mut(&ability.ability_data.ability_type).cloned() {
        println!("Spawning ability: {:?}", ability.ability_data.ability_type);
        let (_, _, angle) = rotation.to_euler(EulerRot::XYZ);
        ability_sprite.transform.translation = origin.translation + rotation.mul_vec3(Vec3::new(1.0, 0.0, 0.0)) * 64.0;
        ability_sprite.transform.translation = origin.translation;
        match ability.ability_data.ability_type {
            AbilityType::FireBall => {
                let (mut ability_instance , damage, animator, 
                         grav, rb, constraints, coll, sensor , vel, ability, auto_destroy
                ) = (
                    ability_sprite, 
                    Damage { damage_amount: ability.ability_data.magnitude, damage_type: DamageType::MAGICAL }, 
                    LoopingAnimator::new(4, 0.2),
                    GravityScale(0.0),
                    RigidBody::Dynamic,
                    LockedAxes::ROTATION_LOCKED,
                    Collider::ball(32.0),
                    Sensor,
                    Velocity { linvel: Vec2::from_angle(angle) * ability.ability_data.speed, angvel: 0.0 },
                    AbilityTag { ability_type: ability.ability_data.ability_type },
                    AutoDestroy::new(2.0)
                ); 
                ability_instance.transform.rotation = rotation;
                commands.spawn((ability_instance , damage, animator, grav, rb, constraints, coll, sensor, vel, ability, auto_destroy));
            },
            AbilityType::IceStorm => {
                let (mut ability_instance, damage_over_time , slow, 
                         grav, rb, constraints, coll, sensor, vel, ability, auto_destroy
                ) = (
                    ability_sprite, 
                    DamageOverTime { tick_damage: ability.ability_data.magnitude, damage_type: DamageType::PHYSICAL, duration: 0.5 }, 
                    Slow { speed_reduction: ability.ability_data.magnitude * 10.0, duration: ability.ability_data.magnitude },
                    GravityScale(0.0),
                    RigidBody::KinematicVelocityBased,
                    LockedAxes::ROTATION_LOCKED,
                    Collider::ball(64.0),
                    Sensor,
                    Velocity { linvel: Vec2::from_angle(angle) * ability.ability_data.speed, angvel: 2.0 * std::f32::consts::PI },
                    AbilityTag { ability_type: ability.ability_data.ability_type },
                    AutoDestroy::new(5.0)
                ); 
                ability_instance.transform.rotation = rotation;
                let ability_bundle = (ability_instance, damage_over_time , slow, grav, rb, constraints, coll, sensor, vel, ability, auto_destroy);
                if let Some(particle_effect) = ability_particles.particle_effects.get(&ParticleType::IceStorm) {
                    let particles = commands.spawn(ParticleEffectBundle { effect: ParticleEffect::new(particle_effect.clone()), transform: Transform::from_scale(Vec3::splat(10.0)), ..Default::default() }).id();
                    commands.spawn(ability_bundle).add_child(particles);
                } else {
                    commands.spawn(ability_bundle);
                }
            },
            AbilityType::HealOrb => {
                let (mut ability_instance, heal, grav, rb, 
                         constraints, coll, sensor, vel, ability, auto_destroy
                ) = (
                    ability_sprite, 
                    Heal { heal_amount: ability.ability_data.magnitude }, 
                    GravityScale(0.0),
                    RigidBody::Dynamic,
                    LockedAxes::ROTATION_LOCKED,
                    Collider::ball(4.0),
                    Sensor,
                    Velocity { linvel: Vec2::ZERO, angvel: 0.0 },
                    AbilityTag { ability_type: ability.ability_data.ability_type },
                    AutoDestroy::new(10.0)
                );
                ability_instance.transform.rotation = Quat::IDENTITY;
                commands.spawn((ability_instance , heal, grav, rb, constraints, coll, sensor, vel, ability, auto_destroy));
            }
        };
    }
}

pub fn player_heal(
    mut commands: Commands,
    heal_query: Query<(&Heal, Entity), (With<AbilityTag>, With<Collider>)>,
    mut player_query: Query<(&mut Health, Entity), (With<Player>, With<Collider>)>,
    rapier: Res<RapierContext>
) {
    let (mut player_health, player_entity) = player_query.single_mut();
    for (heal, heal_entity) in heal_query.iter() {
        if rapier.intersection_pair(player_entity, heal_entity).is_some() {
            player_health.heal(heal.heal_amount);
            println!("Healed player for {}", heal.heal_amount);
            commands.entity(heal_entity).despawn_recursive();
        }
    }
}

pub fn player_damage(
    mut commands: Commands,
    mut health_query: Query<(&mut Health, Entity), (With<Collider>, Without<Player>)>,
    damage_query: Query<(&Damage, Entity), (With<AbilityTag>, With<Collider>)>,
    rapier: Res<RapierContext>
) {
    for (mut enemy_health, enemy_entity) in health_query.iter_mut() {
        for (damage, damage_entity) in damage_query.iter() {
            if rapier.intersection_pair(enemy_entity, damage_entity).is_some() {
                enemy_health.damage(damage.damage_amount, damage.damage_type);
                println!("Damaged entity: {}", enemy_entity.index());
                commands.entity(enemy_entity).despawn_recursive();
                break;
            }
        }
    }
}

pub fn player_dot(
    mut health_query: Query<(&mut Health, Entity), (With<Collider>, Without<Player>)>,
    damage_query: Query<(&DamageOverTime, Entity), (With<AbilityTag>, With<Collider>)>,
    rapier: Res<RapierContext>
) {
    for (mut enemy_health, enemy_entity) in health_query.iter_mut() {
        for (dot, damage_entity) in damage_query.iter() {
            if rapier.intersection_pair(enemy_entity, damage_entity).is_some() {
                enemy_health.add_dot(dot.tick_damage, dot.duration, dot.damage_type, damage_entity.index());
            }
        }
    }
}

pub fn player_slow(
    mut stat_query: Query<(&mut Stats, Entity), (With<Collider>, Without<Player>)>,
    slow_query: Query<(&Slow, Entity), (With<AbilityTag>, With<Collider>)>,
    rapier: Res<RapierContext>
) {
    for (mut enemy_stats, enemy_entity) in stat_query.iter_mut() {
        for (slow, slow_entity) in slow_query.iter() {
            if rapier.intersection_pair(enemy_entity, slow_entity).is_some() {
                enemy_stats.add_duration_change(StatType::Speed, slow.speed_reduction, slow.duration, slow_entity.index());
            }
        }
    }
}

impl Default for AbilitySystem {
    fn default() -> Self {
        return AbilitySystem { 
            abilities: vec![
                Ability::new(AbilityType::FireBall),
                Ability::new(AbilityType::IceStorm),
                Ability::new(AbilityType::HealOrb)
            ]
        };
    }
}

#[derive(Component)]
pub struct Heal {
    pub heal_amount: f32
}

#[derive(Component)]
pub struct Slow {
    pub speed_reduction: f32,
    pub duration: f32
}

#[derive(Component)]
pub struct Damage {
    pub damage_amount: f32,
    pub damage_type: DamageType
}

#[derive(Component)]
pub struct DamageOverTime {
    pub tick_damage: f32,
    pub damage_type: DamageType,
    pub duration: f32
}

#[derive(Component)]
pub struct Stun {
    pub stun_duration: f32
}

#[derive(Component)]
pub struct AbilityTag { 
    pub ability_type: AbilityType
}

#[derive(Component, Reflect)]
pub struct AutoDestroy {
    pub duration: f32,
    pub remaining: f32
}

impl AutoDestroy {
    fn new(duration: f32) -> AutoDestroy {
        return AutoDestroy { duration, remaining: duration };
    }
}

fn auto_destroy_abilities(
    time: Res<Time>,
    mut commands: Commands, 
    mut query: Query<(&mut AutoDestroy, &AbilityTag, &Transform, Entity)>,
    particles: Res<AbilityParticle>
) {
    let mut to_destroy: Vec<(Entity, AbilityType, &Transform)> = Vec::new();
    for (mut auto_destroy, ability, transform, entity) in query.iter_mut() {
        auto_destroy.remaining = (auto_destroy.remaining - time.delta_seconds()).max(0.0);
        if auto_destroy.remaining == 0.0 {
            to_destroy.push((entity, ability.ability_type, transform));
        }
    }

    for (entity, ability_type, transform) in to_destroy {
        let (pos, rot) = (transform.translation, transform.rotation);
        match ability_type {
            AbilityType::FireBall => println!("Not implemented"),
            AbilityType::IceStorm => {
                commands.spawn((AutoDestroy::new(0.5), ParticleEffectBundle { effect: ParticleEffect::new(particles.particle_effects.get(&ParticleType::IceStormFinish).unwrap().clone()), transform: transform.with_scale(Vec3::splat(10.0)), ..default()}));
            },
            AbilityType::HealOrb => println!("Not implemented")
        }
        commands.entity(entity).despawn_recursive();
    }
}

fn auto_destroy_entities(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(&mut AutoDestroy, Entity), Without<AbilityTag>>
) {
    let mut to_destroy: Vec<Entity> = Vec::new();
    for (mut auto_destroy, entity) in query.iter_mut() {
        auto_destroy.remaining = (auto_destroy.remaining - time.delta_seconds()).max(0.0);
        if auto_destroy.remaining == 0.0 {
            to_destroy.push(entity);
        }
    }

    for entity in to_destroy {
        commands.entity(entity).despawn_recursive();
    }
}
