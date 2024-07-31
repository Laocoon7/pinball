use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::data::death_zone::DeathZoneBundle;

#[derive(Serialize, Deserialize)]
pub struct DeathZoneObject {
    pub position: Vec2,
}

impl DeathZoneObject {
    pub fn spawn(&self, commands: &mut Commands) -> Entity {
        commands.spawn(DeathZoneBundle::new(self.position)).id()
    }
}
