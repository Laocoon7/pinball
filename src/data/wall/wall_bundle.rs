use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    data::{
        constants::{DEFAULT_LAYER, WALL_RESTITUTION, WALL_THICKNESS_HALF},
        wall::Wall,
    },
    library::resources::Library,
};

#[derive(Bundle)]
pub struct WallBundle {
    pub name: Name,

    // Physics
    pub collider: Collider,
    pub restitution: Restitution,

    // Sprite
    pub sprite_bundle: SpriteBundle,
    pub atlas: TextureAtlas,

    // Tags
    pub wall: Wall,
}

impl WallBundle {
    pub fn new(library: &Library, start: Vec2, end: Vec2) -> Self {
        let center = (start + end) * 0.5;
        let direction = end - start;
        let angle = direction.y.atan2(direction.x);
        let scale = direction.length();

        let (sprite_bundle, atlas) = library.wall_middle(
            Transform {
                translation: center.extend(DEFAULT_LAYER),
                scale: Vec3::new(scale / 32.0, 1.0, 1.0),
                rotation: Quat::from_rotation_z(angle),
            },
            None,
            None,
        );

        Self {
            name: Name::new("Wall"),
            collider: Collider::cuboid(WALL_THICKNESS_HALF, WALL_THICKNESS_HALF),
            restitution: Restitution::coefficient(WALL_RESTITUTION),
            sprite_bundle,
            atlas,
            wall: Wall::new(start, end),
        }
    }
}
