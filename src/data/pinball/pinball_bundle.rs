use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    data::{
        constants::{BALL_MASS, BALL_RESTITUTION, BALL_SCALE, BALL_SIZE_HALF, DEFAULT_LAYER},
        pinball::Pinball,
    },
    library::resources::Library,
};

#[derive(Bundle)]
pub struct PinballBundle {
    pub name: Name,

    // Physics
    pub rigidbody: RigidBody,
    pub ccd: Ccd,
    pub collider: Collider,
    pub collider_mass_properties: ColliderMassProperties,
    pub restitution: Restitution,

    // Sprite
    pub sprite_bundle: SpriteBundle,

    // Tags
    pub pinball: Pinball,
}

impl PinballBundle {
    pub fn new(library: &Library, position: Vec2) -> Self {
        let sprite_bundle = library.ball(
            Transform {
                translation: position.extend(DEFAULT_LAYER),
                scale: Vec3::new(BALL_SCALE, BALL_SCALE, 1.0),
                ..Default::default()
            },
            None,
        );

        Self {
            name: Name::new("Pinball"),
            rigidbody: RigidBody::Dynamic,
            ccd: Ccd::enabled(),
            collider: Collider::ball(BALL_SIZE_HALF),
            collider_mass_properties: ColliderMassProperties::Mass(BALL_MASS),
            restitution: Restitution {
                coefficient: BALL_RESTITUTION,
                combine_rule: CoefficientCombineRule::Max,
            },
            sprite_bundle,
            pinball: Pinball,
        }
    }
}
