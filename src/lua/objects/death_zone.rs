use bevy::prelude::*;

use crate::data::death_zone::DeathZoneBundle;

pub struct DeathZoneObject {
    pub position: Vec2,
}

impl DeathZoneObject {
    pub fn spawn(&self, commands: &mut Commands) -> Entity {
        commands.spawn(DeathZoneBundle::new(self.position)).id()
    }
}