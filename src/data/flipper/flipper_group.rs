use bevy::prelude::*;

#[derive(Debug, Component, Reflect, Default, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub enum FlipperGroup {
    #[default]
    Left,
    Right,
}
