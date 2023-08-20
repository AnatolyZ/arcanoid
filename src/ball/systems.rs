use super::{components::Ball, MAX_X_SPEED, MAX_Y_SPEED, MIM_Y_SPEED, MIN_X_SPEED};
use crate::textures::resources::Textures;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn spawn_ball(mut commands: Commands, textures: Res<Textures>) {
    commands.spawn((
        Ball(),
        SpriteBundle {
            texture: textures.ball.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::ball(16.0),
        Velocity {
            linvel: Vec2::new(4.0, 4.0),
            angvel: 0.4,
        },
        ExternalImpulse {
            impulse: Vec2::new(0.0, -8.0),
            torque_impulse: 0.0,
        },
        Ccd::enabled(),
        Restitution::new(2.0),
    ));
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
