use bevy::prelude::*;

use crate::velocity::*;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Boid {
}

#[derive(Clone, Copy, PartialEq)]
struct BoidData {
    entity: Entity,
    position: Vec3,
    velocity: Vec3,
    boid: Boid,
}

const BOID_COHERENCE_DISTANCE_SQUARED: f32 = 500.0;
const BOID_COHERENCE_FACTOR: f32 = 0.1;
const BOID_AVOIDANCE_DISTANCE_SQUARED: f32 = 100.0;
const BOID_AVOIDANCE_FACTOR: f32 = 0.2;
const BOID_VELOCITY_MATCHING_FACTOR: f32 = 0.1;
const BOID_SPEED_LIMIT: f32 = 10.0;

pub fn boid_update(
    time: Res<Time>,
    mut query: Query<(Entity, &Transform, &mut Velocity, &Boid)>,
) {
    let mut boids: Vec<BoidData> = query
        .iter_mut()
        .map(|(e, t, v, b)| BoidData {
            entity: e,
            position: t.translation,
            velocity: v.velocity,
            boid: *b,
        })
        .collect();
    
    let boid_count = boids.len();

    for i in 0..boid_count {
        cohere_boid_flock(boids.as_mut_slice(), i, boid_count, &*time);
        avoid_neightbour_boids(boids.as_mut_slice(), i, boid_count, &*time);
        match_neighbour_boids_velocity(boids.as_mut_slice(), i, boid_count, &*time);
        limit_boid_speed(boids.as_mut_slice(), i, &*time);

        match query.get_component_mut::<Velocity>(boids[i].entity) {
            Ok(mut ent) => {
                (*ent).velocity = boids[i].velocity;
            }
            _ => (),
        }
    }
}

fn cohere_boid_flock(
    boids: &mut [BoidData],
    boid_index: usize,
    boid_count: usize,
    time: &Time,
) {
    let mut boid_flock_center = Vec3::zero();
    let mut boid_flock_count = 0;
    for i in 0..boid_count {
        let boids_distance = boids[boid_index].position - boids[i].position;
        if boids_distance.length_squared() < BOID_COHERENCE_DISTANCE_SQUARED {
            boid_flock_center += boids[i].position;
            boid_flock_count += 1;
        }
    }

    if boid_flock_count > 0 {
        boid_flock_center *= 1.0 / boid_flock_count as f32;
        boids[boid_index].velocity += (boid_flock_center - boids[boid_index].position) * BOID_COHERENCE_FACTOR * time.delta_seconds;
    }
}

fn avoid_neightbour_boids(
    boids: &mut [BoidData],
    boid_index: usize,
    boid_count: usize,
    time: &Time,
) {
    let mut accumulated_avoidance = Vec3::zero();
    for i in 0..boid_count {
        if i != boid_index {
            let boids_distance = boids[boid_index].position - boids[i].position;
            if boids_distance.length_squared() < BOID_AVOIDANCE_DISTANCE_SQUARED {
                accumulated_avoidance += boids_distance;
            }
        }
    }

    boids[boid_index].velocity += accumulated_avoidance * BOID_AVOIDANCE_FACTOR * time.delta_seconds;
}

fn match_neighbour_boids_velocity(
    boids: &mut [BoidData],
    boid_index: usize,
    boid_count: usize,
    time: &Time,
) {
    let mut boid_flock_velocity = Vec3::zero();
    let mut boid_flock_count = 0;
    for i in 0..boid_count {
        let boids_distance = boids[boid_index].position - boids[i].position;
        if boids_distance.length_squared() < BOID_COHERENCE_DISTANCE_SQUARED {
            boid_flock_velocity += boids[i].velocity;
            boid_flock_count += 1;
        }
    }

    if boid_flock_count > 0 {
        boid_flock_velocity *= 1.0 / boid_flock_count as f32;
        boids[boid_index].velocity += (boid_flock_velocity - boids[boid_index].velocity) * BOID_VELOCITY_MATCHING_FACTOR * time.delta_seconds;
    }
}


fn limit_boid_speed(
    boids: &mut [BoidData],
    boid_index: usize,
    time: &Time,
) {
    if boids[boid_index].velocity.length_squared() > BOID_SPEED_LIMIT * BOID_SPEED_LIMIT {
        boids[boid_index].velocity = boids[boid_index].velocity.normalize() * BOID_SPEED_LIMIT * time.delta_seconds;
    }
}