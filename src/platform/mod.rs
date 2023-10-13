use bevy::prelude::*;

mod components;
mod systems;

pub use components::Platform;
pub struct PlatformPlugin;

use crate::states::GameState;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_platform)
            .add_systems(
                Update,
                systems::move_platform.run_if(in_state(GameState::Game)),
            );
    }
}
