use bevy::prelude::*;

use crate::data::score::{
    systems::{initialize_score, update_score_ui},
    Score, ScoreUi,
};

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Score>();
        app.register_type::<ScoreUi>();

        app.init_resource::<Score>();

        app.add_systems(Startup, initialize_score);
        app.add_systems(Update, update_score_ui);
    }
}
