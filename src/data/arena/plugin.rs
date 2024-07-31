use bevy::prelude::*;

use crate::data::arena::systems::build_arena;

pub struct ArenaPlugin;
impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_arena);
    }
}
