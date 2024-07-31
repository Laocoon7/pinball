use bevy::prelude::*;

use crate::camera::components::MainCamera;

#[derive(Bundle, Default)]
pub struct MainCameraBundle {
    pub main_camera: MainCamera,
}
