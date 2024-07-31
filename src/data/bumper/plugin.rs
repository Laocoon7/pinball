use bevy::prelude::*;

use crate::data::bumper::{systems::add_score, Bumper};

pub struct BumperPlugin;
impl Plugin for BumperPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Bumper>();

        app.add_systems(Update, add_score);
    }
}
