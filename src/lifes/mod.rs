mod components;
mod events;
mod resources;
mod systems;

use crate::states::GameState;
use bevy::prelude::*;
pub use events::LifeLost;

pub struct LifesPlugin {
    initial_lifes: u32,
}

impl LifesPlugin {
    pub fn new(initial_lifes: u32) -> Self {
        Self { initial_lifes }
    }
}

impl Plugin for LifesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(resources::LifesSettings {
            initial_lifes: self.initial_lifes,
        })
        .add_event::<LifeLost>()
        .add_systems(OnExit(GameState::Menu), systems::spawn_lifes)
        .add_systems(OnExit(GameState::Menu), systems::despawn_lifes)
        .add_systems(OnExit(GameState::OverOver), systems::reset_lifes)
        .add_systems(
            Update,
            (systems::redraw_lifes, systems::update_lifes_count).run_if(in_state(GameState::Game)),
        );
    }
}
