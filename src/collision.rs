use bevy::prelude::*;

use crate::audio::*;
use crate::cooldown::*;
use crate::velocity::*;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
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

const ASTEROID_COLLISION_SOUND_DURATION: f64 = 1.;

pub fn collision_update(
    time: Res<Time>,

    // For collision sound effects
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut collision_sound_cooldown: Local<Cooldown>,

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

            collision_gameplay_logic(
                &*time,
                &asset_server,
                &audio,
                &mut collision_sound_cooldown,
                obj1,
                obj2,
            );

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

const LETHAL_RELATIVE_VELOCITY_OF_ASTEROID: f32 = 3.;
const LETHAL_RELATIVE_VELOCITY_OF_ASTEROID_SQUARED: f32 =
    LETHAL_RELATIVE_VELOCITY_OF_ASTEROID * LETHAL_RELATIVE_VELOCITY_OF_ASTEROID;

// Objects parameters to collision_gameplay_logic are ordered by collision type
// to reduce the number of permutations
fn collision_gameplay_logic(
    time: &Time,

    // For collision sound effects
    asset_server: &Res<AssetServer>,
    audio: &Res<Audio>,
    collision_sound_cooldown: &mut Local<Cooldown>,

    obj_a: &CollisionData,
    obj_b: &CollisionData,
) {
    // Order the objects by collision type to reduce the number of permutations
    let obj1 = if obj_a.collision.ctype <= obj_b.collision.ctype {
        &obj_a
    } else {
        &obj_b
    };
    let obj2 = if obj_a.collision.ctype <= obj_b.collision.ctype {
        &obj_b
    } else {
        &obj_a
    };

    match (obj1.collision.ctype, obj2.collision.ctype) {
        (CollisionType::Asteroid, CollisionType::Asteroid) => (),
        (CollisionType::Asteroid, CollisionType::Earth) => (),
        (CollisionType::Asteroid, CollisionType::Player) => {
            let relative_velocity = obj1.velocity - obj2.velocity;
            let relative_speed_squared = relative_velocity.length_squared();

            if relative_speed_squared > LETHAL_RELATIVE_VELOCITY_OF_ASTEROID_SQUARED {
                play_sound(asset_server, audio, "audio/SpaceshipCrash.mp3");
            } else if collision_sound_cooldown.over(&time) {
                play_sound(asset_server, audio, "audio/AsteroidCollision.mp3");
                collision_sound_cooldown.reset(&time, ASTEROID_COLLISION_SOUND_DURATION);
            }
        }
        (CollisionType::Earth, CollisionType::Player) => (),

        // All ordered collision permutations have already been handled
        _ => (),
    };
}
