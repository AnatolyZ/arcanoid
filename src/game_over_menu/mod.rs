mod components;
mod systems;

use crate::states::GameState;
use bevy::prelude::*;

use self::systems::{buttons_press_system, buttons_state_system};

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Over), systems::spawn_game_over_menu)
            .add_systems(OnExit(GameState::Over), systems::despawn_game_over_menu)
            .add_systems(
                Update,
                (buttons_state_system, buttons_press_system).run_if(in_state(GameState::Over)),
            );
    }
}
