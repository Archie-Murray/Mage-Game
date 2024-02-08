
use crate::{damage::health::{Health, EntityType}, entity::{stats::Stats, stat_type::StatType}};
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use crate::animation::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn player_move_input(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &Stats), With<Player>>,
) {
    let input: Vec2 = Vec2::new(
        (keyboard_input.pressed(KeyCode::D) as i32 - keyboard_input.pressed(KeyCode::A) as i32)
            as f32,
        (keyboard_input.pressed(KeyCode::W) as i32 - keyboard_input.pressed(KeyCode::S) as i32)
            as f32    
    );
    println!("Player input: {input}");
    let (mut velocity, stats) = query.single_mut();
    velocity.linvel = time.delta_seconds() * *stats.get_stat(StatType::Speed).unwrap_or(&100.0) * input;
}

pub fn spawn_player(mut commands: Commands, assets: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let texture_handle: Handle<Image> = assets.load("player/player.png");
    let texture_atlas: TextureAtlas = TextureAtlas::from_grid(texture_handle, Vec2::new(48.0, 64.0), 3, 4, None, None);
    commands.spawn((
        Player,
        Health::new(100.0, 10, 10, EntityType::Player),
        Animations { 
            animation_indices: HashMap::from([
                (AnimationType::Idle, HashMap::from([
                    (AnimationDirection::Up, AnimationIndices { first: 0, last: 0 }),
                    (AnimationDirection::Down, AnimationIndices { first: 3, last: 3 }),
                    (AnimationDirection::Left, AnimationIndices { first: 6, last: 6 }),
                    (AnimationDirection::Right, AnimationIndices { first: 9, last: 9 })
                ])),
                (AnimationType::Walk, HashMap::from([
                    (AnimationDirection::Up, AnimationIndices { first: 0, last: 2 }),
                    (AnimationDirection::Down, AnimationIndices { first: 3, last: 5 }),
                    (AnimationDirection::Left, AnimationIndices { first: 6, last: 8 }),
                    (AnimationDirection::Right, AnimationIndices { first: 9, last: 11 })
                ]))
            ]), 
            current: AnimationType::Idle 
        },
        SpriteSheetBundle {
            texture_atlas: texture_atlases.add(texture_atlas),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        AnimationTimer {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            is_animating: true
        },
        RigidBody::Dynamic,
        GravityScale(0.0),
        Velocity {
            linvel: Vec2::ZERO,
            angvel: 0.0
        },
        Collider::ball(0.5),
        Stats::default()
    ));
}

pub fn animate_player(
    time: Res<Time>,
    mut player_query: Query<(&Animations, &Velocity, &mut AnimationTimer, &mut TextureAtlasSprite), With<Player>>
) {
    for (animation, velocity, mut anim_timer, mut sprite) in &mut player_query {
        if !anim_timer.is_animating {
            return;
        }
        anim_timer.timer.tick(time.delta());
        if anim_timer.timer.just_finished() {
            let indices = animation.animation_indices.get(&animation.current).unwrap().get(&vec2_to_direction(&velocity.linvel)).unwrap();
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                (sprite.index + 1).min(indices.last)
            };
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
