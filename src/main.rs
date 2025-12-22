use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, spaceship_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("spaceship.png"),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        Spaceship
    ));
}

#[derive(Component)]
struct Spaceship;

fn spaceship_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Spaceship>>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut();

    let speed = 300.0; // pixels per second
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction != Vec3::ZERO {
        direction = direction.normalize(); // prevents faster diagonal movement
        transform.translation += direction * speed * time.delta_seconds();
    }
}
