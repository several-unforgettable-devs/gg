use bevy::prelude::*;

use crate::GameState;

#[derive(Debug, Default, PartialEq, Clone, Copy, Properties)]
pub struct Velocity {
    pub velocity: Vec3,
}

pub fn velocity_update(
    time: Res<Time>,
    game_state: ResMut<crate::GameState>,
    mut query: Query<(&mut Transform, &mut Velocity)>,
) {
    if *game_state != GameState::Running {
        return;
    }

    for (mut transform, velocity) in query.iter_mut() {
        let displacement = velocity.velocity * time.delta_seconds;
        transform.translation += displacement;
    }
}
