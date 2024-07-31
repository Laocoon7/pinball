use bevy::prelude::*;

use crate::{
    data::{
        bumper::BumperBundle,
        constants::{
            ARENA_BOTTOM, DEATH_ZONE_WIDTH_HALF, WALL_THICKNESS, WALL_THICKNESS_HALF, WINDOW_BOTTOM,
            WINDOW_LEFT, WINDOW_RIGHT, WINDOW_TOP,
        },
        death_zone::DeathZoneBundle,
        flipper::FlipperBundle,
        pinball::PinballBundle,
        shooter::ShooterBundle,
        wall::WallBundle,
    },
    library::resources::Library,
};

pub fn build_arena(mut commands: Commands, library: Res<Library>) {
    // Build the 4 edges of the screen
    // BottomLeft -> BottomRight
    commands.spawn(WallBundle::new(
        &library,
        Vec2::new(WINDOW_LEFT, WINDOW_BOTTOM + WALL_THICKNESS_HALF),
        Vec2::new(WINDOW_RIGHT, WINDOW_BOTTOM + WALL_THICKNESS_HALF),
    ));
    // BottomRight -> TopRight
    commands.spawn(WallBundle::new(
        &library,
        Vec2::new(WINDOW_RIGHT - WALL_THICKNESS_HALF, WINDOW_BOTTOM),
        Vec2::new(WINDOW_RIGHT - WALL_THICKNESS_HALF, WINDOW_TOP),
    ));
    // TopRight -> TopLeft
    commands.spawn(WallBundle::new(
        &library,
        Vec2::new(WINDOW_RIGHT, WINDOW_TOP - WALL_THICKNESS_HALF),
        Vec2::new(WINDOW_LEFT, WINDOW_TOP - WALL_THICKNESS_HALF),
    ));
    // TopLeft -> BottomLeft
    commands.spawn(WallBundle::new(
        &library,
        Vec2::new(WINDOW_LEFT + WALL_THICKNESS_HALF, WINDOW_TOP),
        Vec2::new(WINDOW_LEFT + WALL_THICKNESS_HALF, WINDOW_BOTTOM),
    ));

    // Add Slopes at the bottom
    // BottomLeft -> BottomCenter
    commands.spawn(WallBundle::new(
        &library,
        Vec2::new(
            WINDOW_LEFT + WALL_THICKNESS_HALF,
            WINDOW_BOTTOM + WALL_THICKNESS_HALF + WALL_THICKNESS,
        ),
        Vec2::new(-DEATH_ZONE_WIDTH_HALF, WINDOW_BOTTOM + WALL_THICKNESS_HALF),
    ));
    // BottomRight -> BottomCenter
    commands.spawn(WallBundle::new(
        &library,
        Vec2::new(
            WINDOW_RIGHT - WALL_THICKNESS_HALF,
            WINDOW_BOTTOM + WALL_THICKNESS_HALF + WALL_THICKNESS,
        ),
        Vec2::new(DEATH_ZONE_WIDTH_HALF, WINDOW_BOTTOM + WALL_THICKNESS_HALF),
    ));

    // Add a pinball
    commands.spawn(PinballBundle::new(&library, Vec2::new(396.0, 350.0)));

    // Add a bumper
    commands.spawn(BumperBundle::new(&library, Vec2::new(-350.0, 350.0)));
    commands.spawn(BumperBundle::new(&library, Vec2::new(-150.0, 450.0)));
    commands.spawn(BumperBundle::new(&library, Vec2::new(50.0, 350.0)));

    // Add flippers
    commands.spawn(FlipperBundle::new_left(
        &library,
        Vec2::new(-250.0, -350.0),
        Some(Vec2::new(0.75, 0.75)),
    ));
    commands.spawn(FlipperBundle::new_right(
        &library,
        Vec2::new(250.0, -350.0),
        Some(Vec2::new(0.75, 0.75)),
    ));

    // Add DeathZone
    commands.spawn(DeathZoneBundle::new(Vec2::new(0.0, ARENA_BOTTOM)));

    // Shooter + Wall
    commands.spawn(WallBundle::new(
        &library,
        Vec2::new(380.0, -50.0),
        Vec2::new(380.0, 400.0),
    ));
    commands.spawn(ShooterBundle::new(&library, Vec2::new(396.0, -50.0)));

    // Random Bumpers / Wall

    commands.spawn(BumperBundle::new(&library, Vec2::new(-150.0, -150.0)));
    commands.spawn(WallBundle::new(
        &library,
        Vec2::new(-250.0, -300.0),
        Vec2::new(-250.0, -150.0),
    ));
    commands.spawn(WallBundle::new(
        &library,
        Vec2::new(-250.0, -150.0),
        Vec2::new(-350.0, 200.0),
    ));
    commands.spawn(WallBundle::new(
        &library,
        Vec2::new(250.0, -300.0),
        Vec2::new(250.0, -150.0),
    ));
    commands.spawn(WallBundle::new(
        &library,
        Vec2::new(250.0, -150.0),
        Vec2::new(380.0, -50.0),
    ));
    commands.spawn(BumperBundle::new(&library, Vec2::new(-0.0, -400.0)));
}
