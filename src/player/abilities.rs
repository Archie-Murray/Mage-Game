use std::time::Duration;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy::utils::hashbrown::HashMap;

use bevy::reflect::*;

use crate::animation::{looping_animator::LoopingAnimator, *};

#[derive(Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub enum EffectType { Slow, Damage, Heal, Stun }

#[derive(Reflect)]
pub struct AbilityData {
    pub id: u32,
    pub cooldown: f32,
    pub effect_type: EffectType,
    pub magnitude: f32
}

#[derive(Component, Reflect)]
pub struct AbilitySystem {
    pub abilities: Vec<Ability>
}

#[derive(Resource)]
pub struct AbilityBundle {
    pub sprites: HashMap<u32, SpriteSheetBundle>
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
    fn new(ability_data: AbilityData) -> Self {
        return Ability { cooldown_timer: Timer::new(Duration::from_secs_f32(ability_data.cooldown), TimerMode::Once), ability_data, done: true };
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
        (0u32, SpriteSheetBundle { 
            texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("abilities/fire_ball.png"), Vec2::splat(32.0), 5, 1, None, None)),
            sprite: TextureAtlasSprite::new(0), 
            transform: Transform::from_scale(Vec3::splat(1.0)), 
            .. default()
        }),
        (1u32, SpriteSheetBundle { 
            texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("abilities/ice_storm.png"), Vec2::splat(32.0), 5, 1, None, None)),
            sprite: TextureAtlasSprite::new(0), 
            transform: Transform::from_scale(Vec3::splat(1.0)),
            .. default()
        }),        
        (2u32, SpriteSheetBundle { 
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

pub fn cast_ability(
    mut commands: Commands,
    mut ability_sprites: ResMut<AbilityBundle>,
    mut query: Query<(&mut AbilitySystem, &Transform)>,
    keyboard: Res<Input<KeyCode>>
) {
    let (mut ability_system, transform) = query.single_mut();
    if keyboard.just_pressed(KeyCode::Q) {
        if let Some(mut ability) = ability_system.abilities.iter_mut().next() {
            if ability.can_use() {
                use_ability(ability, transform.translation, commands, ability_sprites);
            }
        }
    }
}
fn use_ability(ability: &mut Ability, pos: Vec3, mut commands: Commands, mut ability_sprites: ResMut<AbilityBundle>) {
    ability.cooldown_timer.set_duration(Duration::from_secs_f32(ability.ability_data.cooldown));
    ability.cooldown_timer.reset();
    if let Some(mut ability_sprite) = ability_sprites.sprites.get_mut(&ability.ability_data.id).cloned() {
        ability_sprite.transform.translation = pos;
        match ability.ability_data.effect_type {
            EffectType::Slow => commands.spawn((ability_sprite, Slow { speed_reduction: ability.ability_data.magnitude }, LoopingAnimator::new(4, 0.2))),
            EffectType::Damage => commands.spawn((ability_sprite, Damage { damage_amount: ability.ability_data.magnitude }, LoopingAnimator::new(4, 0.2))),
            EffectType::Heal => commands.spawn((ability_sprite, Heal { heal_amount: ability.ability_data.magnitude }, LoopingAnimator::new(4, 0.2))),
            EffectType::Stun => commands.spawn((ability_sprite, Stun { stun_duration: ability.ability_data.magnitude })),
        };
    }
}


impl Default for AbilitySystem {
    fn default() -> Self {
        return AbilitySystem { abilities: vec![Ability::new(AbilityData { id: 0u32, cooldown: 2.0, magnitude: 5.0, effect_type: EffectType::Damage })] };
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
pub struct Stun {
    pub stun_duration: f32
}
