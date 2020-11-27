use bevy::{
    //input::gamepad::{Gamepad, GamepadButton, GamepadEvent, GamepadEventType},
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
    //utils::HashSet,
};

use rand::Rng;

mod gravity;
use crate::gravity::*;
mod velocity;
use crate::velocity::*;
mod debug;
use debug::{change_text_system, infotext_system};

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(GameState::Running)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(infotext_system)
        .add_system(player_control_update)
        .add_system(test_end_condition)
        .add_system(velocity_update)
        .add_system(gravity_update)
        .add_system(change_text_system)
        .add_system(mouse_input_update)
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

#[derive(Default)]
struct MouseState {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    mouse_motion_event_reader: EventReader<MouseMotion>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
    mouse_wheel_event_reader: EventReader<MouseWheel>,
}

struct CameraState;

const ROTATION_RATE: f32 = 0.002;

fn mouse_input_update(
    mut state: Local<MouseState>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
    mut query: Query<(&CameraState, &mut Transform)>,
) {
    for (cameraState, mut transform) in query.iter_mut() {
        let quat = transform.rotation;
        let rotation_mat = Mat3::from_quat(quat);

        let mouse_motion_events = state.mouse_motion_event_reader.iter(&mouse_motion_events);

        for MouseMotion { delta } in mouse_motion_events {
            let roll_magnitude = -ROTATION_RATE * delta.y;
            let pitch_magnitude = -ROTATION_RATE * delta.x;

            let pitch = Quat::from_axis_angle(rotation_mat.y_axis, pitch_magnitude);
            let roll = Quat::from_axis_angle(rotation_mat.x_axis, roll_magnitude);

            transform.rotation = pitch * roll * transform.rotation;
            transform.rotation = transform.rotation.normalize();
        }
    }
}

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
        let forward = rotation_mat.x_axis;

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
            let yaw_magnitude = -ROTATION_RATE * delta.y;
            let pitch_magnitude = -ROTATION_RATE * delta.x;

            let yaw = Quat::from_axis_angle(rotation_mat.z_axis, yaw_magnitude);
            let pitch = Quat::from_axis_angle(rotation_mat.y_axis, pitch_magnitude);

            transform.rotation = yaw * pitch * transform.rotation;
            transform.rotation = transform.rotation.normalize();
        }
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
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
            material: materials.add(Color::rgb(1., 0.9, 0.9).into()),
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .with(PlayerControl)
        .with(Gravity { mass: 1. })
        .with(Velocity::default());
}

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
            material: materials.add(Color::rgb(1., 0.9, 0.9).into()),
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
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_scene(asset_server.load("models/skybox/skybox.gltf"))
        // Light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(5.0, 10.0, 10.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .with(CameraState);

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
