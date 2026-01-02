use bevy::prelude::*;
use crate::components::*;
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
        Transform::from_scale(Vec3::splat(2.0)),
        GlobalTransform::default(),
        Spaceship,
        Velocity::default(),
        AngularVelocity::default(),
    ));

    // Asteroids
    spawn_asteroids(&mut commands, asset_server);

    // Background
    spawn_starfield(&mut commands);
}
