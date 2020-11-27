use bevy::prelude::*;
use rand::Rng;

use bevy::input::mouse::MouseMotion;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(GameState::Running)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(player_control_update.system())
        .add_system(test_end_condition.system())
        .add_system(velocity_update.system())
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

#[derive(Default)]
struct PlayerInputState {
    mouse_motion_event_reader: EventReader<MouseMotion>,
}

const ROTATION_RATE: f32 = 0.002;

fn player_control_update(
    mut input_state: Local<PlayerInputState>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerControl, &mut Transform, &mut Velocity)>,
) {
    for (_, mut transform, mut velocity) in query.iter_mut() {
        let quat = transform.rotation;
        let rotation_mat = Mat3::from_quat(quat);
        let forward = rotation_mat.x_axis();

        let mut acceleration = 0.0;
        if keyboard_input.pressed(KeyCode::W) {
            acceleration += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            acceleration -= 1.0;
        }

        let delta_v = forward * acceleration * time.delta_seconds;
        velocity.velocity += delta_v;

        let mouse_motion_events = input_state
            .mouse_motion_event_reader
            .iter(&mouse_motion_events);

        for MouseMotion { delta } in mouse_motion_events {
            let yaw_magnitude = -ROTATION_RATE * delta.y();
            let pitch_magnitude = -ROTATION_RATE * delta.x();

            let yaw = Quat::from_axis_angle(rotation_mat.z_axis(), yaw_magnitude);
            let pitch = Quat::from_axis_angle(rotation_mat.y_axis(), pitch_magnitude);

            transform.rotation = yaw * pitch * transform.rotation;
            transform.rotation = transform.rotation.normalize();
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy, Properties)]
struct Velocity {
    pub velocity: Vec3,
}

fn velocity_update(time: Res<Time>, mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        let displacement = velocity.velocity * time.delta_seconds;
        transform.translation += displacement;
    }
}

struct PlayerControl;

struct EarthMarker;

fn add_ship(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
            material: materials.add(Color::rgb(1., 0.9, 0.9).into()),
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .with(Velocity::default())
        .with(PlayerControl)
        .with_children(|parent| {
            // Camera
            parent.spawn(Camera3dComponents {
                transform: Transform::from_matrix(Mat4::from_rotation_translation(
                    Quat::from_xyzw(-0.3, -1.0, -0.3, 1.0).normalize(),
                    Vec3::new(-18.0, 20.0, 0.0),
                )),
                ..Default::default()
            });
        });
}

fn add_earth(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 2.0,
                ..Default::default()
            })),
            material: materials.add(Color::rgb(1., 0.9, 0.9).into()),
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .with(EarthMarker);
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
        commands.spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: rng.gen_range(asteroid_min_radius, asteroid_max_radius),
                subdivisions: 4,
            })),
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            transform: Transform::from_translation(asteroid_position),
            ..Default::default()
        });
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        // Light
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });

    add_ship(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(-40.0, 0., 0.0),
    );
    add_earth(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(0.0, 0.0, 0.0),
    );
    add_asteroids(&mut commands, &mut meshes, &mut materials);
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
