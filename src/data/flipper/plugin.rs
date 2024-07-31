use bevy::prelude::*;

use crate::data::flipper::{Flipper, FlipperGroup};

pub struct FlipperPlugin;
impl Plugin for FlipperPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FlipperGroup>();
        app.register_type::<Flipper>();
    }
}
