use bevy::prelude::*;

mod components;
pub(crate) mod systems;

pub use components::Platform;
pub struct PlatformPlugin;

use crate::states::GameState;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Setup), systems::spawn_platform)
            .add_systems(
                Update,
                (systems::new_ball_check, systems::move_platform).run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::OverOver), (systems::despawn_platform,))
            .add_systems(OnEnter(GameState::Menu), (systems::despawn_platform,))
            .add_systems(
                OnExit(GameState::LevelComplete),
                (systems::despawn_platform,),
            );
    }
}
