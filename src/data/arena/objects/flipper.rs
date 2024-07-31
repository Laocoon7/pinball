use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    data::flipper::{FlipperBundle, FlipperGroup},
    library::resources::Library,
};

#[derive(Serialize, Deserialize)]
pub struct FlipperObject {
    pub group: FlipperGroup,
    pub position: Vec2,
    pub scale: Option<Vec2>,
}

impl FlipperObject {
    pub fn spawn(&self, commands: &mut Commands, library: &Library) -> Entity {
        match self.group {
            FlipperGroup::Left => {
                commands.spawn(FlipperBundle::new_left(library, self.position, self.scale)).id()
            },
            FlipperGroup::Right => {
                commands.spawn(FlipperBundle::new_right(library, self.position, self.scale)).id()
            },
        }
    }
}
