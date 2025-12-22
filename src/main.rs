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
            transform: Transform::from_scale(Vec3::splat(0.5)),
            ..default()
        },
        Spaceship
    ));
}

#[derive(Component)]
struct Spaceship;

fn spaceship_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Spaceship>>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut();

    let thrust_speed = 400.0;
    let side_thrust_speed = 300.0;
    let rotation_speed = 3.5;

    let dt = time.delta_seconds();

    // -------- Rotation --------
    if keyboard.pressed(KeyCode::KeyQ) {
        transform.rotate_z(rotation_speed * dt);
    }
    if keyboard.pressed(KeyCode::KeyE) {
        transform.rotate_z(-rotation_speed * dt);
    }

    // -------- Local directions --------
    let forward = transform.rotation * Vec3::Y;
    let right = transform.rotation * Vec3::X;

    // -------- Forward / backward thrust --------
    let mut forward_thrust = 0.0;
    if keyboard.pressed(KeyCode::KeyW) {
        forward_thrust += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        forward_thrust -= 1.0;
    }

    // -------- Side thrusters --------
    let mut side_thrust = 0.0;
    if keyboard.pressed(KeyCode::KeyD) {
        side_thrust += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        side_thrust -= 1.0;
    }

    // -------- Apply movement --------
    transform.translation +=
        forward * forward_thrust * thrust_speed * dt +
        right * side_thrust * side_thrust_speed * dt;
}
