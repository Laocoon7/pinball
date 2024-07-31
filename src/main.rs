#![allow(clippy::module_inception)]
#![allow(clippy::type_complexity)]

use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier2d::prelude::*;

use crate::data::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub mod camera;
pub mod data;
#[cfg(feature = "dev")]
pub mod dev;
pub mod input;
pub mod library;

const PIXELS_PER_METER: f32 = 100.0; // Default from examples
const GRAVITY: f32 = 9.81 * PIXELS_PER_METER;
const TABLE_ANGLE: f32 = 10.0; // Set between 6.0 and 10.0
const TABLE_ANGLE_RAD: f32 = TABLE_ANGLE * (std::f32::consts::PI / 180.0);

fn main() {
    let mut app = App::new();
    // Bevy
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Pinball".to_string(),
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            ..Default::default()
        }),
        ..Default::default()
    }));

    // Rapier
    let mut rapier_configuration = RapierConfiguration::new(1.0);
    rapier_configuration.gravity = Vec2::NEG_Y * GRAVITY * TABLE_ANGLE_RAD.sin();
    app.insert_resource(rapier_configuration);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
        PIXELS_PER_METER,
    ));

    // Our plugins
    #[cfg(feature = "dev")]
    app.add_plugins(dev::DevPlugin);
    app.add_plugins(camera::CameraPlugin);
    app.add_plugins(data::DataPlugin);
    app.add_plugins(input::InputPlugin);
    app.add_plugins(library::LibraryPlugin);

    // Run
    app.run();
}
