use bevy::prelude::*;

#[derive(Component)]
pub struct Ball {
    pub radius: f32,
    pub lay_on_platform: bool,
}
