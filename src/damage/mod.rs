pub fn multiplier_from_defence(defence: i32) -> f32 {
    if defence > 0 {
        return 100.0 / 100.0 + defence as f32;
    } else {
        return 2.0 - (100.0 / (100.0 - defence as f32));
    }
}
pub mod health;
pub mod damagetype;
