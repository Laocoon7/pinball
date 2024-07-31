use bevy::prelude::*;

use crate::data::{
    constants::{WINDOW_LEFT, WINDOW_TOP},
    score::ScoreUiBundle,
};

pub fn initialize_score(mut commands: Commands) {
    commands.spawn(ScoreUiBundle::new(Vec2::new(WINDOW_LEFT, WINDOW_TOP)));
}
