use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::winit::WinitWindows;
use bevy_hanabi::prelude::*;
use winit::window::Icon;
use bevy_ecs_ldtk::prelude::*;
mod player;
mod damage;
mod animation;
mod abilities;
mod entity;
mod input;
mod map;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(
            ImagePlugin::default_nearest()
                )
                .set( 
            WindowPlugin {
                        primary_window: Some(Window {
                            title: "Mage Game".into(),
                            resolution: (1920.0, 1080.0).into(),
                            prevent_default_event_handling: false,
                            .. default()
                        }),
                        ..default()
                }
            ),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .insert_resource(RapierConfiguration { gravity: Vec2::ZERO, ..default() })
        .add_plugins(HanabiPlugin)
        .add_plugins(RapierDebugRenderPlugin { enabled: false, ..Default::default() })
        .add_plugins(LdtkPlugin)
        .register_ldtk_int_cell::<map::WallBundle>(1)
        .register_ldtk_int_cell::<map::VoidBundle>(2)
        .insert_resource(LevelSelection::index(0))
        .add_plugins(abilities::ability_particles::ParticlePlugin)
        .add_plugins(entity::EntityPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .register_type::<abilities::abilities::AbilitySystem>()
        .register_type::<abilities::abilities::AutoDestroy>()
        .add_plugins(damage::health::HealthPlugin)
        .add_plugins(player::playerplugin::PlayerPlugin)
        .add_plugins(abilities::abilities::AbilitySystemPlugin)
        .add_plugins(animation::AnimatorPlugin)
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, set_icon, spawn_tilemap.before(player::playerplugin::spawn_player)));
        app.add_systems(Update, (toggle_rapier_debug, map::spawn_wall_collision, map::spawn_void_collision, map::void_collisions));
        app.add_systems(Update, camera_follow.after(player::playerplugin::player_move_input));
    }
}

fn camera_follow(
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<player::Player>)>,
    player_query: Query<&Transform, (With<player::Player>, Without<MainCamera>)>
) {
    let Ok(mut camera) = camera_query.get_single_mut() else {return;};
    let Ok(player) = player_query.get_single() else {return;};
    if (player.translation.truncate() - camera.translation.truncate()).length_squared() < 100.0 { return; }
    let lerp = camera.translation.truncate().lerp(player.translation.truncate(), time.delta_seconds());
    camera.translation = lerp.extend(camera.translation.z);
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map = LdtkWorldBundle {
        ldtk_handle: asset_server.load("environment/main.ldtk"),
        transform: Transform::from_xyz(-768.0, -768.0, -10.0),
        ..default()
    };
    commands.spawn(map);
}

fn spawn_camera(mut commands: Commands) {
    let camera: Camera2dBundle = Camera2dBundle { projection: OrthographicProjection { scale: 1.0 / 3.0, near: -100.0, far: 100.0, ..default() }, ..default() };
    commands.spawn((camera, MainCamera));
}

fn set_icon(windows: NonSend<WinitWindows>) {
    let image = image::open("assets/logo.png")
        .expect("Failed to open logo path")
        .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    let icon = Icon::from_rgba(rgba, width, height).unwrap();
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}

pub fn toggle_rapier_debug(
    input: Res<Input<KeyCode>>,
    mut render_context: ResMut<DebugRenderContext>
) {
    if input.just_pressed(KeyCode::Escape) {
        println!("Toggled render context");
        render_context.enabled = !render_context.enabled;
    }
}
