use bevy::prelude::{Commands, Sprite, Camera2d, Res, AssetServer, Transform, default};
use bevy_rapier2d::prelude::*;
use crate::components::{MainCamera, Spaceship};
use crate::background::spawn_starfield;
use crate::constants::SPACESHIP_ASSET;
use crate::stellar_system::spawn_asteroids;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn((
        Camera2d::default(),
        MainCamera,
    ));

    // Spaceship
    commands.spawn((
        Sprite {
            image: asset_server.load(SPACESHIP_ASSET),
            ..default()
        },
        Transform::default(),
        RigidBody::Dynamic,
        Collider::ball(16.0),
        Velocity::zero(),
        Damping {
            linear_damping: 0.2,
            angular_damping: 1.0,
        },
        Spaceship,
    ));

    // Asteroids
    spawn_asteroids(&mut commands, asset_server);

    // Background
    spawn_starfield(&mut commands);
}
