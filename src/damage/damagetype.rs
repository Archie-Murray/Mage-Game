use bevy_inspector_egui::prelude::*;
use bevy::prelude::*;

#[derive(Clone, Copy, Reflect)]
pub enum DamageType { PHYSICAL, MAGICAL, BYPASS }
