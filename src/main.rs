use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::winit::WinitWindows;
use bevy_hanabi::prelude::*;
use winit::window::Icon;
mod player;
mod damage;
mod animation;
mod abilities;
mod entity;
mod input;

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
        .add_plugins(HanabiPlugin)
        .add_plugins(RapierDebugRenderPlugin::default())
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
        app.add_systems(Startup, (spawn_camera, set_icon));
        app.add_systems(Update, toggle_rapier_debug);
    }
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    let camera: Camera2dBundle = Camera2dBundle { ..default() };
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
