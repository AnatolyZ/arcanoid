use super::components::Brick;
use crate::textures::{resources::Textures, HALF_TILE_SIZE, TILE_SIZE};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn spawn_brick(
    commands: &mut Commands,
    coordinates: Vec2,
    texture: Handle<TextureAtlas>,
    resistance: u8,
) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture,
            transform: Transform::from_xyz(coordinates.x, coordinates.y, 0.0),
            sprite: TextureAtlasSprite::new(0), // always spawn a brick without a crack
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(HALF_TILE_SIZE * 3.0, HALF_TILE_SIZE * 3.0),
        Brick { resistance },
    ));
}

pub fn spawn_bricks_array(mut commands: Commands, textures: ResMut<Textures>) {
    for i in 0..3 {
        for j in 0..3 {
            spawn_brick(
                &mut commands,
                Vec2::new(i as f32 * TILE_SIZE * 3.0, j as f32 * TILE_SIZE * 3.0),
                textures.sand.clone(),
                1,
            );
        }
    }
}
