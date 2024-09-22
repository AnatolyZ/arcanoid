mod components;
mod systems;
mod resources;

use bevy::prelude::*;
use crate::states::GameState;

pub struct LifesPlugin {
    initial_lifes: u32,
}

impl LifesPlugin {
    pub fn new(initial_lifes: u32) -> Self {
        Self {
            initial_lifes,
        }
    }
}

impl Plugin for LifesPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(resources::LifesSettings{initial_lifes: self.initial_lifes})
            .add_systems(OnExit(GameState::Menu), systems::spawn_lifes)
            .add_systems(OnExit(GameState::Menu), systems::despawn_lifes);
    }
}