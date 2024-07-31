use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    data::arena::objects::{
        BumperObject, DeathZoneObject, FlipperObject, ShooterObject, SpawnerObject, WallObject,
    },
    library::resources::Library,
};

#[derive(Serialize, Deserialize)]
pub enum ArenaObject {
    Bumper(BumperObject),
    DeathZone(DeathZoneObject),
    Flipper(FlipperObject),
    Shooter(ShooterObject),
    Spawner(SpawnerObject),
    Wall(WallObject),
}

impl ArenaObject {
    pub fn spawn(&self, commands: &mut Commands, library: &Library) -> Entity {
        match self {
            ArenaObject::Bumper(bumper) => bumper.spawn(commands, library),
            ArenaObject::DeathZone(death_zone) => death_zone.spawn(commands),
            ArenaObject::Flipper(flipper) => flipper.spawn(commands, library),
            ArenaObject::Shooter(shooter) => shooter.spawn(commands, library),
            ArenaObject::Spawner(spawner) => spawner.spawn(commands),
            ArenaObject::Wall(wall) => wall.spawn(commands, library),
        }
    }
}
