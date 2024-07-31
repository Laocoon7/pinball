use bevy::prelude::*;

use crate::camera::{components::MainCamera, systems::initialize_camera};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MainCamera>();

        app.add_systems(Startup, initialize_camera);
    }
}
