use bevy::prelude::*;

use crate::data::constants::{FLIPPER_MAX_ROTATION, FLIPPER_MIN_ROTATION};

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Flipper {
    pub current_rotation: f32,
}

impl Default for Flipper {
    fn default() -> Self {
        Self {
            current_rotation: FLIPPER_MIN_ROTATION,
        }
    }
}

impl Flipper {
    pub fn add_rotation(&mut self, angle: f32) -> f32 {
        self.current_rotation += angle;
        if self.current_rotation > FLIPPER_MAX_ROTATION {
            self.current_rotation = FLIPPER_MAX_ROTATION;
        }
        self.current_rotation
    }

    pub fn sub_rotation(&mut self, angle: f32) -> f32 {
        self.current_rotation -= angle;
        if self.current_rotation < FLIPPER_MIN_ROTATION {
            self.current_rotation = FLIPPER_MIN_ROTATION;
        }
        self.current_rotation
    }
}
