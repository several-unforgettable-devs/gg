use bevy::prelude::*;

use crate::velocity::*;

#[derive(Clone, Copy, PartialEq)]
pub enum CollisionType {
    Asteroid,
    Earth,
    Player,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Collision {
    pub mass: f32,
    pub radius: f32,
    pub ctype: CollisionType,
}

const COLLISION_SPRING_CONSTANT: f32 = 2048.;

pub fn collision_update(
    time: Res<Time>,
    mut query: Query<(Entity, &Transform, &mut Velocity, &Collision)>,
) {
    let objects: Vec<(Entity, Vec3, Collision)> = query
        .iter_mut()
        .map(|(e, t, _, c)| (e, t.translation, *c))
        .collect();

    let object_count = objects.len();

    for i in 0..object_count {
        let (
            e1,
            p1,
            Collision {
                mass: m1,
                radius: r1,
                ctype: _c1,
            },
        ) = &objects[i];

        for j in (i + 1)..object_count {
            let (
                e2,
                p2,
                Collision {
                    mass: m2,
                    radius: r2,
                    ctype: _c2,
                },
            ) = &objects[j];

            let displacement = *p2 - *p1;

            let combined_radius = *r1 + *r2;
            let combined_radius_squared = combined_radius * combined_radius;

            let distance_squared = displacement.length_squared();
            if distance_squared > combined_radius_squared {
                // No collision
                continue;
            }

            let distance = distance_squared.sqrt();
            let direction = displacement / distance;

            let compression = combined_radius - distance;

            let force = COLLISION_SPRING_CONSTANT * compression;

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
