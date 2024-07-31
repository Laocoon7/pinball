use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    data::{
        bumper::Bumper,
        constants::{BUMPER_RESTITUTION, BUMPER_SCALE, BUMPER_SIZE_HALF, DEFAULT_LAYER},
    },
    library::resources::Library,
};

#[derive(Bundle)]
pub struct BumperBundle {
    pub name: Name,

    // Physics
    pub collider: Collider,
    pub restitution: Restitution,
    pub active_events: ActiveEvents,

    // Sprite
    pub sprite_bundle: SpriteBundle,

    // Tags
    pub bumper: Bumper,
}

impl BumperBundle {
    pub fn new(library: &Library, position: Vec2) -> Self {
        let sprite_bundle = library.bumper(
            Transform {
                translation: position.extend(DEFAULT_LAYER),
                scale: Vec3::new(BUMPER_SCALE, BUMPER_SCALE, 1.0),
                ..Default::default()
            },
            None,
            None,
        );

        Self {
            name: Name::new("Bumper"),
            collider: Collider::ball(BUMPER_SIZE_HALF),
            restitution: Restitution::coefficient(BUMPER_RESTITUTION),
            active_events: ActiveEvents::COLLISION_EVENTS,
            sprite_bundle,
            bumper: Bumper::default(),
        }
    }
}
