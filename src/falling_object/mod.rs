mod components;
mod systems;

use crate::textures::resources::Textures;
use crate::textures::HALF_BRICK_TILE_SIZE;
use crate::SCREEN_HEIGHT;
use crate::SCREEN_WIDTH;
use bevy::log;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

pub struct FallingObjectPlugin;

impl Plugin for FallingObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::detect_collision);
    }
}

pub fn spawn_falling_object(
    commands: &mut Commands,
    textures: &ResMut<Textures>,
    buff_probability: i32,
    debuff_probability: i32,
    x: f32,
    y: f32,
) {
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(0..100);
    log::info!("Random number: {}", random_number);
    if random_number < buff_probability {
        log::info!("Spawning buff at {}, {}", x, y);
        commands.spawn((
            components::Buff::ExpandPlatform,
            SpriteBundle {
                texture: textures.ball.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        x - SCREEN_WIDTH / 2.0 + HALF_BRICK_TILE_SIZE + 8.0,
                        y - SCREEN_HEIGHT / 2.0 + HALF_BRICK_TILE_SIZE + 8.0,
                        3.0,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            },
            RigidBody::Dynamic,
            Collider::ball(16.0),
            Velocity {
                linvel: Vec2::new(0.0, -200.0),
                angvel: 0.0,
            },
            ExternalImpulse {
                impulse: Vec2::new(0.0, 0.0),
                torque_impulse: 0.0,
            },
            ActiveEvents::COLLISION_EVENTS,
            Sensor,
        ));
    } else if buff_probability <= random_number
        && random_number < buff_probability + debuff_probability
    {
        log::info!("Spawning debuff");
        commands.spawn((
            components::Debuff::ShrinkPlatform,
            SpriteBundle {
                texture: textures.ball.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        x - SCREEN_WIDTH / 2.0 + HALF_BRICK_TILE_SIZE + 8.0,
                        y - SCREEN_HEIGHT / 2.0 + HALF_BRICK_TILE_SIZE + 8.0,
                        3.0,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            },
            RigidBody::Dynamic,
            Collider::ball(16.0),
            Velocity {
                linvel: Vec2::new(0.0, -200.0),
                angvel: 0.0,
            },
            ExternalImpulse {
                impulse: Vec2::new(0.0, 0.0),
                torque_impulse: 0.0,
            },
            ActiveEvents::COLLISION_EVENTS,
            Sensor,
        ));
    }
}
