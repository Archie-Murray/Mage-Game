
use crate::{animation::AnimationTimer, damage::health::Health};
use bevy::prelude::*;
use crate::animation::*;

#[derive(Component)]
pub struct Player;

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let input: Vec3 = Vec3::new(
        (keyboard_input.pressed(KeyCode::D) as i32 - keyboard_input.pressed(KeyCode::A) as i32)
            as f32,
        (keyboard_input.pressed(KeyCode::W) as i32 - keyboard_input.pressed(KeyCode::S) as i32)
            as f32,
        0.0,
    );
    query.single_mut().translation += input * time.delta_seconds() * 100.0;
}

pub fn spawn_player(mut commands: Commands, assets: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let texture_handle: Handle<Image> = assets.load("Player/idle_src.png");
    let texture_atlas: TextureAtlas = TextureAtlas::from_grid(texture_handle, Vec2::splat(32.0), 8, 8, None, None);
    commands.spawn((
        Player,
        Health::new(100.0, 10, 10),
        Animations { 
            animation_indices: bevy::utils::hashbrown::HashMap::from([
                (AnimationType::Idle, AnimationIndices { first: 0, last: 8 })
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
            timer: Timer::from_seconds(0.1, TimerMode::Repeating)
        }
    ));
}

pub fn animate_player(
    time: Res<Time>,
    mut player_query: Query<
        (&Animations, &mut AnimationTimer, &mut TextureAtlasSprite), 
        With<Player>
    >
) {
    for (animation, mut anim_timer, mut sprite) in &mut player_query {
        anim_timer.timer.tick(time.delta());
        if anim_timer.timer.just_finished() {
            sprite.index = if sprite.index == animation.animation_indices.get(&animation.current).unwrap().last {
                animation.animation_indices.get(&animation.current).unwrap().first
            } else {
                sprite.index + 1
            };
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (move_player, animate_player));
    }
}
