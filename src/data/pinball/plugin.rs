use bevy::prelude::*;

use crate::data::pinball::Pinball;

pub struct PinballPlugin;
impl Plugin for PinballPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Pinball>();
    }
}
