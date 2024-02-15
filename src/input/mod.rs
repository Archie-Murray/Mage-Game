use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource, Default)]
pub struct Mouse {
    pub position: Vec2,
    pub world_position: Vec2
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Mouse>()
            .add_systems(Update, update_mouse_pos);
    }
}

pub fn update_mouse_pos(
    q: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<crate::MainCamera>>,
    mut mouse: ResMut<Mouse>
) {
    let (camera, transform) = camera.single();
    if let Ok(current_window) = q.get_single() {
        mouse.position = current_window.cursor_position().unwrap_or(mouse.position);
        mouse.world_position = camera.viewport_to_world_2d(transform, mouse.position).unwrap_or(mouse.world_position);
    }
}
