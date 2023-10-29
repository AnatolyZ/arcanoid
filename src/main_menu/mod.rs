mod components;
mod systems;

use crate::states::GameState;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), systems::spawn_main_menu)
            .add_systems(OnExit(GameState::Menu), systems::despawn_main_menu);
    }
}
