use bevy::prelude::*;
use rand::Rng;

pub fn spawn_starfield(commands: &mut Commands) {
    let mut rng = rand::rng();

    for _ in 0..400 {
        let x = rng.random_range(-4000.0..4000.0);
        let y = rng.random_range(-4000.0..4000.0);
        let size = rng.random_range(0.5..2.0);

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(x, y, -10.0)),
            ..default()
        });
    }
}
