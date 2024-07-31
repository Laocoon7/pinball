use bevy::prelude::*;

use crate::data::spawner::Spawner;

pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Spawner>();
    }
}
