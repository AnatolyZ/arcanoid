use super::components::{Brick, BrickType};
use crate::ball::Ball;
use crate::states::GameState;
use crate::textures::{resources::Textures, HALF_BRICK_TILE_SIZE};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

pub fn add_brick_textures(
    mut commands: Commands,
    textures: ResMut<Textures>,
    mut bricks_query: Query<(Entity, &mut Brick, &Transform), Added<Brick>>,
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
) {
    let sand_destruction_sound = asset_server.load("sounds/sand_destruction.ogg");
    let sand_collision_sound = asset_server.load("sounds/sand_collision.ogg");
    let stone_destruction_sound = asset_server.load("sounds/stone_destruction.ogg");
    let stone_collision_sound = asset_server.load("sounds/stone_collision.ogg");
    let rock_destruction_sound = asset_server.load("sounds/rock_destruction.ogg");
    let rock_collision_sound = asset_server.load("sounds/rock_collision.ogg");
    let marble_destruction_sound = asset_server.load("sounds/marble_destruction.ogg");
    let marble_collision_sound = asset_server.load("sounds/marble_collision.ogg");
    for (brick_entity, mut brick, transform) in bricks_query.iter_mut() {
        let sprite = match brick.brick_type {
            BrickType::Sand => {
                brick.collision_sound = sand_collision_sound.clone();
                brick.destruction_sound = sand_destruction_sound.clone();
                textures.sand.clone()
            }
            BrickType::Stone => {
                brick.collision_sound = stone_collision_sound.clone();
                brick.destruction_sound = stone_destruction_sound.clone();
                textures.stone.clone()
            }
            BrickType::Rock => {
                brick.collision_sound = rock_collision_sound.clone();
                brick.destruction_sound = rock_destruction_sound.clone();
                textures.rock.clone()
            }
            BrickType::Marble => {
                brick.collision_sound = marble_collision_sound.clone();
                brick.destruction_sound = marble_destruction_sound.clone();
                textures.marble.clone()
            }
        };
        commands.entity(brick_entity).insert((
            SpriteBundle {
                texture: sprite.texture,
                transform: *transform,
                ..Default::default()
            },
            TextureAtlas {
                layout: sprite.layout,
                ..Default::default()
            },
            RigidBody::Fixed,
            Collider::cuboid(HALF_BRICK_TILE_SIZE, HALF_BRICK_TILE_SIZE),
        ));
    }
    if bricks_query.iter().count() != 0 {
        next_state.set(GameState::Game);
    }
}

pub fn collision_handler(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    mut brick_query: Query<(&mut TextureAtlas, &mut Brick)>,
    mut ball_speed_query: Query<&mut Velocity, With<Ball>>,
) {
    for ev in collisions.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = ev {
            let (brick_entity, mut ball_speed, mut sprite, mut brick) = {
                if let (Ok((sprite, brick)), Ok(ball_speed)) = (
                    brick_query.get_mut(*entity1),
                    ball_speed_query.get_mut(*entity2),
                ) {
                    (*entity1, ball_speed, sprite, brick)
                } else if let (Ok((sprite, brick)), Ok(ball_speed)) = (
                    brick_query.get_mut(*entity2),
                    ball_speed_query.get_mut(*entity1),
                ) {
                    (*entity2, ball_speed, sprite, brick)
                } else {
                    continue;
                }
            };

            let sum_speed = (ball_speed.linvel.x.powi(2) + ball_speed.linvel.y.powi(2)).sqrt();
            brick.health.0 -= (sum_speed / brick.resistance as f32) as i32;
            if brick.health.0 <= 0 {
                commands.entity(brick_entity).despawn();
                ball_speed.linvel *= brick.inhibition_rate;
                let rotate = Rng::gen_range(&mut rand::thread_rng(), 0..=10);
                ball_speed.linvel = Vec2::new(1.0, 0.1 - 0.02 * rotate as f32)
                    .normalize()
                    .rotate(ball_speed.linvel);
                commands.spawn(AudioBundle {
                    source: brick.destruction_sound.clone(),
                    settings: PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Despawn,
                        ..Default::default()
                    },
                });
            } else {
                sprite.index = ((100 - brick.health.0) * 9 / 100) as usize;
                commands.spawn(AudioBundle {
                    source: brick.collision_sound.clone(),
                    settings: PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Despawn,
                        ..Default::default()
                    },
                });
            }
        }
    }
}
