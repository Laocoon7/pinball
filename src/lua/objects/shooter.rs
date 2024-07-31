use bevy::prelude::*;

use crate::{data::shooter::ShooterBundle, library::resources::Library};

pub struct ShooterObject {
    pub position: Vec2,
}

impl ShooterObject {
    pub fn spawn(&self, commands: &mut Commands, library: &Library) -> Entity {
        commands.spawn(ShooterBundle::new(library, self.position)).id()
    }
}