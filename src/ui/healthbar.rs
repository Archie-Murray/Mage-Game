use bevy::prelude::*;

use crate::entity::health::Health;

const LERP_SPEED: f32 = 0.5;

#[derive(Component, Default, Reflect)]
pub struct HealthBar {
    pub target_value: f32,
}

#[derive(Component)]
pub struct HealthBarSprite;

#[derive(Bundle, Default)]
pub struct HealthBarBundle {
    pub health_bar: HealthBar,
    pub sprite_bundle: SpriteBundle,
}

impl HealthBarBundle {
    pub fn new(initial_value: f32, texture: Handle<Image>, offset: Vec2) -> Self {
        Self {
            health_bar: HealthBar {
                target_value: initial_value,
                ..default()
            },
            sprite_bundle: SpriteBundle {
                texture,
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::TopCenter,
                    ..Default::default()
                },
                transform: Transform::from_xyz(offset.x, offset.y, 0.0),
                ..default()
            },
        }
    }
}

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_health_bar, update_health_bar_value));
    }
}

pub fn update_health_bar_value(
    time: Res<Time>,
    mut health_bar_entities: Query<(&mut HealthBar, &Parent)>,
    health_entities: Query<&Health>,
) {
    for (mut health_bar,  parent) in health_bar_entities.iter_mut() {
        let Ok(health) = health_entities.get(parent.get()) else { info!("No health in parent"); continue; };
        health_bar.target_value = move_toward(health_bar.target_value, health.get_percent(), LERP_SPEED * time.delta_seconds());
    }
}

pub fn update_health_bar(
    assets: Res<Assets<Image>>,
    mut health_bars: Query<(&mut Sprite, &HealthBar, &Handle<Image>), Changed<HealthBar>>,
) {
    for (mut sprite, health_bar, image_handle) in health_bars.iter_mut() {
        let Some(image) = assets.get(image_handle) else { continue; };
        let rect = Vec2::new(image.width() as f32 * health_bar.target_value, image.height() as f32);
        sprite.rect = Some(Rect {
            min: Vec2::ZERO,
            max: rect,
        });
        sprite.custom_size = Some(rect);
    }
}

pub fn move_toward(current: f32, target: f32, max_delta: f32) -> f32 {
    if (target - current).abs() <= max_delta {
        return target;
    }
    target + (target - current) * max_delta
}
