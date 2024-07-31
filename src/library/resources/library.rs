use bevy::{prelude::*, sprite::Anchor};
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource, Reflect)]
#[reflect(Resource)]
pub struct Library {
    #[asset(path = "images/simple/ball.png")]
    pub ball: Handle<Image>,

    #[asset(path = "images/simple/bumper.png")]
    pub bumper: Handle<Image>,

    #[asset(path = "images/simple/flipper.png")]
    pub flipper: Handle<Image>,

    #[asset(path = "images/simple/slingshot.png")]
    pub slingshot: Handle<Image>,

    #[asset(path = "images/simple/square.png")]
    pub square: Handle<Image>,

    // #[asset(path = "images/wall_32x32.png")]
    #[cfg_attr(feature = "dev", asset(path = "images/wall_32x32_debug.png"))]
    #[cfg_attr(not(feature = "dev"), asset(path = "images/wall_32x32.png"))]
    pub wall: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 2, rows = 1))]
    pub wall_layout: Handle<TextureAtlasLayout>,
}

impl Library {
    pub fn ball(&self, transform: Transform, custom_size: Option<Vec2>) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                custom_size,
                anchor: Anchor::Center,
                ..Default::default()
            },
            transform,
            texture: self.ball.clone(),
            ..Default::default()
        }
    }
    pub fn bumper(
        &self,
        transform: Transform,
        custom_size: Option<Vec2>,
        color: Option<Color>,
    ) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                color: color.unwrap_or_default(),
                custom_size,
                anchor: Anchor::Center,
                ..Default::default()
            },
            transform,
            texture: self.bumper.clone(),
            ..Default::default()
        }
    }
    pub fn flipper_left(
        &self,
        transform: Transform,
        custom_size: Option<Vec2>,
        color: Option<Color>,
    ) -> SpriteBundle {
        // Can't perform float math in const fn
        const X: f32 = 40.0;
        const Y: f32 = 45.0;
        const WIDTH: f32 = 328.0;
        const HEIGHT: f32 = 90.0;
        const CENTER_X: f32 = WIDTH / 2.0;
        const CENTER_Y: f32 = HEIGHT / 2.0;
        const TRANSLATED_X: f32 = X - CENTER_X;
        const TRANSLATED_Y: f32 = Y - CENTER_Y;
        const NORMALIZED_X: f32 = TRANSLATED_X / WIDTH;
        const NORMALIZED_Y: f32 = TRANSLATED_Y / HEIGHT;
        const ANCHOR_X: f32 = NORMALIZED_X; // * 0.5;
        const ANCHOR_Y: f32 = NORMALIZED_Y; // * 0.5;

        SpriteBundle {
            sprite: Sprite {
                color: color.unwrap_or_default(),
                flip_x: true,
                custom_size,
                anchor: Anchor::Custom(Vec2 {
                    x: ANCHOR_X,
                    y: ANCHOR_Y,
                }),
                ..Default::default()
            },
            transform,
            texture: self.flipper.clone(),
            ..Default::default()
        }
    }
    pub fn flipper_right(
        &self,
        transform: Transform,
        custom_size: Option<Vec2>,
        color: Option<Color>,
    ) -> SpriteBundle {
        // Can't perform float math in const fn
        const X: f32 = 280.0;
        const Y: f32 = 45.0;
        const WIDTH: f32 = 328.0;
        const HEIGHT: f32 = 90.0;
        const CENTER_X: f32 = WIDTH / 2.0;
        const CENTER_Y: f32 = HEIGHT / 2.0;
        const TRANSLATED_X: f32 = X - CENTER_X;
        const TRANSLATED_Y: f32 = Y - CENTER_Y;
        const NORMALIZED_X: f32 = TRANSLATED_X / WIDTH;
        const NORMALIZED_Y: f32 = TRANSLATED_Y / HEIGHT;
        const ANCHOR_X: f32 = NORMALIZED_X; // * 0.5;
        const ANCHOR_Y: f32 = NORMALIZED_Y; // * 0.5;

        SpriteBundle {
            sprite: Sprite {
                color: color.unwrap_or_default(),
                custom_size,
                anchor: Anchor::Custom(Vec2 {
                    x: ANCHOR_X,
                    y: ANCHOR_Y,
                }),
                ..Default::default()
            },
            transform,
            texture: self.flipper.clone(),
            ..Default::default()
        }
    }
    pub fn slingshot_left(
        &self,
        transform: Transform,
        custom_size: Option<Vec2>,
        color: Option<Color>,
    ) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                color: color.unwrap_or_default(),
                flip_x: true,
                custom_size,
                anchor: Anchor::Center,
                ..Default::default()
            },
            transform,
            texture: self.slingshot.clone(),
            ..Default::default()
        }
    }
    pub fn slingshot_right(
        &self,
        transform: Transform,
        custom_size: Option<Vec2>,
        color: Option<Color>,
    ) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                color: color.unwrap_or_default(),
                custom_size,
                anchor: Anchor::Center,
                ..Default::default()
            },
            transform,
            texture: self.slingshot.clone(),
            ..Default::default()
        }
    }
    pub fn square(&self, transform: Transform, custom_size: Option<Vec2>) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                custom_size,
                anchor: Anchor::Center,
                ..Default::default()
            },
            transform,
            texture: self.square.clone(),
            ..Default::default()
        }
    }
    pub fn wall_start(
        &self,
        transform: Transform,
        custom_size: Option<Vec2>,
        color: Option<Color>,
    ) -> (SpriteBundle, TextureAtlas) {
        (
            SpriteBundle {
                sprite: Sprite {
                    color: color.unwrap_or_default(),
                    custom_size,
                    anchor: Anchor::Center,
                    ..Default::default()
                },
                transform,
                texture: self.wall.clone(),
                ..Default::default()
            },
            TextureAtlas {
                layout: self.wall_layout.clone(),
                index: 0,
            },
        )
    }
    pub fn wall_middle(
        &self,
        transform: Transform,
        custom_size: Option<Vec2>,
        color: Option<Color>,
    ) -> (SpriteBundle, TextureAtlas) {
        (
            SpriteBundle {
                sprite: Sprite {
                    color: color.unwrap_or_default(),
                    custom_size,
                    anchor: Anchor::Center,
                    ..Default::default()
                },
                transform,
                texture: self.wall.clone(),
                ..Default::default()
            },
            TextureAtlas {
                layout: self.wall_layout.clone(),
                index: 1,
            },
        )
    }
}
