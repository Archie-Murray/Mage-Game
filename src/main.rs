use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::winit::WinitWindows;
use bevy_hanabi::prelude::*;
use winit::window::Icon;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_shader_utils::ShaderUtilsPlugin;
mod animation;
mod abilities;
mod entity;
mod input;
mod map;
mod pathfinding;
mod debug;
mod ui;

use entity::*;

static WORLD_SIZE: IVec2 = IVec2::new(1024, 1024);

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
                            mode: bevy::window::WindowMode::BorderlessFullscreen,
                            prevent_default_event_handling: false,
                            present_mode: bevy::window::PresentMode::AutoVsync,
                            ..default()
                        }),
                        ..default()
                }
            ),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .insert_resource(RapierConfiguration { gravity: Vec2::ZERO, ..default() })
        .add_plugins(HanabiPlugin)
        .add_plugins(map::MapPlugin)
        .add_plugins(ShaderUtilsPlugin)
        .add_plugins(RapierDebugRenderPlugin { enabled: false, ..Default::default() })
        .add_plugins(abilities::ability_particles::ParticlePlugin)
        .add_plugins(ui::UIPlugin)
        .add_plugins(entity::EntityPlugin)
        .add_plugins(enemy::orc::Plugin)
        .register_type::<enemy::Enemy>()
        .add_plugins(pathfinding::PathfindingPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(WorldInspectorPlugin::default())
        .register_type::<abilities::abilities::AbilitySystem>()
        .register_type::<abilities::abilities::AutoDestroy>()
        .register_type::<animation::directional_animator::DirectionalAnimator>()
        .register_type::<ui::healthbar::HealthBar>()
        .register_type::<entity::stats::Stats>()
        .add_plugins(entity::particles::ParticlePlugin)
        .add_plugins(abilities::abilities::AbilitySystemPlugin)
        .add_plugins(animation::AnimatorPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(debug::FPSCounter)
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, set_icon));
        app.register_type::<pathfinding::Grid>();
        app.insert_resource(pathfinding::Grid::default());
        app.add_systems(Update, toggle_debug);
        app.add_systems(Update, camera_follow.after(player::player_move_input));
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

pub fn toggle_debug(
    input: Res<ButtonInput<KeyCode>>,
    mut render_context: ResMut<DebugRenderContext>,
    mut debug: ResMut<crate::debug::Debug>, 
    mut fps_root: Query<&mut Visibility, With<debug::FpsRoot>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        println!("Toggled render context");
        debug.show_debug = !debug.show_debug;
        render_context.enabled = debug.show_debug;
        let mut fps_visibility = fps_root.single_mut();
        *fps_visibility = match *fps_visibility {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden
        };
    }
}
