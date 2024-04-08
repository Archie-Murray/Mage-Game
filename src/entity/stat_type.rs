use bevy::prelude::Reflect;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Reflect, Debug)]
pub enum StatType { Health, Defence, MagicDefence, Attack, Magic, Speed }
