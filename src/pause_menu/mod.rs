mod components;
mod systems;

use crate::states::GameState;
use bevy::prelude::*;

use self::systems::{buttons_press_system, buttons_state_system};

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused), systems::spawn_pause_menu)
            .add_systems(OnExit(GameState::Paused), systems::despawn_pause_menu)
            .add_systems(
                Update,
                (buttons_state_system, buttons_press_system).run_if(in_state(GameState::Paused)),
            );
    }
}
