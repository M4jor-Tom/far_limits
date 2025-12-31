use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use crate::components::*;

pub fn camera_follow(
    mut cam_query: Query<&mut Transform, (With<MainCamera>, Without<Spaceship>)>,
    ship_query: Query<&Transform, With<Spaceship>>,
    time: Res<Time>,
) {
    let mut cam_transform = cam_query.single_mut();
    let ship_transform = ship_query.single();

    let dt = time.delta_seconds();
    let follow_strength: f32 = 5.0;

    cam_transform.translation =
        cam_transform.translation.lerp(ship_transform.translation, follow_strength * dt);
}

pub fn camera_zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    let mut cam = query.single_mut();

    if keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight) {
        for ev in scroll_evr.read() {
            cam.scale -= ev.y * 0.1;
            cam.scale = cam.scale.clamp(0.5, 5.0);
        }
    }
}
