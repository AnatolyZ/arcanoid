use super::components::Platform;
use crate::textures::{resources::Textures, HALF_TILE_SIZE, TILE_SIZE};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn spawn_platform(mut commands: Commands, windows: Query<&Window>, textures: Res<Textures>) {
    let window = windows.single();

    commands
        .spawn((
            Platform(),
            SpriteSheetBundle {
                texture_atlas: textures.industrial.clone(),
                sprite: TextureAtlasSprite::new(4),
                transform: Transform::from_xyz(0.0, -window.height() / 2.0 + TILE_SIZE, 0.0),
                ..Default::default()
            },
            RigidBody::Dynamic,
            Collider::cuboid(HALF_TILE_SIZE, HALF_TILE_SIZE - 3.0),
            ExternalImpulse {
                impulse: Vec2::new(0.0, 0.0),
                torque_impulse: 0.0,
            },
            Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            },
            LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y,
            Ccd::enabled(),
            Restitution::new(0.2),
        ))
        .with_children(|children| {
            children.spawn((
                SpriteSheetBundle {
                    texture_atlas: textures.industrial.clone(),
                    sprite: TextureAtlasSprite::new(5),
                    transform: Transform::from_xyz(TILE_SIZE, 0.0, 0.0),
                    ..Default::default()
                },
                Collider::cuboid(HALF_TILE_SIZE, HALF_TILE_SIZE - 3.0),
            ));
            children.spawn((
                SpriteSheetBundle {
                    texture_atlas: textures.industrial.clone(),
                    sprite: TextureAtlasSprite::new(5),
                    transform: Transform::from_xyz(TILE_SIZE * 2.0, 0.0, 0.0),
                    ..Default::default()
                },
                Collider::cuboid(HALF_TILE_SIZE, HALF_TILE_SIZE - 3.0),
            ));
            children.spawn((
                SpriteSheetBundle {
                    texture_atlas: textures.industrial.clone(),
                    sprite: TextureAtlasSprite::new(6),
                    transform: Transform::from_xyz(TILE_SIZE * 3.0, 0.0, 0.0),
                    ..Default::default()
                },
                Collider::cuboid(HALF_TILE_SIZE, HALF_TILE_SIZE - 3.0),
                Restitution::new(0.2),
            ));
        });
}

pub fn move_platform(
    mut query: Query<&mut ExternalImpulse, With<Platform>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for mut binding in query.iter_mut() {
        let force = binding.as_mut();
        if keyboard_input.pressed(KeyCode::Left) {
            force.impulse = Vec2::new(-3.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) {
            force.impulse = Vec2::new(3.0, 0.0);
        }
    }
}
