use bevy::prelude::*;
use crate::components::*;

pub fn apply_physics(
    mut query: Query<(
        &mut Transform,
        &mut Velocity,
        &mut AngularVelocity,
    ), With<Spaceship>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity, mut angular_velocity) = query.single_mut();
    let dt = time.delta_seconds();

    transform.translation += velocity.0.extend(0.0) * dt;
    transform.rotate_z(angular_velocity.0 * dt);

    // Damping
    velocity.0 *= 0.99;
    angular_velocity.0 *= 0.95;
}
