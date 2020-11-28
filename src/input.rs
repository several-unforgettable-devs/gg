use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::audio::play_sound;
use crate::cooldown::*;
use crate::velocity::*;

#[derive(Copy, Clone, Debug)]
pub struct CameraInput;

#[derive(Copy, Clone, Debug)]
pub struct PlayerInput;

#[derive(Default)]
pub struct MouseState {
    // mouse_button_event_reader: EventReader<MouseButtonInput>,
    mouse_motion_event_reader: EventReader<MouseMotion>,
    // cursor_moved_event_reader: EventReader<CursorMoved>,
    // mouse_wheel_event_reader: EventReader<MouseWheel>,
}

const ROTATION_RATE: f32 = 0.002;

// type CameraQuery = Query<(&CameraInput, &mut Transform)>;
// type PlayerQuery = Query<(&PlayerInput, &mut Transform)>;

pub fn mouse_input_update(
    mut state: Local<MouseState>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    // mut queries: QuerySet<(CameraQuery, PlayerQuery)>,
    mut queries: QuerySet<
            (
                Query<(&PlayerInput, &mut Transform)>,
                Query<(&CameraInput, &mut Transform)>)
            >
    // mut camera_query: ,
    // mut player_query: Query<(&PlayerInput, &mut Transform)>,
) {
    let mut player_transform_copy = Transform::from_translation(Vec3::zero());
    for (_, mut player_transform) in queries.q1_mut().iter_mut() {
        player_transform.translation = Vec3::zero();

        let quat = player_transform.rotation;
        let rotation_mat = Mat3::from_quat(quat);

        let mouse_motion_events = state.mouse_motion_event_reader.iter(&mouse_motion_events);

        for MouseMotion { delta } in mouse_motion_events {
            let yaw_magnitude = -ROTATION_RATE * delta.y;
            let pitch_magnitude = -ROTATION_RATE * delta.x;

            let yaw = Quat::from_axis_angle(rotation_mat.x_axis, yaw_magnitude);
            let pitch = Quat::from_axis_angle(rotation_mat.y_axis, pitch_magnitude);

            player_transform.rotation = yaw * pitch * player_transform.rotation;
        }
        player_transform.rotation = player_transform.rotation.normalize();

        player_transform_copy = player_transform.clone();
    }

    let player_transform = &player_transform_copy;
    for (_, mut camera_transform) in queries.q1_mut().iter_mut() {

        camera_transform.rotation = player_transform.rotation;

        let forward = player_transform.forward();
        let up = player_transform.rotation * Vec3::unit_y();
        camera_transform.translation = player_transform.translation + forward * 10.0 + up * 2.5;
    }
}

// pub fn mouse_input_update_player(
//     mut state: Local<MouseState>,
//     mouse_motion_events: Res<Events<MouseMotion>>,
//     camera_query: Query<(&CameraInput, &Transform)>,
//     mut player_query: Query<(&PlayerInput, &mut Transform)>,
// ) {
// }

const THRUSTER_SOUND_DURATION: f64 = 2.5;

pub fn keyboard_input_update(
    // For input
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,

    // For thruster sound effects
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut thruster_sound_cooldown: Local<Cooldown>,

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

            if acceleration != 0. && thruster_sound_cooldown.over(&time) {
                play_sound(
                    &asset_server,
                    &audio,
                    "audio/AmbientThrusterLoopShortened.mp3",
                );
                thruster_sound_cooldown.reset(&time, THRUSTER_SOUND_DURATION);
            }

            let delta_v = forward * acceleration * time.delta_seconds;
            velocity.velocity += delta_v;
        }
    }
}
