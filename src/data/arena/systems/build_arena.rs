use bevy::prelude::*;

use crate::{
    data::{arena::Arena, constants::DEFAULT_ARENA},
    library::resources::Library,
};

pub fn build_arena(mut commands: Commands, library: Res<Library>) {
    match Arena::load(DEFAULT_ARENA) {
        Ok(arena) => {
            arena.spawn(&mut commands, &library);
            info!("Arena loaded!");
        },
        Err(e) => error!("{e}"),
    }
}
