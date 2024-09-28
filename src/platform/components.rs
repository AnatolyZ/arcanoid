use bevy::prelude::*;

#[derive(Component)]
pub struct Platform {
    pub ball_collision_sound: Handle<AudioSource>,
    pub border_collision_sound: Handle<AudioSource>,
}
