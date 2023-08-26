use bevy::prelude::*;

#[derive(Component)]
pub struct Platform {
    pub length: u32, // platform length in tiles (minumum 2 tiles)
}
