use bevy::math::*;
use bevy::prelude::*;

use crate::velocity::*;

// Entities that exert gravity. Useful for large or mid-sized objects that exert
// a non-negligible amount of gravity
pub struct ExertGravity {
    pub mass: f32,
}

// Entities that receive gravity. Useful for small or mid-sized objects that are likely
// to be moved in relation to others
pub struct ReceiveGravity;

pub const GRAVITATIONAL_CONSTANT: f32 = 25.;

// Objects so close that they should have already collided, or they are the same
// object. Either way this avoids division by zero or a near-zero number
pub const MIN_GRAVITATION_DISTANCE: f32 = 1e-3;

pub const MIN_GRAVITATION_DISTANCE_SQUARED: f32 =
    MIN_GRAVITATION_DISTANCE * MIN_GRAVITATION_DISTANCE;

pub fn gravity_update(
    time: Res<Time>,
    exerting: Query<(&Transform, &ExertGravity)>,
    mut receiving: Query<(&Transform, &mut Velocity, &ReceiveGravity)>,
) {
    for (
        &Transform {
            translation: p1, ..
        },
        &ExertGravity { mass: m1 },
    ) in exerting.iter()
    {
        // Pre-multiply everything that does not depend on the second object,
        // to avoid some work in the inner loop
        let premultiplied_factor = GRAVITATIONAL_CONSTANT * m1 * time.delta_seconds;

        for (
            &Transform {
                translation: p2, ..
            },
            mut v2,
            _,
        ) in receiving.iter_mut()
        {
            let displacement = p1 - p2;
            let dist_squared = displacement.length_squared();
            if dist_squared < MIN_GRAVITATION_DISTANCE_SQUARED {
                return;
            }
            let dist_squared_recip = dist_squared.recip();
            let dist_recip = dist_squared_recip.sqrt();
            let change_in_speed = premultiplied_factor * dist_squared_recip;
            let direction = displacement * dist_recip;
            let delta_v = change_in_speed * direction;
            v2.velocity += delta_v;
        }
    }
}
