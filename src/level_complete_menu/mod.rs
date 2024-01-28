mod components;
mod systems;

use crate::states::GameState;
use bevy::prelude::*;

pub struct LevelCompleteMenuPlugin;

impl Plugin for LevelCompleteMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::LevelComplete),
            systems::spawn_level_complete_menu,
        )
        .add_systems(
            OnExit(GameState::LevelComplete),
            systems::despawn_level_complete_menu,
        )
        .add_systems(
            Update,
            (systems::wait_for_key_press).run_if(in_state(GameState::LevelComplete)),
        )
        .add_systems(
            Update,
            (systems::check_for_completion).run_if(in_state(GameState::Game)),
        );
    }
}
