use super::components::Platform;
use crate::ball::components::Ball;
use crate::ball::NewBallOnPlatform;
use crate::play_area::MainCamera;
use crate::textures::{resources::Textures, HALF_TILE_SIZE, TILE_SIZE};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

const PLATFORM_DEFAULT_LENGTH: u32 = 10;
const PLATFORM_FIRST_TILE: usize = 4;
const PLATFORM_MIDDLE_TILE: usize = 5;
const PLATFORM_LAST_TILE: usize = 6;
const BALL_SPRITE_RADIUS: f32 = 16.0;

fn get_plarform_center_x_offset(platform_length: u32) -> f32 {
    platform_length as f32 * TILE_SIZE / 2.0 - HALF_TILE_SIZE
}

pub fn spawn_platform(mut commands: Commands, windows: Query<&Window>, textures: Res<Textures>) {
    let window = windows.single();

    let center_offset = get_plarform_center_x_offset(PLATFORM_DEFAULT_LENGTH) * -1f32;
    let platform_entity = commands
        .spawn((
            Platform {
                length: PLATFORM_DEFAULT_LENGTH,
            },
            SpriteBundle {
                texture: textures.industrial.texture.clone(),
                transform: Transform::from_xyz(
                    center_offset,
                    -window.height() / 2.0 + TILE_SIZE,
                    1.0,
                ),
                ..Default::default()
            },
            TextureAtlas {
                layout: textures.industrial.layout.clone(),
                index: PLATFORM_FIRST_TILE,
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
            .spawn((
                SpriteBundle {
                    texture: textures.industrial.texture.clone(),
                    transform: Transform::from_xyz(TILE_SIZE * i as f32, 0.0, 0.0),
                    ..Default::default()
                },
                TextureAtlas {
                    layout: textures.industrial.layout.clone(),
                    index: PLATFORM_MIDDLE_TILE,
                },
            ))
            .id();
        commands
            .entity(platform_entity)
            .add_child(middle_tile_entity);
    }
    let last_tile_entity = commands
        .spawn((
            SpriteBundle {
                texture: textures.industrial.texture.clone(),
                transform: Transform::from_xyz(
                    TILE_SIZE * (PLATFORM_DEFAULT_LENGTH - 1) as f32,
                    0.0,
                    0.0,
                ),
                ..Default::default()
            },
            TextureAtlas {
                layout: textures.industrial.layout.clone(),
                index: PLATFORM_LAST_TILE,
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
    mut query: Query<(
        &mut ExternalImpulse,
        &mut Velocity,
        &GlobalTransform,
        &Platform,
    )>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = window.single();
    let (camera, camera_transform) = camera_query.single();
    let cursor_position = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate());

    for (mut binding, mut velocity, transform, platform) in query.iter_mut() {
        let force = binding.as_mut();
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            force.impulse = Vec2::new(-30000.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            force.impulse = Vec2::new(30000.0, 0.0);
        }

        if let Some(cursor_position) = cursor_position {
            let diff = cursor_position.x
                - transform.translation().x
                - get_plarform_center_x_offset(platform.length);
            velocity.linvel = Vec2::new(diff * 10.0, 0.0);
        }
    }
}

pub fn despawn_platform(mut commands: Commands, platform_query: Query<Entity, With<Platform>>) {
    for entity in platform_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn new_ball_check(
    commands: Commands,
    textures: Res<Textures>,
    platform_query: Query<(Entity, &Platform)>,
    mut new_ball_events: EventReader<NewBallOnPlatform>,
) {
    if new_ball_events.read().count() > 0 {
        let (platform_entity, platform) = platform_query.single();
        spawn_ball_on_platform(commands, textures, platform_entity, platform.length);
    }
}
