use crate::animation::*;
use crate::animation::directional_animator::*;
use crate::{
    damage::health::{EntityType, Health},
    entity::{stat_type::StatType, stats::Stats},
};
use bevy::utils::hashbrown::HashMap;
use bevy_rapier2d::prelude::*;
use crate::player::Player;
use crate::abilities::abilities::AbilitySystem;

use bevy::prelude::*;
pub fn player_move_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &Stats), With<Player>>,
) {
    let input: Vec2 = Vec2::new(
        (keyboard_input.pressed(KeyCode::D) as i32 - keyboard_input.pressed(KeyCode::A) as i32)
            as f32,
        (keyboard_input.pressed(KeyCode::W) as i32 - keyboard_input.pressed(KeyCode::S) as i32)
            as f32,
    );
    let (mut velocity, stats) = query.single_mut();
    velocity.linvel =
        *stats.get_stat(StatType::Speed).unwrap_or(&50.0) * input; //NOTE: Rapier already applies deltaTime multiplication
}

pub fn spawn_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle: Handle<Image> = assets.load("player/player.png");
    let texture_atlas: TextureAtlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(48.0, 64.0), 3, 4, None, None);
    commands.spawn((
        Player,
        Health::new(100.0, 10, 10, EntityType::Player),
        DirectionalAnimator {
            animation_indices: HashMap::from([
                (
                    AnimationType::Idle,
                    HashMap::from([
                        ( AnimationDirection::Up, AnimationIndices::new(0, 0 ),),
                        ( AnimationDirection::Down, AnimationIndices::new(6, 6),),
                        ( AnimationDirection::Left, AnimationIndices::new(9, 9),),
                        ( AnimationDirection::Right, AnimationIndices::new(3, 3),),
                    ]),
                ),
                (
                    AnimationType::Walk,
                    HashMap::from([
                        ( AnimationDirection::Up, AnimationIndices::new(0, 2),),
                        ( AnimationDirection::Down, AnimationIndices::new(6, 8),),
                        ( AnimationDirection::Left, AnimationIndices::new(9, 11),),
                        ( AnimationDirection::Right, AnimationIndices::new(3, 5),),
                    ]),
                ),
            ]),
            animation: AnimationType::Idle,
            direction: AnimationDirection::Up,
            last_update_timer: 0.0
        },
        SpriteSheetBundle {
            texture_atlas: texture_atlases.add(texture_atlas),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(50.0, 0.0, 1.0),
            ..default()
        },
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Velocity {
            linvel: Vec2::ZERO,
            angvel: 0.0,
        },
        Collider::capsule_y(16.0, 12.0),
        Stats::default(),
        AbilitySystem::default(),
    ));
}

pub fn animate_player(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut DirectionalAnimator, With<Player>>,
) {
    let player_input = Vec2::new(
        (if input.get_pressed().find(|key_press| **key_press == KeyCode::D).is_some() { 1.0 } else { 0.0 }) - (if input.get_pressed().find(|key_press| **key_press == KeyCode::A).is_some() { 1.0 } else { 0.0 }),
        (if input.get_pressed().find(|key_press| **key_press == KeyCode::W).is_some() { 1.0 } else { 0.0 }) - (if input.get_pressed().find(|key_press| **key_press == KeyCode::S).is_some() { 1.0 } else { 0.0 })
    );
    let Ok(mut player_animator) = player_query.get_single_mut() else { return; };
    if player_input.length_squared() <= 0.01 {
        player_animator.update_animation(AnimationType::Idle);
    } else {
        player_animator.update_animation(AnimationType::Walk);
        let vel_dir: AnimationDirection = vec2_to_direction(&player_input);
        if vel_dir != player_animator.direction {
            player_animator.update_direction(vel_dir);
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (player_move_input, animate_player));
    }
}
