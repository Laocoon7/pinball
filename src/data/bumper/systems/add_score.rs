use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::data::{bumper::Bumper, pinball::Pinball, score::resources::Score};

pub fn add_score(
    rapier_context: Res<RapierContext>,
    mut score: ResMut<Score>,
    q_bumpers: Query<(Entity, &Bumper)>,
    q_balls: Query<Entity, With<Pinball>>,
) {
    let pinballs: Vec<Entity> = q_balls.iter().collect();
    for (entity, bumper) in q_bumpers.iter() {
        for pair in rapier_context.contact_pairs_with(entity) {
            let other = if pair.collider1() == entity { pair.collider2() } else { pair.collider1() };
            if pinballs.contains(&other) {
                **score += bumper.score;
                info!("Score: {}", **score);
            }
        }
    }
}
