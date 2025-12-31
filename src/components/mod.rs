use bevy::prelude::*;

#[derive(Component)]
pub struct Spaceship;

#[derive(Component)]
pub struct Asteroid;

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default)]
pub struct AngularVelocity(pub f32);

#[derive(Component)]
pub struct MainCamera;
