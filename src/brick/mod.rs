use bevy::prelude::*;

mod components;
mod systems;

pub struct BricksPlugin;

impl Plugin for BricksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (systems::collision_handler, systems::spawn_bricks_ldtk),
        );
    }
}
