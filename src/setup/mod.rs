use bevy::prelude::*;
use crate::components::*;
use crate::background::spawn_starfield;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
    ));

    // Spaceship
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("spaceship.png"),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        Spaceship,
        Velocity::default(),
        AngularVelocity::default(),
    ));

    // Background
    spawn_starfield(&mut commands);
}
