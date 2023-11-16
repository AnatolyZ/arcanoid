use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub enum ButtonType {
    Start,
    Exit,
}
