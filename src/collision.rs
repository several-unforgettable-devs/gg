use bevy::prelude::*;

use crate::audio::*;
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

struct CollisionData {
    entity: Entity,
    position: Vec3, // Position
    velocity: Vec3, // Velocity
    collision: Collision,
}

pub fn collision_update(
    time: Res<Time>,
    mut query: Query<(Entity, &Transform, &mut Velocity, &Collision)>,
) {
    let objects: Vec<CollisionData> = query
        .iter_mut()
        .map(|(e, t, v, c)| CollisionData {
            entity: e,
            position: t.translation,
            velocity: v.velocity,
            collision: *c,
        })
        .collect();

    let object_count = objects.len();

    for i in 0..object_count {
        let obj1 = &objects[i];

        for j in (i + 1)..object_count {
            let obj2 = &objects[j];

            let displacement = obj2.position - obj1.position;

            let combined_radius = obj1.collision.radius + obj2.collision.radius;
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

            match query.get_component_mut::<Velocity>(obj1.entity) {
                Ok(mut v1) => {
                    let delta_v = impulse / obj1.collision.mass;
                    (*v1).velocity -= delta_v;
                }
                _ => (),
            }
            match query.get_component_mut::<Velocity>(obj2.entity) {
                Ok(mut v2) => {
                    let delta_v = impulse / obj2.collision.mass;
                    (*v2).velocity += delta_v;
                }
                _ => (),
            }
        }
    }
}

fn collision_gameplay_logic() {

    // use CollisionType::*;
    // match (c1, c2) {
    //     (Asteroid, Asteroid) => (),

    // };

    // let relativeVelocity = *v2 - *v1;
    // let relativeSpeed = relativeVelocity.length();
}
