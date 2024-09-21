use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Animation {
    pub phase: usize,
    pub sprites: Vec<usize>,
}

#[derive(Component)]
pub struct OutSensor;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Border;

#[derive(Component)]
pub struct LdtkWorld;
