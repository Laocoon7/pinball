use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::data::spawner::SpawnerBundle;

#[derive(Serialize, Deserialize)]
pub struct SpawnerObject {
    pub position: Vec2,
}

impl SpawnerObject {
    pub fn spawn(&self, commands: &mut Commands) -> Entity {
        commands.spawn(SpawnerBundle::new(self.position)).id()
    }
}
