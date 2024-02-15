use std::time::Duration;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy::utils::hashbrown::HashMap;

use bevy_rapier2d::prelude::*;

use crate::input::Mouse;

use crate::animation::looping_animator::LoopingAnimator;

#[derive(Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub enum EffectType { Slow, Damage, Heal, Stun }

#[derive(Reflect, InspectorOptions, Hash, PartialEq, Eq)]
#[reflect(InspectorOptions)]
pub enum AbilityType { FireBall, IceStorm, HealOrb }

#[derive(Reflect)]
pub struct AbilityData {
    pub id: AbilityType,
    pub cooldown: f32,
    pub magnitude: f32,
    pub speed: f32
}

impl AbilityData {
    pub fn from_type(ability_type: AbilityType) -> Self {
        return match ability_type {
            AbilityType::FireBall => AbilityData { id: AbilityType::FireBall, cooldown: 2.0, magnitude: 5.0, speed: 100.0 },
            AbilityType::IceStorm => AbilityData { id: AbilityType::IceStorm, cooldown: 5.0, magnitude: 1.0, speed: 25.0 },
            AbilityType::HealOrb => AbilityData { id: AbilityType::HealOrb, cooldown: 10.0, magnitude: 10.0, speed: 0.0 }
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
        app.add_systems(Update, (update_abilities, cast_ability));
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
            texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("abilities/ice_storm.png"), Vec2::splat(64.0), 5, 1, None, None)),
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
        use_ability(ability, transform, rotation, commands, ability_sprites);
    }

}

fn use_ability(ability: &mut Ability, origin: &Transform, rotation: Quat, mut commands: Commands, mut ability_sprites: ResMut<AbilityBundle>) {
    ability.cooldown_timer.set_duration(Duration::from_secs_f32(ability.ability_data.cooldown));
    ability.cooldown_timer.reset();
    if let Some(mut ability_sprite) = ability_sprites.sprites.get_mut(&ability.ability_data.id).cloned() {
        let (_, _, angle) = rotation.to_euler(EulerRot::XYZ);
        ability_sprite.transform.translation = origin.translation + rotation.mul_vec3(Vec3::new(1.0, 0.0, 0.0)) * 64.0;
        match ability.ability_data.id {
            AbilityType::FireBall => {
                let (mut ability_instance , damage, animator, grav, rb, constraints, coll, sensor , vel) = (
                    ability_sprite, 
                    Damage { damage_amount: ability.ability_data.magnitude }, 
                    LoopingAnimator::new(4, 0.2),
                    GravityScale(0.0),
                    RigidBody::Dynamic,
                    LockedAxes::ROTATION_LOCKED,
                    Collider::ball(32.0),
                    Sensor,
                    Velocity { linvel: Vec2::from_angle(angle) * ability.ability_data.speed, angvel: 0.0 }
                ); 
                ability_instance.transform.rotation = rotation;
                commands.spawn((ability_instance , damage, animator, grav, rb, constraints, coll, sensor, vel));
            },
            AbilityType::IceStorm => {
                let (mut ability_instance, damage_over_time , slow, animator, grav, rb, constraints, coll, sensor, vel) = (
                    ability_sprite, 
                    DamageOverTime { tick_damage: ability.ability_data.magnitude }, 
                    Slow { speed_reduction: ability.ability_data.magnitude * 10.0 },
                    LoopingAnimator::new(4, 0.2),
                    GravityScale(0.0),
                    RigidBody::KinematicVelocityBased,
                    LockedAxes::ROTATION_LOCKED,
                    Collider::ball(64.0),
                    Sensor,
                    Velocity { linvel: Vec2::from_angle(angle) * ability.ability_data.speed, angvel: std::f32::consts::FRAC_PI_4 },
                ); 
                ability_instance.transform.rotation = rotation;
                commands.spawn((ability_instance, damage_over_time , slow, animator, grav, rb, constraints, coll, sensor, vel));
            },
            AbilityType::HealOrb => {
                let (mut ability_instance , heal, grav, rb, constraints, coll, sensor, vel) = (
                    ability_sprite, 
                    Heal { heal_amount: ability.ability_data.magnitude }, 
                    GravityScale(0.0),
                    RigidBody::Dynamic,
                    LockedAxes::ROTATION_LOCKED,
                    Collider::ball(32.0),
                    Sensor,
                    Velocity { linvel: Vec2::ZERO, angvel: 0.0 }
                );
                ability_instance.transform.rotation = Quat::IDENTITY;
                commands.spawn((ability_instance , heal, grav, rb, constraints, coll, sensor, vel));
            }
        };
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
    pub speed_reduction: f32
}

#[derive(Component)]
pub struct Damage {
    pub damage_amount: f32
}

#[derive(Component)]
pub struct DamageOverTime {
    pub tick_damage: f32
}

#[derive(Component)]
pub struct Stun {
    pub stun_duration: f32
}
