use bevy::math::*;
use bevy::prelude::*;

use crate::velocity::*;

#[derive(Copy, Clone, Debug)]
pub struct Gravity {
    pub mass: f32,
}

pub const GRAVITATIONAL_CONSTANT: f32 = 50.;

// Objects so close that they should have already collided, or they are the same
// object. Either way this avoids division by zero or a near-zero number
pub const MIN_GRAVITATION_DISTANCE: f32 = 1e-3;

pub const MIN_GRAVITATION_DISTANCE_SQUARED: f32 =
    MIN_GRAVITATION_DISTANCE * MIN_GRAVITATION_DISTANCE;

pub fn gravity_update(
    time: Res<Time>,
    mut query: Query<(Entity, &Transform, &mut Velocity, &Gravity)>,
) {
    let objects: Vec<(Entity, Vec3, Gravity)> = query
        .iter_mut()
        .map(|(e, t, _, g)| (e, t.translation, *g))
        .collect();

    let object_count = objects.len();

    for i in 0..object_count {
        let (e1, p1, Gravity { mass: m1 }) = &objects[i];

        for j in (i + 1)..object_count {
            let (e2, p2, Gravity { mass: m2 }) = &objects[j];

            let displacement = *p1 - *p2;
            let dist_squared = displacement.length_squared();
            if dist_squared < MIN_GRAVITATION_DISTANCE_SQUARED {
                continue;
            }
            let dist_squared_recip = dist_squared.recip();
            let dist_recip = dist_squared_recip.sqrt();
            let direction = displacement * dist_recip;

            let force = GRAVITATIONAL_CONSTANT * m1 * m2 * dist_squared_recip;
            let impulse_magnitude = force * time.delta_seconds;

            let impulse = impulse_magnitude * direction;

            match query.get_component_mut::<Velocity>(*e1) {
                Ok(mut v1) => {
                    let delta_v = impulse / *m1;
                    (*v1).velocity -= delta_v;
                }
                _ => (),
            }
            match query.get_component_mut::<Velocity>(*e2) {
                Ok(mut v2) => {
                    let delta_v = impulse / *m2;
                    (*v2).velocity += delta_v;
                }
                _ => (),
            }
        }
    }
}
