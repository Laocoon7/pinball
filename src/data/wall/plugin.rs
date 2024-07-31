use bevy::prelude::*;

use crate::data::wall::Wall;

pub struct WallPlugin;
impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Wall>();
    }
}
