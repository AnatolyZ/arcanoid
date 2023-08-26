pub mod resources;
mod systems;

use bevy::prelude::*;

pub const TILE_SIZE: f32 = 18.0;
pub const HALF_TILE_SIZE: f32 = TILE_SIZE / 2.0;
pub const BRICK_TILE_SIZE: f32 = TILE_SIZE * 3.0;
pub const HALF_BRICK_TILE_SIZE: f32 = BRICK_TILE_SIZE / 2.0;

pub struct TexturesPlugin;

impl Plugin for TexturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, systems::load_textures);
    }
}
