use bevy::prelude::*;
use rand::Rng;
use crate::{components::Asteroid, constants::{ASTEROID_BROWN_ASSET, ASTEROID_DARK_ASSET, ASTEROID_GREY_ASSET}};

pub fn spawn_asteroids(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let mut rng: rand::prelude::ThreadRng = rand::rng();

    const TOTAL_ITERATIONS: u8 = 40;
    const FIRST_THIRD: f32 = (TOTAL_ITERATIONS as f32) / 3.0;
    const SECOND_THIRD: f32 = FIRST_THIRD * 2.0;
    for iteration in 0..TOTAL_ITERATIONS {
        let x: f32 = rng.random_range(-4000.0..4000.0);
        let y: f32 = rng.random_range(-4000.0..4000.0);
        let interation_as_float_32: f32 = iteration as f32;
        let asteroid_asset: &str = match interation_as_float_32 {
            0.0..=FIRST_THIRD => ASTEROID_BROWN_ASSET,
            FIRST_THIRD..=SECOND_THIRD => ASTEROID_DARK_ASSET,
            _ => ASTEROID_GREY_ASSET
        };

        commands.spawn((
                Sprite {
                image: asset_server.load(asteroid_asset),
                ..default()
            },
            Transform::from_translation(Vec3::new(x, y, -10.0)),
            GlobalTransform::default(),
            Asteroid
        ));
    }
}
