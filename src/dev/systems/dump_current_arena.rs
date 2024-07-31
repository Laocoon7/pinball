use bevy::prelude::*;

use crate::{data::arena::Arena, data::constants::DEFAULT_ARENA};

pub fn dump_current_arena(world: &mut World) {
    let keys = world.resource::<ButtonInput<KeyCode>>();
    if keys.just_pressed(KeyCode::F10) {
        let arena = Arena::from_world(world);

        match arena.save(DEFAULT_ARENA) {
            Ok(()) => {
                info!("Saved arena!");
            },
            Err(e) => error!("{e}"),
        }
    }
}
