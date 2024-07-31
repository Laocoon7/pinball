use bevy::prelude::*;

use crate::data::score::{Score, ScoreUi};

pub fn update_score_ui(score: Res<Score>, mut q_score_ui: Query<&mut Text, With<ScoreUi>>) {
    for mut text in q_score_ui.iter_mut() {
        text.sections[1].value = (**score).to_string();
    }
}
