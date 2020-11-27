use bevy::prelude::*;

#[derive(Debug, Default, PartialEq, Clone, Copy, Properties)]
pub struct Velocity {
    pub velocity: Vec3,
}

pub fn velocity_update(time: Res<Time>, mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        let displacement = velocity.velocity * time.delta_seconds;
        transform.translation += displacement;
    }
}
