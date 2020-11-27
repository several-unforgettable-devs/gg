use bevy::prelude::*;
use rand::Rng;



mod gravity;
use crate::gravity::*;
mod velocity;
use crate::velocity::*;
mod ship;
use crate::ship::*;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(GameState::Running)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(player_control_update)
        .add_system(test_end_condition)
        .add_system(velocity_update)
        .add_system(gravity_update)
        .run();
}

#[allow(dead_code)]
#[derive(PartialEq)]
enum GameState {
    Running,
    Paused,
    GameOver,
    Won,
}

struct EarthMarker;

fn add_earth(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 2.0,
                ..Default::default()
            })),
            material: materials.add(Color::rgb(0.2, 0.2, 1.0).into()),
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .with(EarthMarker)
        .with(Gravity { mass: 10. })
        .with(Velocity::default());
}

fn add_asteroids(
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
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: rng.gen_range(asteroid_min_radius, asteroid_max_radius),
                    subdivisions: 4,
                })),
                material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
                transform: Transform::from_translation(asteroid_position),
                ..Default::default()
            })
            .with(Gravity { mass: 1. })
            .with(Velocity::default());
    }
}

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        // Light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });

    add_ship(
        commands,
        &mut meshes,
        &mut materials,
        Vec3::new(-40.0, 0., 0.0),
    );

    add_earth(
        commands,
        &mut meshes,
        &mut materials,
        Vec3::new(0.0, 0.0, 0.0),
    );

    add_asteroids(commands, &mut meshes, &mut materials);
}

fn calc_dist_sq(pos1: Vec3, pos2: Vec3) -> f32 {
    let diff = pos1 - pos2;
    diff.dot(diff)
}

fn test_end_condition(
    mut game_state: ResMut<GameState>,
    player_query: Query<(&PlayerControl, &Transform)>,
    earth_query: Query<(&EarthMarker, &Transform)>,
) {
    if *game_state != GameState::Running {
        return;
    }

    const VICTORY_DISTANCE: f32 = 10.0;
    let distance_sq: f32 = VICTORY_DISTANCE * VICTORY_DISTANCE;

    for (_, player_transform) in player_query.iter() {
        for (_, earth_transform) in earth_query.iter() {
            if calc_dist_sq(player_transform.translation, earth_transform.translation)
                <= distance_sq
            {
                // insert end of game message here!!
                println!("YOU WIN!!");
                *game_state = GameState::Won;
            }
        }
    }
}
