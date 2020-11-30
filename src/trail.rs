use crate::input::PlayerInput;
use crate::velocity::Velocity;
use crate::GameState;
use bevy::prelude::*;
use rand::Rng;

pub struct MotionTrailPlugin;

// Settings
const TRAIL_SIZE: f32 = 0.025;
const MAX_TRAIL_DIST: f32 = 10.0;
const SPAWN_ANGLE_RADIUS: f32 = std::f32::consts::PI / 2.0;
const NUM_OF_TRAIL_PARTICLES: i32 = 40;

// Internal derived constants
const MAX_TRAIL_DIST_SQ: f32 = MAX_TRAIL_DIST * MAX_TRAIL_DIST;
const SPAWN_DISTANCE: f32 = MAX_TRAIL_DIST * 0.75;

struct TrailObj;

impl Plugin for MotionTrailPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_trail).add_system(update_trail);
    }
    // span 1 quad, re-draw very 60ms
}

fn make_trail_obj(
    commands: &mut Commands,
    trail_material: &Handle<StandardMaterial>,
    trail_mesh: &Handle<Mesh>,
) {
    commands
        .spawn(PbrBundle {
            mesh: trail_mesh.clone(),
            material: trail_material.clone(),
            transform: Transform {
                ..Default::default()
            },
            ..Default::default()
        })
        .with(TrailObj);
}

fn setup_trail(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let trail_material = materials.add(Color::rgb(0.75, 0.75, 1.0).into());
    let trail_mesh = meshes.add(Mesh::from(shape::Cube { size: TRAIL_SIZE }));

    for _ in 1..NUM_OF_TRAIL_PARTICLES {
        make_trail_obj(commands, &trail_material, &trail_mesh);
    }
}

fn build_quat_to_align_vec(original_vector: Vec3, new_direction: Vec3) -> Quat {
    let quat_vector = original_vector.cross(new_direction).normalize();
    let quat_angle = new_direction.x.acos();
    Quat::from_axis_angle(quat_vector, quat_angle)
}

fn build_trail_quat(velocity: &Vec3) -> Quat {
    let velocity_copy = velocity.clone().normalize();
    build_quat_to_align_vec(Vec3::unit_x(), velocity_copy)
}

fn valid_vec3(vec: &Vec3) -> bool {
    !vec.is_nan().any()
}

#[allow(dead_code)]
fn calc_trail_spawn_point(
    player_position: Vec3,
    player_up: Vec3,
    player_velocity_unit: Vec3,
) -> Vec3 {
    let mut rng = rand::thread_rng();
    let projected_radius = SPAWN_DISTANCE * (SPAWN_ANGLE_RADIUS / 2.0).sin();
    let projected_dir = if valid_vec3(&player_velocity_unit) {
        player_velocity_unit
    } else {
        Vec3::zero()
    };
    let x_mag = rng.gen_range(-1.0, 1.0);
    let y_mag = rng.gen_range(-1.0, 1.0);
    let x_dir = player_up.cross(projected_dir).normalize();
    let y_dir = x_dir.cross(projected_dir).normalize();

    let final_pos = player_position
        + projected_dir * MAX_TRAIL_DIST * 0.75
        + x_mag * x_dir * projected_radius
        + y_mag * y_dir * projected_radius;
    final_pos
}

fn update_trail(
    mut trail_query: Query<(&TrailObj, &mut Transform)>,
    player_query: Query<(&PlayerInput, &Transform, &Velocity)>,
    game_state: Res<GameState>,
) {
    if *game_state == GameState::Lost {
        for (_, mut transform) in trail_query.iter_mut() {
            transform.scale = Vec3::one();
        }
    }

    for (_, player_transform, velocity_info) in player_query.iter() {
        let player_up = player_transform.rotation * Vec3::unit_y();
        let player_velocity = velocity_info.velocity.length();
        let player_velocity_dir = velocity_info.velocity.clone().normalize();

        if !valid_vec3(&player_velocity_dir) {
            return;
        }

        let trail_quat = build_trail_quat(&velocity_info.velocity);
        for (_, mut transform) in trail_query.iter_mut() {
            let vector_from_player = transform.translation - player_transform.translation;

            let new_translation = if vector_from_player.length_squared() > MAX_TRAIL_DIST_SQ {
                calc_trail_spawn_point(player_transform.translation, player_up, player_velocity_dir)
            } else {
                transform.translation
            };

            *transform = Transform {
                translation: new_translation,
                rotation: trail_quat,
                scale: Vec3::new(player_velocity, 1.0, 1.0),
            };
        }
    }
}
