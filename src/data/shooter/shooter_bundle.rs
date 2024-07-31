use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    data::{
        constants::{DEFAULT_LAYER, SHOOTER_RESTITUTION, SHOOTER_SCALE, SHOOTER_SIZE_HALF},
        shooter::Shooter,
    },
    library::resources::Library,
};

#[derive(Bundle)]
pub struct ShooterBundle {
    pub name: Name,

    // Physics
    pub collider: Collider,
    pub restitution: Restitution,

    // Sprite
    pub sprite_bundle: SpriteBundle,

    // Tags
    pub shooter: Shooter,
}

impl ShooterBundle {
    pub fn new(library: &Library, position: Vec2) -> Self {
        let sprite_bundle = library.bumper(
            Transform {
                translation: position.extend(DEFAULT_LAYER),
                scale: Vec3::new(SHOOTER_SCALE, SHOOTER_SCALE, 1.0),
                ..Default::default()
            },
            None,
            None,
        );

        Self {
            name: Name::new("Shooter"),
            collider: Collider::ball(SHOOTER_SIZE_HALF),
            restitution: Restitution::coefficient(SHOOTER_RESTITUTION),
            sprite_bundle,
            shooter: Shooter,
        }
    }
}
