use bevy::prelude::*;
use rand::Rng;

use crate::collision::*;
use crate::gravity::*;
use crate::velocity::*;

pub fn add_asteroids(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();

    let asteroid_density = 30.0;
    let asteroid_max_spacing = 0.7;
    let asteroid_max_spawn_distance = 150.0;
    let asteroid_min_radius = 0.9;
    let asteroid_max_radius = 3.0;
    let asteroid_relative_tangential_speed = 150.0;

    let asteroids_per_axis: i32 = (asteroid_max_spawn_distance / asteroid_density) as i32;
    let total_asteroids = asteroids_per_axis.pow(3);
    let asteroid_max_spawn_radius = asteroid_max_spawn_distance / 2.0;

    let asteroid_min_offset = -asteroid_max_spacing * asteroid_density / 2.0;
    let asteroid_max_offset = asteroid_max_spacing * asteroid_density / 2.0;

    for x in 0..total_asteroids {
        let asteroid_offset = Vec3::new(
            rng.gen_range(asteroid_min_offset, asteroid_max_offset),
            rng.gen_range(asteroid_min_offset, asteroid_max_offset),
            rng.gen_range(asteroid_min_offset, asteroid_max_offset),
        );
        let mut asteroid_position = Vec3::new(
            (x % asteroids_per_axis) as f32 * asteroid_density - asteroid_max_spawn_radius,
            ((x / asteroids_per_axis) % asteroids_per_axis) as f32 * asteroid_density
                - asteroid_max_spawn_radius,
            ((x / asteroids_per_axis.pow(2)) % asteroids_per_axis) as f32 * asteroid_density
                - asteroid_max_spawn_radius,
        );
        asteroid_position += asteroid_offset;

        let asteroid_rotation: Vec3 = asteroid_position.normalize().cross(
            Vec3::new(
                rng.gen_range(0.0, 1.0),
                rng.gen_range(0.0, 1.0),
                rng.gen_range(0.0, 1.0),
            )
            .normalize(),
        );

        let asteroid_radius = rng.gen_range(asteroid_min_radius, asteroid_max_radius);

        let asteroid_mass = asteroid_radius * asteroid_radius;

        let asteroid_distance = asteroid_position.length();
        // let asteroid_max_spawn_distance_squared = asteroid_max_spawn_distance * asteroid_max_spawn_distance;

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: asteroid_radius,
                    subdivisions: 4,
                })),
                material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
                transform: Transform::from_translation(asteroid_position),
                ..Default::default()
            })
            .with(Gravity {
                mass: asteroid_mass,
            })
            .with(Collision {
                mass: asteroid_mass,
                radius: asteroid_radius,
                etype: EntityType::Asteroid,
            })
            .with(Velocity {
                velocity: asteroid_rotation
                    * asteroid_relative_tangential_speed
                    / asteroid_distance.sqrt(),
            });
    }
}
