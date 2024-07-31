use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::data::{
    constants::{DEATH_ZONE_HEIGHT_HALF, DEATH_ZONE_WIDTH_HALF, DEFAULT_LAYER},
    death_zone::DeathZone,
};

#[derive(Bundle)]
pub struct DeathZoneBundle {
    pub name: Name,

    // Physics
    pub collider: Collider,
    pub sensor: Sensor,
    pub active_events: ActiveEvents,

    // Transform
    pub transform: TransformBundle,

    // Tags
    pub deathzone: DeathZone,
}

impl DeathZoneBundle {
    pub fn new(position: Vec2) -> Self {
        Self {
            name: Name::new("DeathZone"),
            collider: Collider::cuboid(DEATH_ZONE_WIDTH_HALF, DEATH_ZONE_HEIGHT_HALF),
            sensor: Sensor,
            active_events: ActiveEvents::COLLISION_EVENTS,
            transform: TransformBundle {
                local: Transform::from_translation(position.extend(DEFAULT_LAYER)),
                ..Default::default()
            },
            deathzone: DeathZone,
        }
    }
}
