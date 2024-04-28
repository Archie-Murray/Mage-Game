pub mod healthbar;
pub mod pause;

pub struct UIPlugin;

use bevy::prelude::Plugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((pause::PausePlugin, healthbar::HealthBarPlugin));
    }
}
