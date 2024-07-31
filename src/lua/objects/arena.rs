use bevy::prelude::*;

use crate::{library::resources::Library, lua::objects::LuaObject};

pub struct LuaArena {
    pub width: u32,
    pub height: u32,
    pub objects: Vec<LuaObject>,
}

impl LuaArena {
    pub fn spawn(&self, commands: &mut Commands, library: &Library) {
        for object in self.objects.iter() {
            object.spawn(commands, library);
        }
    }
}