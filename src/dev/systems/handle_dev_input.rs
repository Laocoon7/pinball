use crate::data::pinball::PinballBundle;
use crate::library::resources::Library;
use bevy::prelude::*;

pub fn handle_dev_input(mut commands: Commands, keys: Res<ButtonInput<KeyCode>>, library: Res<Library>) {
    if keys.pressed(KeyCode::Space) {
        commands.spawn(PinballBundle::new(&library, Vec2::new(0.0, 150.0)));
    }
}
