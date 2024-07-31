use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::data::{death_zone::DeathZone, pinball::Pinball};

pub fn destroy_balls(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    q_death_zones: Query<Entity, With<DeathZone>>,
    q_balls: Query<Entity, With<Pinball>>,
) {
    let pinballs: Vec<Entity> = q_balls.iter().collect();
    for entity in q_death_zones.iter() {
        for pair in rapier_context.intersection_pairs_with(entity) {
            let other = if pair.0 == entity { pair.1 } else { pair.0 };
            if pinballs.contains(&other) {
                commands.entity(other).despawn_recursive();
                info!("Ball Destroyed!");
            }
        }
    }
}
