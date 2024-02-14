use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod player;
mod damage;
mod animation;
mod entity;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(GamePlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(player::playerplugin::PlayerPlugin)
        .add_plugins(player::abilities::AbilitySystemPlugin)
        .add_plugins(animation::AnimatorPlugin)
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera: Camera2dBundle = Camera2dBundle { ..default() };
    commands.spawn(camera);
}
