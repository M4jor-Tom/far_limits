use bevy::prelude::*;
use crate::components::*;

pub fn spaceship_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        &Transform,
        &mut Velocity,
        &mut AngularVelocity,
    ), With<Spaceship>>,
    time: Res<Time>,
) {
    let (transform, mut velocity, mut angular_velocity) = query.single_mut();
    let dt = time.delta_seconds();

    let thrust = 600.0;
    let side_thrust = 450.0;
    let rotation_thrust = 6.0;

    let forward = transform.rotation * Vec3::Y;
    let right = transform.rotation * Vec3::X;

    // Rotation
    if keyboard.pressed(KeyCode::KeyA) {
        angular_velocity.0 += rotation_thrust * dt;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        angular_velocity.0 -= rotation_thrust * dt;
    }

    // Forward / backward
    if keyboard.pressed(KeyCode::KeyW) {
        velocity.0 += forward.truncate() * thrust * dt;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        velocity.0 -= forward.truncate() * thrust * dt;
    }

    // Strafing
    if keyboard.pressed(KeyCode::KeyQ) {
        velocity.0 -= right.truncate() * side_thrust * dt;
    }
    if keyboard.pressed(KeyCode::KeyE) {
        velocity.0 += right.truncate() * side_thrust * dt;
    }
}
