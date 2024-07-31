use bevy::prelude::*;

use crate::data::score::resources::Score;

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Score>();

        app.init_resource::<Score>();
    }
}
