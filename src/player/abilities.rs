use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

enum EffectType { Slow, Damage, Heal, Stun }

struct AbilityData {
    pub id: u32,
    pub cooldown: f32,
    pub effect_type: EffectType,
    pub magnitude: f32
}

pub struct AbilitySystem {
    pub ability_bundles: HashMap<u32, SpriteBundle>,
}

#[derive(Resource)]
pub struct AbilityBundle {
    pub sprites: HashMap<u32, SpriteBundle>
}

impl Default for AbilityBundle {
    fn default() -> Self {
        return AbilityBundle { sprites: HashMap::new() };
    }
}

pub struct Ability {
    pub ability_data: AbilityData,
    pub cooldown_timer: Timer,
    pub done: bool
}

pub struct AbilitySystemPlugin;

impl Plugin for AbilitySystemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AbilityBundle>();
        app.add_systems(Startup, init_abilites);
    }
}

fn init_abilites(
    mut abilities: ResMut<AbilityBundle>, asset_server: Res<AssetServer>
) {
    abilities.sprites = HashMap::from([
        (0u32, SpriteBundle { texture: asset_server.load("abilities/fireball.png"), .. default() }),
        (1u32, SpriteBundle { texture: asset_server.load("abilities/ice_storm.png"), .. default() }),
        (2u32, SpriteBundle { texture: asset_server.load("abilities/health_orb.png"), .. default() }),
    ]);
}

impl AbilitySystem {

    pub fn can_use(&self, ability: &Ability) -> bool {
        return ability.cooldown_timer.finished();
    }

    pub fn use_ability(&self, ability: &mut Ability, pos: Vec3, mut commands: Commands, mut abilities: ResMut<AbilityBundle>) {
        ability.cooldown_timer.set_duration(Duration::from_secs_f32(ability.ability_data.cooldown));
        ability.cooldown_timer.reset();
        if let Some(mut ability_sprite) = abilities.sprites.get_mut(&ability.ability_data.id).cloned() {
            ability_sprite.transform.translation = pos;
            match ability.ability_data.effect_type {
                EffectType::Slow => commands.spawn((ability_sprite, Slow { speed_reduction: ability.ability_data.magnitude })),
                EffectType::Damage => commands.spawn((ability_sprite, Damage { damage_amount: ability.ability_data.magnitude })),
                EffectType::Heal => commands.spawn((ability_sprite, Heal { heal_amount: ability.ability_data.magnitude })),
                EffectType::Stun => commands.spawn((ability_sprite, Stun { stun_duration: ability.ability_data.magnitude })),
            };
        }
    }

    pub fn update_ability(&self, ability: &mut Ability, delta_time: f32) {
        ability.cooldown_timer.tick(Duration::from_secs_f32(delta_time));
        if self.can_use(ability) {
            ability.done = true;
        }
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
