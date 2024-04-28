use bevy::prelude::Reflect;

#[derive(Clone, Copy, Reflect, Debug)]
pub enum DamageType { PHYSICAL, MAGICAL, BYPASS }

pub fn multiplier_from_defence(defence: i32) -> f32 {
    if defence > 0 {
        return 100.0 / 100.0 + defence as f32;
    } else {
        return 2.0 - (100.0 / (100.0 - defence as f32));
    }
}
