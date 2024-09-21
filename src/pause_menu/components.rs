use bevy::prelude::*;

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub enum ButtonType {
    Resume,
    MainMenu,
    Exit,
}
