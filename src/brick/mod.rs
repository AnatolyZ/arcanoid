use bevy::prelude::*;

mod components;
mod systems;
pub struct BricksPlugin;

impl Plugin for BricksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_bricks_array)
            .add_systems(Update, systems::collision_handler);
    }
}
