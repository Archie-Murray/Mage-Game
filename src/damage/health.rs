use bevy::prelude::*;
use crate::damage::damagetype::DamageType;
use crate::damage;
#[derive(Component)]
pub struct Health {
    currentHealth: f32,
    maxHealth: f32,
    magicalDefence: i32,
    physicalDefence: i32,
    dead: bool,
    isInvulnerable: bool
}

impl Health {
    pub fn new(health: f32, physicalDefence: i32, magicalDefence: i32) -> Health {
        return Self { currentHealth: health, maxHealth: health, magicalDefence, physicalDefence, dead: false, isInvulnerable: false };
    }

    pub fn damage(&mut self, amount: f32, damageType: DamageType) {
        if !self.dead && !self.isInvulnerable {
            self.currentHealth = f32::max(0.0, self.currentHealth - amount * self.defence_multiplier(damageType));
        }
        if self.currentHealth == 0.0 {
            self.dead = true;
        }
    }

    fn defence_multiplier(&mut self, damageType: DamageType) -> f32 {
        return match damageType {
            DamageType::PHYSICAL => damage::multiplier_from_defence(self.physicalDefence),
            DamageType::MAGICAL => damage::multiplier_from_defence(self.magicalDefence),
            _ => 1.0
        };
    }
}
