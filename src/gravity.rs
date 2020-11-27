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
    for (e1, p1, Gravity { mass: m1 }) in objects.iter() {
        // Pre-multiply everything that does not depend on the second object,
        // to avoid some work in the inner loop
        let premultiplied_factor = GRAVITATIONAL_CONSTANT * m1 * time.delta_seconds;

        for (e2, p2, _) in objects.iter() {
            if e2 == e1 {
                continue;
            }
            let displacement = *p1 - *p2;
            let dist_squared = displacement.length_squared();
            if dist_squared < MIN_GRAVITATION_DISTANCE_SQUARED {
                continue;
            }
            match query.get_component_mut::<Velocity>(*e2) {
                Ok(mut v2) => {
                    let dist_squared_recip = dist_squared.recip();
                    let dist_recip = dist_squared_recip.sqrt();
                    let change_in_speed = premultiplied_factor * dist_squared_recip;
                    let direction = displacement * dist_recip;
                    let delta_v = change_in_speed * direction;
                    (*v2).velocity += delta_v;
                }
                _ => (),
            }
        }
    }
}
