pub mod resources;
mod systems;

use bevy::prelude::*;

use crate::states::GameState;

pub const TILE_SIZE: f32 = 18.0;
pub const HALF_TILE_SIZE: f32 = TILE_SIZE / 2.0;
pub const BRICK_TILE_SIZE: f32 = TILE_SIZE * 3.0;
pub const HALF_BRICK_TILE_SIZE: f32 = BRICK_TILE_SIZE / 2.0;

pub struct TexturesPlugin;

impl Plugin for TexturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::LoadTextures), systems::load_textures)
            .add_systems(
                Update,
                systems::transition_to_spawn_camera.run_if(in_state(GameState::LoadTextures)),
            );
    }
}
