use bevy::{color::palettes::css, prelude::*, render::camera::ScalingMode};

use crate::camera::components::MainCameraBundle;

pub fn initialize_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("MainCamera"),
        Camera2dBundle {
            projection: OrthographicProjection {
                far: 1000.,
                near: -1000.,
                scaling_mode: ScalingMode::WindowSize(1.0),
                ..Default::default()
            },
            camera: Camera {
                clear_color: ClearColorConfig::Custom(Color::Srgba(css::BLACK)),
                ..Default::default()
            },
            ..Default::default()
        },
        MainCameraBundle::default(),
    ));
}
