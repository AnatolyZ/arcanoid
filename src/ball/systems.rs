use super::{components::Ball, MAX_X_SPEED, MAX_Y_SPEED, MIM_Y_SPEED, MIN_X_SPEED};
use crate::platform::Platform;
use crate::textures::resources::Textures;
use crate::textures::{HALF_TILE_SIZE, TILE_SIZE};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const BALL_SPRITE_RADIUS: f32 = 16.0;

pub fn spawn_ball(
    mut commands: Commands,
    textures: Res<Textures>,
    platform_entity_query: Query<(Entity, &Platform)>,
) {
    let (platform_entity, platform) = platform_entity_query.single();
    let ball_entity = commands
        .spawn((
            Ball {
                radius: BALL_SPRITE_RADIUS,
                lay_on_platform: true,
            },
            SpriteBundle {
                texture: textures.ball.clone(),
                transform: Transform::from_xyz(
                    platform.length as f32 * TILE_SIZE / 2.0 - HALF_TILE_SIZE,
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

pub fn confine_ball_speed(mut query: Query<&mut Velocity, With<Ball>>) {
    for mut binding in query.iter_mut() {
        let force = binding.as_mut();

        //Confine max speed
        if force.linvel.x > MAX_X_SPEED {
            force.linvel.x = MAX_X_SPEED;
        }
        if force.linvel.x < -MAX_X_SPEED {
            force.linvel.x = -MAX_X_SPEED;
        }
        if force.linvel.y > MAX_Y_SPEED {
            force.linvel.y = MAX_Y_SPEED;
        }
        if force.linvel.y < -MAX_Y_SPEED {
            force.linvel.y = -MAX_Y_SPEED;
        }

        //Confine min speed
        if force.linvel.x > 0.0 && force.linvel.x < MIN_X_SPEED {
            force.linvel.x = MIN_X_SPEED;
        }
        if force.linvel.x < 0.0 && force.linvel.x > -MIN_X_SPEED {
            force.linvel.x = -MIN_X_SPEED;
        }
        if force.linvel.y > 0.0 && force.linvel.y < MIM_Y_SPEED {
            force.linvel.y = MIM_Y_SPEED;
        }
        if force.linvel.y < 0.0 && force.linvel.y > -MIM_Y_SPEED {
            force.linvel.y = -MIM_Y_SPEED;
        }
    }
}

pub fn launch_ball(
    mut commands: Commands,
    mut ball_query: Query<(Entity, &mut Velocity, &mut Transform, &mut Ball)>,
    mut platform_query: Query<(Entity, &Transform, &Velocity, &Platform), Without<Ball>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (ball_entity, mut velocity, mut ball_transform, mut ball) in ball_query.iter_mut() {
        if ball.lay_on_platform {
            let (platform_entity, platform_transform, platform_velocity, platform) =
                platform_query.single_mut();
            if keyboard_input.just_pressed(KeyCode::Up)
                || keyboard_input.just_pressed(KeyCode::Space)
            {
                ball.lay_on_platform = false;
                velocity.linvel = Vec2::new(platform_velocity.linvel.x, 200.0);
                ball_transform.translation.y =
                    platform_transform.translation.y + ball.radius + HALF_TILE_SIZE;
                ball_transform.translation.x = platform_transform.translation.x - HALF_TILE_SIZE
                    + platform.length as f32 * TILE_SIZE / 2.0;
                commands
                    .entity(platform_entity)
                    .remove_children(&vec![ball_entity]);
            }
        }
    }
}
