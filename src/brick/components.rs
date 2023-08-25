use bevy::prelude::*;

#[derive(Component)]
pub struct Brick {
    pub resistance: u8,
}

#[derive(Component)]
pub struct Health(pub i32); // Health of the brick in percentage
