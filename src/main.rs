use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                spaceship_input,
                apply_physics,
                camera_follow,
                camera_zoom
            ),
        )
        .run();
}

/* ===================== COMPONENTS ===================== */

#[derive(Component)]
struct Spaceship;

#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Component, Default)]
struct AngularVelocity(f32);

#[derive(Component)]
struct MainCamera;

/* ===================== SETUP ===================== */

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    // Starfield background
    spawn_starfield(&mut commands);
}

/* ===================== INPUT ===================== */

fn spaceship_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        &Transform,
        &mut Velocity,
        &mut AngularVelocity,
    ), With<Spaceship>>,
    time: Res<Time>,
) {
    let (transform, mut velocity, mut angular_velocity) = query.single_mut();

    let dt = time.delta_seconds();

    let thrust = 600.0;
    let side_thrust = 450.0;
    let rotation_thrust = 6.0;

    // Local axes
    let forward = transform.rotation * Vec3::Y;
    let right = transform.rotation * Vec3::X;

    // -------- Rotation inertia --------
    if keyboard.pressed(KeyCode::KeyA) {
        angular_velocity.0 += rotation_thrust * dt;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        angular_velocity.0 -= rotation_thrust * dt;
    }

    // -------- Forward / backward thrust --------
    if keyboard.pressed(KeyCode::KeyW) {
        velocity.0 += forward.truncate() * thrust * dt;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        velocity.0 -= forward.truncate() * thrust * dt;
    }

    // -------- Side thrusters --------
    if keyboard.pressed(KeyCode::KeyQ) {
        velocity.0 -= right.truncate() * side_thrust * dt;
    }
    if keyboard.pressed(KeyCode::KeyE) {
        velocity.0 += right.truncate() * side_thrust * dt;
    }
}

/* ===================== PHYSICS ===================== */

fn apply_physics(
    mut query: Query<(
        &mut Transform,
        &mut Velocity,
        &mut AngularVelocity,
    ), With<Spaceship>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity, mut angular_velocity) =
        query.single_mut();

    let dt = time.delta_seconds();

    // Apply movement
    transform.translation += velocity.0.extend(0.0) * dt;
    transform.rotate_z(angular_velocity.0 * dt);

    // Damping (space drag / stabilization)
    velocity.0 *= 0.99;
    angular_velocity.0 *= 0.95;
}

/* ===================== CAMERA ===================== */

fn camera_follow(
    mut cam_query: Query<&mut Transform, (With<MainCamera>, Without<Spaceship>)>,
    ship_query: Query<&Transform, With<Spaceship>>,
    time: Res<Time>,
) {
    let mut cam_transform = cam_query.single_mut();
    let ship_transform = ship_query.single();

    let dt = time.delta_seconds();
    let follow_strength = 5.0;

    cam_transform.translation = cam_transform
        .translation
        .lerp(ship_transform.translation, follow_strength * dt);
}

fn camera_zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    let mut cam = query.single_mut();

    // Only zoom if Ctrl is held
    if keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight) {
        for ev in scroll_evr.read() {
            // Adjust scale (smaller = zoom in, larger = zoom out)
            cam.scale -= ev.y * 0.1;
            cam.scale = cam.scale.clamp(0.5, 5.0); // limit zoom
        }
    }
}

/* ===================== BACKGROUND ===================== */

fn spawn_starfield(commands: &mut Commands) {
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
