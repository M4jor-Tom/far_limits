use bevy::prelude::*;
use rand::Rng;

pub fn spawn_starfield(commands: &mut Commands) {
    let mut rng: rand::prelude::ThreadRng = rand::rng();

    for _ in 0..400 {
        let x: f32 = rng.random_range(-4000.0..4000.0);
        let y: f32 = rng.random_range(-4000.0..4000.0);
        let size: f32 = rng.random_range(0.5..2.0);

        commands.spawn((
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            Transform::from_translation(Vec3::new(x, y, -10.0)),
            GlobalTransform::default(),
        ));
    }
}
