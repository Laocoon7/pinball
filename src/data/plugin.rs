use bevy::prelude::*;

use crate::data::{
    arena::ArenaPlugin, bumper::BumperPlugin, death_zone::DeathZonePlugin, flipper::FlipperPlugin,
    pinball::PinballPlugin, score::ScorePlugin, shooter::ShooterPlugin, spawner::SpawnerPlugin,
    wall::WallPlugin,
};

pub struct DataPlugin;
impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ArenaPlugin);
        app.add_plugins(BumperPlugin);
        app.add_plugins(DeathZonePlugin);
        app.add_plugins(FlipperPlugin);
        app.add_plugins(PinballPlugin);
        app.add_plugins(ScorePlugin);
        app.add_plugins(ShooterPlugin);
        app.add_plugins(SpawnerPlugin);
        app.add_plugins(WallPlugin);
    }
}
