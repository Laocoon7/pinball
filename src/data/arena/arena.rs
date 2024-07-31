use std::path::Path;

use anyhow::Result;
use bevy::{ecs::system::SystemState, prelude::*, window::PrimaryWindow};
use serde::{Deserialize, Serialize};

use crate::{
    data::{
        arena::objects::{
            ArenaObject, BumperObject, DeathZoneObject, FlipperObject, ShooterObject, SpawnerObject,
            WallObject,
        },
        bumper::Bumper,
        death_zone::DeathZone,
        flipper::{Flipper, FlipperGroup},
        shooter::Shooter,
        spawner::Spawner,
        wall::Wall,
    },
    library::resources::Library,
};

#[derive(Serialize, Deserialize)]
pub struct Arena {
    pub width: u32,
    pub height: u32,
    pub objects: Vec<ArenaObject>,
}

impl Arena {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        Ok(ron::from_str::<Arena>(&contents)?)
    }

    pub fn from_world(world: &mut World) -> Self {
        let mut state: SystemState<(
            Query<&Window, With<PrimaryWindow>>,
            Query<&Transform, With<Bumper>>,
            Query<&Transform, With<DeathZone>>,
            Query<(&Transform, &FlipperGroup), With<Flipper>>,
            Query<&Transform, With<Shooter>>,
            Query<&Transform, With<Spawner>>,
            Query<&Wall>,
        )> = SystemState::from_world(world);

        let (q_windows, q_bumpers, q_death_zones, q_flippers, q_shooters, q_spawners, q_walls) =
            state.get(world);

        let (width, height) = if let Some(window) = q_windows.iter().next() {
            (window.physical_width(), window.physical_height())
        } else {
            panic!("No primary window");
        };

        let mut arena = Self {
            width,
            height,
            objects: Vec::new(),
        };

        for transform in q_bumpers.iter() {
            let position = transform.translation.truncate();

            arena.objects.push(ArenaObject::Bumper(BumperObject { position }));
        }
        for transform in q_death_zones.iter() {
            let position = transform.translation.truncate();

            arena.objects.push(ArenaObject::DeathZone(DeathZoneObject { position }));
        }
        for (transform, &group) in q_flippers.iter() {
            let position = transform.translation.truncate();
            let scale = Some(transform.scale.truncate());

            arena.objects.push(ArenaObject::Flipper(FlipperObject {
                group,
                position,
                scale,
            }));
        }
        for transform in q_shooters.iter() {
            let position = transform.translation.truncate();

            arena.objects.push(ArenaObject::Shooter(ShooterObject { position }));
        }
        for transform in q_spawners.iter() {
            let position = transform.translation.truncate();

            arena.objects.push(ArenaObject::Spawner(SpawnerObject { position }));
        }
        for wall in q_walls.iter() {
            arena.objects.push(ArenaObject::Wall(WallObject {
                start: wall.start(),
                end: wall.end(),
            }));
        }

        arena
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let contents = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())?;
        Ok(std::fs::write(path, contents)?)
    }

    pub fn spawn(&self, commands: &mut Commands, library: &Library) {
        for object in self.objects.iter() {
            object.spawn(commands, library);
        }
    }
}
