use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use crate::components::*;

pub fn camera_follow(
    mut cam_query: Query<&mut Transform, (With<MainCamera>, Without<Spaceship>)>,
    ship_query: Query<&Transform, With<Spaceship>>,
    time: Res<Time>,
) {
    let Ok(mut cam_transform) = cam_query.single_mut() else {
        return;
    };
    let Ok(ship_transform) = ship_query.single() else {
        return;
    };

    let dt: f32 = time.delta_secs();
    let follow_strength: f32 = 5.0;

    cam_transform.translation =
        cam_transform.translation.lerp(ship_transform.translation, follow_strength * dt);
}

pub fn camera_zoom(
    mut scroll_evr: MessageReader<MouseWheel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    spaceship_query: Query<&mut Velocity, With<Spaceship>>,
    mut camera_query: Query<&mut Projection, With<MainCamera>>,
) {
    let mut camera_projection: Mut<Projection> = camera_query.single_mut().expect("REASON");
    if let Projection::Orthographic(ref mut orthographic_camera_projection) = *camera_projection {
        let spaceship_velocity: &Velocity = spaceship_query.single().expect("REASON");
        let velocity_threshold_for_camera_zoom: f32 = 0.0;
        let min_camera_scale: f32 = 1.0;
        let max_camera_scale: f32 = 3.0;
        let spaceship_absolute_speed: f32 = spaceship_velocity.0.length();
        let zoom_factor: f32 = 0.01;
        if spaceship_absolute_speed > velocity_threshold_for_camera_zoom {
            let target_scale = (min_camera_scale + spaceship_absolute_speed * zoom_factor).clamp(min_camera_scale, max_camera_scale);
            let smoothing: f32 = 5.0;
            orthographic_camera_projection.scale = orthographic_camera_projection.scale.lerp(target_scale, smoothing * 0.016);
        } else if keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight) {
            for ev in scroll_evr.read() {
                orthographic_camera_projection.scale -= ev.y * 0.1;
                orthographic_camera_projection.scale = orthographic_camera_projection.scale.clamp(0.5, 5.0);
            }
        }
    }
}
