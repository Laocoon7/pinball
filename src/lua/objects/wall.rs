use bevy::prelude::*;

use crate::{data::wall::WallBundle, library::resources::Library};

pub struct WallObject {
    start: Vec2,
    end: Vec2,
}

impl WallObject {
    pub fn spawn(&self, commands: &mut Commands, library: &Library) -> Entity {
        commands.spawn(WallBundle::new(library, self.start, self.end)).id()
    }
}