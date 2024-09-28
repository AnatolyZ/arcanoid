use super::components::Platform;
use crate::ball::components::Ball;
use crate::ball::NewBallOnPlatform;
use crate::play_area::Border;
use crate::play_area::MainCamera;
use crate::textures::{resources::Textures, HALF_TILE_SIZE, TILE_SIZE};
use crate::SCREEN_HEIGHT;
use bevy::ecs::query::QueryFilter;
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
                ball_collision_sound: asset_server.load("sounds/ball_platform.ogg"),
                border_collision_sound: asset_server.load("sounds/border_platform.ogg"),
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
    spawn_ball_on_platform(commands, textures, platform_entity, &asset_server);
}

pub fn spawn_ball_on_platform(
    mut commands: Commands,
    textures: Res<Textures>,
    platform_entity: Entity,
    asset_server: &Res<AssetServer>,
) {
    let ball_entity = commands
        .spawn((
            Ball {
                radius: BALL_SPRITE_RADIUS,
                lay_on_platform: true,
                border_collision_sound: asset_server.load("sounds/ball_border.ogg"),
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
    mut query: Query<(&mut ExternalImpulse, &mut Velocity, &GlobalTransform), With<Platform>>,
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
    asset_server: Res<AssetServer>,
) {
    if new_ball_events.read().count() > 0 {
        let platform_entity = platform_query.single();
        spawn_ball_on_platform(commands, textures, platform_entity, &asset_server);
    }
}

fn get_component<'a, C, F>(
    entity1: Entity,
    entity2: Entity,
    query: &'a Query<&C, F>,
) -> Option<&'a C>
where
    C: Component,
    F: QueryFilter,
{
    if let Ok(component) = query.get(entity1) {
        return Some(component);
    }
    if let Ok(component) = query.get(entity2) {
        return Some(component);
    }
    None
}

pub fn collision_handler(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    platform_query: Query<&Platform>,
    ball_query: Query<&Ball>,
    border_query: Query<&Border>,
) {
    for ev in collisions.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = ev {
            let ball = get_component(*entity1, *entity2, &ball_query);
            let platform = get_component(*entity1, *entity2, &platform_query);
            let border = get_component(*entity1, *entity2, &border_query);

            // Ball and Platform collision
            if let (Some(ball), Some(platform)) = (ball, platform) {
                if ball.lay_on_platform {
                    continue;
                }
                commands.spawn(AudioBundle {
                    source: platform.ball_collision_sound.clone(),
                    settings: PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Despawn,
                        ..Default::default()
                    },
                });
            }
            // Ball and Border collision
            if let (Some(ball), Some(_)) = (ball, border) {
                commands.spawn(AudioBundle {
                    source: ball.border_collision_sound.clone(),
                    settings: PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Despawn,
                        ..Default::default()
                    },
                });
            }
            // Platform and Border collision
            if let (Some(platform), Some(_)) = (platform, border) {
                commands.spawn(AudioBundle {
                    source: platform.border_collision_sound.clone(),
                    settings: PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Despawn,
                        ..Default::default()
                    },
                });
            }
        }
    }
}
