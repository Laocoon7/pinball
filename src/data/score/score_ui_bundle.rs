use bevy::prelude::*;

use crate::data::{
    constants::{SCORE_UI_COLOR, SCORE_UI_FONT_SIZE, UI_LAYER},
    score::ScoreUi,
};

#[derive(Bundle)]
pub struct ScoreUiBundle {
    // Ui
    pub text_bundle: TextBundle,

    // Tags
    pub score_ui: ScoreUi,
}

impl ScoreUiBundle {
    pub fn new(position: Vec2) -> Self {
        let mut text_bundle = TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: SCORE_UI_FONT_SIZE,
                    color: SCORE_UI_COLOR,
                    ..Default::default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCORE_UI_FONT_SIZE,
                color: SCORE_UI_COLOR,
                ..Default::default()
            }),
        ]);

        text_bundle.transform.translation = position.extend(UI_LAYER);

        Self {
            text_bundle,
            score_ui: ScoreUi,
        }
    }
}
