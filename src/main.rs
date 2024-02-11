use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use player::Player;
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
        app.add_systems(Last, update_camera_pos);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera: Camera2dBundle = Camera2dBundle { ..default() };
    commands.spawn(camera);
}

fn update_camera_pos(
    time: Res<Time>,
    mut query: Query<((&mut Transform, With<Camera2d>), (&Transform, &Velocity, With<Player>))>
) {
    let (camera, player) = query.single_mut();
    camera.0.translation.lerp(player.0.translation + Vec3::new(player.1.linvel.x, player.1.linvel.y, camera.0.translation.z), 10.0 * time.delta_seconds());
}

fn setup_phyiscs(mut commands: Commands) {
    let floor = (
        Collider::cuboid(50.0, 50.0),
        RigidBody::Dynamic, 
        Transform::from_xyz(0.0, 0.0, 0.0)
    );
    commands.spawn(floor);
}
