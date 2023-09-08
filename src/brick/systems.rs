use super::components::{Brick, BrickType};
use crate::ball::Ball;
use crate::textures::{resources::Textures, HALF_BRICK_TILE_SIZE};
use bevy::log;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn add_brick_textures(
    mut commands: Commands,
    textures: ResMut<Textures>,
    bricks_query: Query<(Entity, &Brick, &Transform), Added<Brick>>,
) {
    for (brick_entity, brick, transform) in bricks_query.iter() {
        let texture = match brick.brick_type {
            BrickType::Sand => textures.sand.clone(),
            BrickType::Stone => textures.stone.clone(),
            BrickType::Rock => textures.rock.clone(),
            BrickType::Marble => textures.marble.clone(),
        };
        log::info!("Brick Entity: {:?}", brick_entity);
        commands.entity(brick_entity).insert((
            SpriteSheetBundle {
                texture_atlas: texture,
                transform: *transform,
                sprite: TextureAtlasSprite::new(0), // always spawn a brick without a crack
                ..Default::default()
            },
            RigidBody::Fixed,
            Collider::cuboid(HALF_BRICK_TILE_SIZE, HALF_BRICK_TILE_SIZE),
        ));
    }
}

pub fn collision_handler(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    mut brick_query: Query<(&mut TextureAtlasSprite, &mut Brick)>,
    mut ball_speed_query: Query<&mut Velocity, With<Ball>>,
) {
    for ev in collisions.iter() {
        if let CollisionEvent::Started(entity1, entity2, _) = ev {
            log::info!("Collision detected!");
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

            log::info!("Speed: {:?}", ball_speed);
            let sum_speed = (ball_speed.linvel.x.powi(2) + ball_speed.linvel.y.powi(2)).sqrt();
            log::info!("Sum speed: {:?}", sum_speed);
            log::info!("Health before: {:?}", brick.health.0);
            brick.health.0 -= (sum_speed / brick.resistance as f32) as i32;
            log::info!("Health after: {:?}", brick.health.0);
            if brick.health.0 <= 0 {
                commands.entity(brick_entity).despawn();
                ball_speed.linvel.y *= brick.inhibition_rate;
                ball_speed.linvel.x *= brick.inhibition_rate;
            } else {
                sprite.index = ((100 - brick.health.0) * 9 / 100) as usize;
            }
        }
    }
}
