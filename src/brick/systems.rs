use super::components::{Brick, Health};
use crate::ball::Ball;
use crate::textures::{resources::Textures, HALF_TILE_SIZE, TILE_SIZE};
use bevy::log;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn spawn_brick(
    commands: &mut Commands,
    coordinates: Vec2,
    texture: Handle<TextureAtlas>,
    resistance: u8,
    inhibition_rate: f32,
) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture,
            transform: Transform::from_xyz(coordinates.x, coordinates.y, 0.0),
            sprite: TextureAtlasSprite::new(0), // always spawn a brick without a crack
            ..Default::default()
        },
        ActiveEvents::COLLISION_EVENTS,
        RigidBody::Fixed,
        Collider::cuboid(HALF_TILE_SIZE * 3.0, HALF_TILE_SIZE * 3.0),
        Brick {
            resistance,
            inhibition_rate,
        },
        Health(100),
    ));
}

pub fn spawn_bricks_array(mut commands: Commands, textures: ResMut<Textures>) {
    for i in 0..3 {
        for j in 0..3 {
            spawn_brick(
                &mut commands,
                Vec2::new(i as f32 * TILE_SIZE * 3.0, j as f32 * TILE_SIZE * 3.0),
                textures.sand.clone(),
                5,
                0.5,
            );
        }
    }
}

pub fn collision_handler(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    mut brick_query: Query<(&mut TextureAtlasSprite, &mut Health, &Brick)>,
    mut ball_speed_query: Query<&mut Velocity, With<Ball>>,
) {
    for ev in collisions.iter() {
        if let CollisionEvent::Started(ball_entity, brick_entity, _) = ev {
            log::info!("Collision detected!");
            if let Ok((mut sprite, mut health, brick)) = brick_query.get_mut(*brick_entity) {
                if let Ok(mut ball_speed) = ball_speed_query.get_mut(*ball_entity) {
                    log::info!("Speed: {:?}", ball_speed);
                    let sum_speed =
                        (ball_speed.linvel.x.powi(2) + ball_speed.linvel.y.powi(2)).sqrt();
                    log::info!("Sum speed: {:?}", sum_speed);
                    log::info!("Health before: {:?}", health.0);
                    health.0 -= (sum_speed / brick.resistance as f32) as i32;
                    log::info!("Health after: {:?}", health.0);
                    if health.0 <= 0 {
                        commands.entity(*brick_entity).despawn();
                        ball_speed.linvel.y *= brick.inhibition_rate;
                        ball_speed.linvel.x *= brick.inhibition_rate;
                    } else {
                        sprite.index = ((100 - health.0) * 9 / 100) as usize;
                    }
                }
            }
        }
    }
}
