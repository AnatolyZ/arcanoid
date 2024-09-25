use super::components::Platform;
use crate::ball::components::Ball;
use crate::ball::NewBallOnPlatform;
use crate::play_area::MainCamera;
use crate::textures::{resources::Textures, HALF_TILE_SIZE, TILE_SIZE};
use crate::SCREEN_HEIGHT;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

const PLATFORM_DEFAULT_LENGTH: u32 = 10;
const PLATFORM_FIRST_TILE: usize = 4;
const PLATFORM_MIDDLE_TILE: usize = 5;
const PLATFORM_LAST_TILE: usize = 6;
const BALL_SPRITE_RADIUS: f32 = 16.0;

pub fn spawn_platform(
    mut commands: Commands,
    textures: Res<Textures>,
    asset_server: Res<AssetServer>,
) {
    let platform_entity = commands
        .spawn((
            Platform {
                collision_sound: asset_server.load("sounds/platform.ogg"),
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
            Collider::cuboid(
                HALF_TILE_SIZE * PLATFORM_DEFAULT_LENGTH as f32,
                HALF_TILE_SIZE,
            ),
            GlobalTransform::default(),
            Transform::from_xyz(0.0, -SCREEN_HEIGHT / 2.0 + TILE_SIZE, 1.0),
        ))
        .with_children(|children| {
            children.spawn((
                SpriteBundle {
                    texture: textures.industrial.texture.clone(),
                    transform: Transform::from_xyz(
                        -(PLATFORM_DEFAULT_LENGTH as f32) * TILE_SIZE / 2.0 + HALF_TILE_SIZE,
                        0.0,
                        0.0,
                    ),
                    ..Default::default()
                },
                TextureAtlas {
                    layout: textures.industrial.layout.clone(),
                    index: PLATFORM_FIRST_TILE,
                },
            ));
        })
        .id();
    for i in 1..PLATFORM_DEFAULT_LENGTH - 1 {
        let middle_tile_entity = commands
            .spawn((
                SpriteBundle {
                    texture: textures.industrial.texture.clone(),
                    transform: Transform::from_xyz(
                        TILE_SIZE * i as f32 - PLATFORM_DEFAULT_LENGTH as f32 * TILE_SIZE / 2.0
                            + HALF_TILE_SIZE,
                        0.0,
                        0.0,
                    ),
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
                    PLATFORM_DEFAULT_LENGTH as f32 * TILE_SIZE / 2.0 - HALF_TILE_SIZE,
                    0.0,
                    0.0,
                ),
                ..Default::default()
            },
            TextureAtlas {
                layout: textures.industrial.layout.clone(),
                index: PLATFORM_LAST_TILE,
            },
        ))
        .id();
    commands.entity(platform_entity).add_child(last_tile_entity);
    spawn_ball_on_platform(commands, textures, platform_entity);
}

pub fn spawn_ball_on_platform(
    mut commands: Commands,
    textures: Res<Textures>,
    platform_entity: Entity,
) {
    let ball_entity = commands
        .spawn((
            Ball {
                radius: BALL_SPRITE_RADIUS,
                lay_on_platform: true,
            },
            SpriteBundle {
                texture: textures.ball.clone(),
                transform: Transform::from_xyz(0.0, BALL_SPRITE_RADIUS + HALF_TILE_SIZE, 3.0),
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
            ActiveEvents::COLLISION_EVENTS,
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
        &GlobalTransform
    ), With<Platform>>,
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

    for (mut binding, mut velocity, transform) in query.iter_mut() {
        let force = binding.as_mut();
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            force.impulse = Vec2::new(-30000.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            force.impulse = Vec2::new(30000.0, 0.0);
        }

        if let Some(cursor_position) = cursor_position {
            let diff = cursor_position.x - transform.translation().x;
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
    platform_query: Query<Entity, With<Platform>>,
    mut new_ball_events: EventReader<NewBallOnPlatform>,
) {
    if new_ball_events.read().count() > 0 {
        let platform_entity = platform_query.single();
        spawn_ball_on_platform(commands, textures, platform_entity);
    }
}

pub fn collision_handler(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    platform_query: Query<(Entity, &Platform)>,
    ball_query: Query<(Entity, &Ball)>,

) {
    for ev in collisions.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = ev {
            if platform_query.get(*entity1).is_ok() && ball_query.get(*entity2).is_ok() {
                if let Ok((_, ball)) = ball_query.get(*entity2) {
                    if ball.lay_on_platform{
                        continue;
                    }
                if let Ok((_, platform)) = platform_query.get(*entity1) {
                    commands.spawn(AudioBundle{
                        source: platform.collision_sound.clone(),
                        settings: PlaybackSettings{
                            mode:bevy::audio::PlaybackMode::Despawn,
                            ..Default::default()
                        }
                    });
                }
            }
            if platform_query.get(*entity2).is_ok() && ball_query.get(*entity1).is_ok() {
                if let Ok((_, ball)) = ball_query.get(*entity1) {
                    if ball.lay_on_platform{
                        continue;
                    }
                if let Ok((_, platform)) = platform_query.get(*entity2) {
                    commands.spawn(AudioBundle{
                        source: platform.collision_sound.clone(),
                        settings: PlaybackSettings{
                            mode:bevy::audio::PlaybackMode::Despawn,
                            ..Default::default()
                        }
                    });
                }
            }
        }
        }
    }}
}
