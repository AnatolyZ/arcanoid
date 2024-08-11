use crate::states::GameState;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod components;
mod systems;

pub use components::Brick;
pub struct BricksPlugin;

impl Plugin for BricksPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell_for_layer::<components::BrickBundle>("Bricks", 1)
            .register_ldtk_int_cell_for_layer::<components::BrickBundle>("Bricks", 2)
            .register_ldtk_int_cell_for_layer::<components::BrickBundle>("Bricks", 3)
            .register_ldtk_int_cell_for_layer::<components::BrickBundle>("Bricks", 4)
            .add_systems(
                Update,
                systems::add_brick_textures.run_if(in_state(GameState::Setup)),
            )
            .add_systems(
                Update,
                systems::collision_handler.run_if(in_state(GameState::Game)),
            );
    }
}
