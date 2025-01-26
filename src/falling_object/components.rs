use bevy::prelude::*;

#[derive(Component)]
pub enum Buff {
    ExpandPlatform,
}

#[derive(Component)]
pub enum Debuff {
    ShrinkPlatform,
}
