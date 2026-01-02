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
    let Ok((mut transform, mut velocity, mut angular_velocity)) = query.single_mut() else {
        return;
    };
    let dt = time.delta_secs();

    transform.translation += velocity.0.extend(0.0) * dt;
    transform.rotate_z(angular_velocity.0 * dt);

    // Damping
    velocity.0 *= 0.99;
    angular_velocity.0 *= 0.95;
}
