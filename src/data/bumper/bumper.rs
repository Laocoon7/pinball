use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Bumper {
    pub score: u32,
}

impl Default for Bumper {
    fn default() -> Self {
        Self { score: 1000 }
    }
}
