use bevy::{color::palettes::css, prelude::*};

// Window
pub const WINDOW_WIDTH: f32 = 936.0;
pub const WINDOW_WIDTH_HALF: f32 = WINDOW_WIDTH * 0.5;
pub const WINDOW_HEIGHT: f32 = 1260.0;
pub const WINDOW_HEIGHT_HALF: f32 = WINDOW_HEIGHT * 0.5;
pub const WINDOW_LEFT: f32 = -WINDOW_WIDTH_HALF;
pub const WINDOW_RIGHT: f32 = WINDOW_WIDTH_HALF;
pub const WINDOW_BOTTOM: f32 = -WINDOW_HEIGHT_HALF;
pub const WINDOW_TOP: f32 = WINDOW_HEIGHT_HALF;

// Layers
pub const UI_LAYER: f32 = 1.0;
pub const DEFAULT_LAYER: f32 = 0.0;

// Arena
pub const ARENA_WIDTH: f32 = WINDOW_WIDTH - (WALL_THICKNESS * 2.0);
pub const ARENA_WIDTH_HALF: f32 = ARENA_WIDTH * 0.5;
pub const ARENA_HEIGHT: f32 = WINDOW_HEIGHT - (WALL_THICKNESS * 2.0);
pub const ARENA_HEIGHT_HALF: f32 = ARENA_HEIGHT * 0.5;
pub const ARENA_LEFT: f32 = -ARENA_WIDTH_HALF;
pub const ARENA_RIGHT: f32 = ARENA_WIDTH_HALF;
pub const ARENA_BOTTOM: f32 = -ARENA_HEIGHT_HALF;
pub const ARENA_TOP: f32 = ARENA_HEIGHT_HALF;

// Walls
pub const WALL_THICKNESS: f32 = 32.0;
pub const WALL_THICKNESS_HALF: f32 = WALL_THICKNESS * 0.5;
pub const WALL_RESTITUTION: f32 = 0.4;

// Ball
pub const BALL_SIZE: f32 = 76.0;
pub const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
pub const BALL_SCALE: f32 = 0.5;
pub const BALL_MASS: f32 = 10.0;
pub const BALL_RESTITUTION: f32 = 0.4;

// Bumper
pub const BUMPER_SIZE: f32 = 129.0;
pub const BUMPER_SIZE_HALF: f32 = BUMPER_SIZE * 0.5;
pub const BUMPER_SCALE: f32 = 0.5;
pub const BUMPER_RESTITUTION: f32 = 2.0;

// Flipper
pub const FLIPPER_SPEED: f32 = 360.0;
pub const FLIPPER_MAX_ROTATION: f32 = 45.0;
pub const FLIPPER_MIN_ROTATION: f32 = -20.00;
pub const FLIPPER_RESTITUTION: f32 = 0.5;

// DeathZone
pub const DEATH_ZONE_WIDTH: f32 = 200.0;
pub const DEATH_ZONE_WIDTH_HALF: f32 = DEATH_ZONE_WIDTH * 0.5;
pub const DEATH_ZONE_HEIGHT: f32 = 10.0;
pub const DEATH_ZONE_HEIGHT_HALF: f32 = DEATH_ZONE_HEIGHT * 0.5;

// Shooter
pub const SHOOTER_SIZE: f32 = 129.0;
pub const SHOOTER_SIZE_HALF: f32 = SHOOTER_SIZE * 0.5;
pub const SHOOTER_SCALE: f32 = 1.0;
pub const SHOOTER_RESTITUTION: f32 = 10.0;

// UI
pub const SCORE_UI_FONT_SIZE: f32 = 20.0;
pub const SCORE_UI_COLOR: Color = Color::Srgba(css::RED);
