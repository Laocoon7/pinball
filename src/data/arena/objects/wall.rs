use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{data::wall::WallBundle, library::resources::Library};

#[derive(Serialize, Deserialize)]
pub struct WallObject {
    pub start: Vec2,
    pub end: Vec2,
}

impl WallObject {
    pub fn spawn(&self, commands: &mut Commands, library: &Library) -> Entity {
        commands.spawn(WallBundle::new(library, self.start, self.end)).id()
    }
}
