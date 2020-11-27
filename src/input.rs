use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::velocity::*;

pub struct CameraInput;

pub struct PlayerInput;

#[derive(Default)]
pub struct MouseState {
    // mouse_button_event_reader: EventReader<MouseButtonInput>,
    mouse_motion_event_reader: EventReader<MouseMotion>,
    // cursor_moved_event_reader: EventReader<CursorMoved>,
    // mouse_wheel_event_reader: EventReader<MouseWheel>,
}

const ROTATION_RATE: f32 = 0.002;

pub fn mouse_input_update(
    mut state: Local<MouseState>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mut camera_query: Query<(&CameraInput, &mut Transform)>,
    mut player_query: Query<(&PlayerInput, &mut Transform)>,
) {
    for (_, mut player_transform) in player_query.iter_mut() {
        for (_, mut camera_transform) in camera_query.iter_mut() {
            camera_transform.translation = Vec3::zero();

            let quat = player_transform.rotation;
            let rotation_mat = Mat3::from_quat(quat);

            let mouse_motion_events = state.mouse_motion_event_reader.iter(&mouse_motion_events);

            for MouseMotion { delta } in mouse_motion_events {
                let yaw_magnitude = -ROTATION_RATE * delta.y;
                let pitch_magnitude = -ROTATION_RATE * delta.x;

                let yaw = Quat::from_axis_angle(rotation_mat.x_axis, yaw_magnitude);
                let pitch = Quat::from_axis_angle(rotation_mat.y_axis, pitch_magnitude);

                player_transform.rotation = yaw * pitch * player_transform.rotation;
                player_transform.rotation = player_transform.rotation.normalize();
            }

            camera_transform.rotation = player_transform.rotation;

            let forward = player_transform.forward();
            camera_transform.translation = player_transform.translation + forward * 10.0;
        }
    }
}

pub fn keyboard_input_update(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    camera_query: Query<(&CameraInput, &Transform)>,
    mut player_query: Query<(&PlayerInput, &Transform, &mut Velocity)>,
) {
    for (_, transform) in camera_query.iter() {
        let quat = transform.rotation;
        let rotation_mat = Mat3::from_quat(quat);

        // Camera is looking down the negative-z axis
        let forward = -rotation_mat.z_axis;

        for (_, _transform, mut velocity) in player_query.iter_mut() {
            let mut acceleration = 0.0;
            if keyboard_input.pressed(KeyCode::W) {
                acceleration += 10.0;
            }

            if keyboard_input.pressed(KeyCode::S) {
                acceleration -= 10.0;
            }

            let delta_v = forward * acceleration * time.delta_seconds;
            velocity.velocity += delta_v;
        }
    }
}
