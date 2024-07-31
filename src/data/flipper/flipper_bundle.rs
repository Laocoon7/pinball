use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    data::{
        constants::{DEFAULT_LAYER, FLIPPER_RESTITUTION},
        flipper::{Flipper, FlipperGroup},
    },
    library::resources::Library,
};

#[derive(Bundle)]
pub struct FlipperBundle {
    pub name: Name,

    // Physics
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub restitution: Restitution,

    // Sprite
    pub sprite_bundle: SpriteBundle,

    // Tags
    pub group: FlipperGroup,
    pub flipper: Flipper,
}

impl FlipperBundle {
    pub fn new_left(library: &Library, position: Vec2, scale: Option<Vec2>) -> Self {
        let sprite_bundle = library.flipper_left(
            Transform {
                translation: position.extend(DEFAULT_LAYER),
                scale: scale.unwrap_or(Vec2::new(1.0, 1.0)).extend(1.0),
                ..Default::default()
            },
            None,
            None,
        );

        Self {
            name: Name::new("Flipper"),
            rigidbody: RigidBody::KinematicPositionBased,
            collider: Collider::compound(vec![
                (Vec2::new(262.0, -17.0), 0.0, Collider::ball(27.0)),
                (Vec2::new(5.6, 0.0), 0.0, Collider::ball(46.0)),
                (
                    Vec2::new(0.0, 0.0),
                    0.0,
                    Collider::convex_polyline(vec![
                        Vec2::new(262.0, 11.0),
                        Vec2::new(262.0, -44.0),
                        Vec2::new(5.6, -44.0),
                        Vec2::new(5.6, 46.0),
                    ])
                    .expect("Failed to draw collider lines"),
                ),
            ]),
            restitution: Restitution::coefficient(FLIPPER_RESTITUTION),
            sprite_bundle,
            group: FlipperGroup::Left,
            flipper: Flipper::default(),
        }
    }

    pub fn new_right(library: &Library, position: Vec2, scale: Option<Vec2>) -> Self {
        let sprite_bundle = library.flipper_right(
            Transform {
                translation: position.extend(DEFAULT_LAYER),
                scale: scale.unwrap_or(Vec2::new(1.0, 1.0)).extend(1.0),
                ..Default::default()
            },
            None,
            None,
        );

        Self {
            name: Name::new("Flipper"),
            rigidbody: RigidBody::KinematicPositionBased, // maybe velocity based?
            collider: Collider::compound(vec![
                (Vec2::new(-255.0, -17.0), 0.0, Collider::ball(27.0)),
                (Vec2::new(3.0, 0.0), 0.0, Collider::ball(46.0)),
                (
                    Vec2::new(0.0, 0.0),
                    0.0,
                    Collider::convex_polyline(vec![
                        Vec2::new(-255.0, 11.0),
                        Vec2::new(-255.0, -44.0),
                        Vec2::new(3.0, -44.0),
                        Vec2::new(3.0, 46.0),
                    ])
                    .expect("Failed to draw collider lines"),
                ),
            ]),
            restitution: Restitution::coefficient(FLIPPER_RESTITUTION),
            sprite_bundle,
            group: FlipperGroup::Right,
            flipper: Flipper::default(),
        }
    }
}
