use bevy::prelude::*;

#[derive(Component)]
pub struct GameOverMenu;

#[derive(Component)]
pub enum ButtonType {
    Restart,
    MainMenu,
    Exit,
}
