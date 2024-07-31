use bevy::prelude::*;

use crate::data::shooter::Shooter;

pub struct ShooterPlugin;
impl Plugin for ShooterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Shooter>();
    }
}
