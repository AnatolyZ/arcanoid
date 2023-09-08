use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod components;
mod systems;

pub struct BricksPlugin;

impl Plugin for BricksPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell::<components::BrickBundle>(1)
            .register_ldtk_int_cell::<components::BrickBundle>(2)
            .register_ldtk_int_cell::<components::BrickBundle>(3)
            .register_ldtk_int_cell::<components::BrickBundle>(4)
            .add_systems(Update, systems::add_brick_textures)
            .add_systems(Update, systems::collision_handler);
    }
}
