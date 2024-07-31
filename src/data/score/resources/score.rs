use bevy::prelude::*;

#[derive(Resource, Reflect, Default, Deref, DerefMut)]
#[reflect(Resource)]
pub struct Score(pub u32);
