use bevy::prelude::*;

use crate::{data::bumper::BumperBundle, library::resources::Library};

pub struct BumperObject {
    pub position: Vec2,
}

impl BumperObject {
    pub fn spawn(&self, commands: &mut Commands, library: &Library) -> Entity {
        commands.spawn(BumperBundle::new(library, self.position)).id()
    }
}