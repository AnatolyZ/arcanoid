mod systems;

use bevy::prelude::*;

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug, States)]
pub enum GameState {
    Menu,
    #[default]
    Game,
    Paused,
    Over,
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::pause_game_control)
            .add_systems(OnEnter(GameState::Paused), systems::on_enter_pause)
            .add_systems(OnExit(GameState::Paused), systems::on_exit_pause);
    }
}
