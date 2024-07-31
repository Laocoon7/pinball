use bevy::prelude::*;

use crate::{library::resources::Library, lua::objects::{BumperObject, DeathZoneObject, FlipperObject, ShooterObject, WallObject}};

pub enum LuaObject {
    Bumper(BumperObject),
    DeathZone(DeathZoneObject),
    Flipper(FlipperObject),
    Shooter(ShooterObject),
    Wall(WallObject),
}

impl LuaObject {
    pub fn spawn(&self, commands: &mut Commands, library: &Library) -> Entity {
        match self {
            LuaObject::Bumper(bumper) => bumper.spawn(commands, library),
            LuaObject::DeathZone(death_zone) => death_zone.spawn(commands),
            LuaObject::Flipper(flipper) => flipper.spawn(commands, library),
            LuaObject::Shooter(shooter) => shooter.spawn(commands, library),
            LuaObject::Wall(wall) => wall.spawn(commands, library),
        }
    }
}