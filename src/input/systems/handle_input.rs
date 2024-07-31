use bevy::prelude::*;

use crate::{
    data::{
        constants::FLIPPER_SPEED,
        flipper::{Flipper, FlipperGroup},
        pinball::{Pinball, PinballBundle},
        score::Score,
    },
    library::resources::Library,
};

pub fn handle_input(
    mut commands: Commands,
    library: Res<Library>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut score: ResMut<Score>,
    mut q_flippers: Query<(&mut Transform, &mut Flipper, &FlipperGroup)>,
    q_pinballs: Query<Entity, With<Pinball>>,
    mut e_app_exit: EventWriter<AppExit>,
) {
    let rotation_increment = time.delta_seconds() * FLIPPER_SPEED;

    for (mut flipper_transform, mut flipper, flipper_group) in q_flippers.iter_mut() {
        match flipper_group {
            FlipperGroup::Left => {
                let new_rotation = if keys.pressed(KeyCode::KeyZ) {
                    flipper.add_rotation(rotation_increment)
                } else {
                    flipper.sub_rotation(rotation_increment)
                };
                flipper_transform.rotation = Quat::from_rotation_z(new_rotation.to_radians());
                flipper.current_rotation = new_rotation;
            },
            FlipperGroup::Right => {
                let new_rotation = if keys.pressed(KeyCode::Slash) {
                    flipper.add_rotation(rotation_increment)
                } else {
                    flipper.sub_rotation(rotation_increment)
                };
                flipper_transform.rotation = Quat::from_rotation_z(-new_rotation.to_radians());
                flipper.current_rotation = new_rotation;
            },
        }
    }

    if keys.pressed(KeyCode::Escape) {
        e_app_exit.send(AppExit::Success);
    }

    if keys.just_released(KeyCode::KeyN)
        && (keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight))
        && q_pinballs.iter().len() == 0
    {
        **score = 0;
        commands.spawn(PinballBundle::new(&library, Vec2::new(396.0, 350.0)));
    }
}
