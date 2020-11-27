use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

use crate::velocity::*;
use crate::gravity::*;

//struct ShipPlugin;

pub struct PlayerControl;

const ROTATION_RATE: f32 = 0.002;

#[derive(Default)]
pub struct PlayerInputState {
    mouse_motion_event_reader: EventReader<MouseMotion>,
}

pub fn add_ship(
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
        .with(Velocity::default())
        .with_children(|parent| {
            // Camera
            parent.spawn(Camera3dBundle {
                transform: Transform::from_matrix(Mat4::from_rotation_translation(
                    Quat::from_xyzw(-0.3, -1.0, -0.3, 1.0).normalize(),
                    Vec3::new(-18.0, 20.0, 0.0),
                )),
                ..Default::default()
            });
        });
}


pub fn player_control_update(
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