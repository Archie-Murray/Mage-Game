use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
mod player;
mod damage;
mod animation;
mod entity;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(GamePlugin)
        .add_plugins(player::PlayerPlugin)
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, setup_phyiscs));
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera: Camera2dBundle = Camera2dBundle { ..default() };
    commands.spawn(camera);
}

fn setup_phyiscs(mut commands: Commands) {
    let floor = (
        Collider::cuboid(50.0, 50.0),
        RigidBody::Dynamic, 
        Transform::from_xyz(0.0,0.0,0.0)
    );
    commands.spawn(floor);
}
