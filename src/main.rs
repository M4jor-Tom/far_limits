use bevy::prelude::*;

mod components;
mod setup;
mod input;
mod physics;
mod camera;
mod background;

use setup::setup;
use input::spaceship_input;
use physics::apply_physics;
use camera::{camera_follow, camera_zoom};

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
                camera_zoom,
            ),
        )
        .run();
}
