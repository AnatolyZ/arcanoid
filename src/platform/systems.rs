use super::components::Platform;
use crate::ball::components::Ball;
use crate::textures::{resources::Textures, HALF_TILE_SIZE, TILE_SIZE};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PLATFORM_DEFAULT_LENGTH: u32 = 8;
const PLATFORM_FIRST_TILE: usize = 4;
const PLATFORM_MIDDLE_TILE: usize = 5;
const PLATFORM_LAST_TILE: usize = 6;
const BALL_SPRITE_RADIUS: f32 = 16.0;

pub fn spawn_platform(mut commands: Commands, windows: Query<&Window>, textures: Res<Textures>) {
    let window = windows.single();

    let platform_entity = commands
        .spawn((
            Platform {
                length: PLATFORM_DEFAULT_LENGTH,
            },
            SpriteSheetBundle {
                texture_atlas: textures.industrial.clone(),
                sprite: TextureAtlasSprite::new(PLATFORM_FIRST_TILE),
                transform: Transform::from_xyz(0.0, -window.height() / 2.0 + TILE_SIZE, 1.0),
                ..Default::default()
            },
            RigidBody::Dynamic,
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
                Collider::cuboid(
                    HALF_TILE_SIZE * PLATFORM_DEFAULT_LENGTH as f32,
                    HALF_TILE_SIZE - 3.0,
                ),
                Transform::from_xyz(
                    PLATFORM_DEFAULT_LENGTH as f32 * TILE_SIZE / 2.0 - HALF_TILE_SIZE,
                    0.0,
                    0.0,
                ),
            ));
        })
        .id();
    for i in 1..PLATFORM_DEFAULT_LENGTH - 1 {
        let middle_tile_entity = commands
            .spawn((SpriteSheetBundle {
                texture_atlas: textures.industrial.clone(),
                sprite: TextureAtlasSprite::new(PLATFORM_MIDDLE_TILE),
                transform: Transform::from_xyz(TILE_SIZE * i as f32, 0.0, 0.0),
                ..Default::default()
            },))
            .id();
        commands
            .entity(platform_entity)
            .add_child(middle_tile_entity);
    }
    let last_tile_entity = commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: textures.industrial.clone(),
                sprite: TextureAtlasSprite::new(PLATFORM_LAST_TILE),
                transform: Transform::from_xyz(
                    TILE_SIZE * (PLATFORM_DEFAULT_LENGTH - 1) as f32,
                    0.0,
                    0.0,
                ),
                ..Default::default()
            },
            Restitution::new(0.2),
        ))
        .id();
    commands.entity(platform_entity).add_child(last_tile_entity);
    spawn_ball_on_platform(commands, textures, platform_entity, PLATFORM_DEFAULT_LENGTH);
}

pub fn spawn_ball_on_platform(
    mut commands: Commands,
    textures: Res<Textures>,
    platform_entity: Entity,
    platform_length: u32,
) {
    let ball_entity = commands
        .spawn((
            Ball {
                radius: BALL_SPRITE_RADIUS,
                lay_on_platform: true,
            },
            SpriteBundle {
                texture: textures.ball.clone(),
                transform: Transform::from_xyz(
                    platform_length as f32 * TILE_SIZE / 2.0 - HALF_TILE_SIZE,
                    BALL_SPRITE_RADIUS + HALF_TILE_SIZE,
                    3.0,
                ),
                ..Default::default()
            },
            RigidBody::Dynamic,
            Collider::ball(BALL_SPRITE_RADIUS),
            Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            },
            ExternalImpulse {
                impulse: Vec2::new(0.0, 0.0),
                torque_impulse: 0.0,
            },
            Ccd::enabled(),
            Restitution::new(2.0),
        ))
        .id();
    commands.entity(platform_entity).add_child(ball_entity);
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

pub fn despawn_platform(mut commands: Commands, platform_query: Query<Entity, With<Platform>>) {
    for entity in platform_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
