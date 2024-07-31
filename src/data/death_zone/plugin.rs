use bevy::prelude::*;

use crate::data::death_zone::{systems::destroy_balls, DeathZone};

pub struct DeathZonePlugin;
impl Plugin for DeathZonePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DeathZone>();

        app.add_systems(Update, destroy_balls);
    }
}
