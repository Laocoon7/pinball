use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Wall {
    /// This is just to store what was first created in the bundle
    start: Vec2,
    /// This is just to store what was first created in the bundle
    end: Vec2,
}

impl Wall {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> Vec2 {
        self.start
    }

    pub fn end(&self) -> Vec2 {
        self.end
    }
}
