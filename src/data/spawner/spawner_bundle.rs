use bevy::prelude::*;

use crate::data::{constants::DEFAULT_LAYER, spawner::Spawner};

#[derive(Bundle)]
pub struct SpawnerBundle {
    pub name: Name,

    // Transform
    pub transform: TransformBundle,

    // Tags
    pub spawner: Spawner,
}

impl SpawnerBundle {
    pub fn new(position: Vec2) -> Self {
        Self {
            name: Name::new("Spawner"),
            transform: TransformBundle {
                local: Transform::from_translation(position.extend(DEFAULT_LAYER)),
                ..Default::default()
            },
            spawner: Spawner,
        }
    }
}
