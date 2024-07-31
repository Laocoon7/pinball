use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Component, Reflect, Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub enum FlipperGroup {
    #[default]
    Left,
    Right,
}
