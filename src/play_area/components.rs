use bevy::prelude::*;

#[derive(Component)]
pub struct Animation {
    pub phase: usize,
    pub sprites: Vec<usize>,
}

#[derive(Component)]
pub struct OutSensor;
