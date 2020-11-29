use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::audio::play_sound;
use crate::bullets::*;
use crate::cooldown::*;
use crate::velocity::*;

pub struct CameraInput;

pub struct PlayerInput;

#[derive(Default)]
pub struct MouseState {
    mouse_motion_event_reader: EventReader<MouseMotion>,
}

const ROTATION_RATE: f32 = 0.002;

fn set_cursor_capture(windows: &mut ResMut<Windows>, cursor_captured: bool) {
    match windows.get_primary_mut() {
        Some(window) => {
            window.set_cursor_lock_mode(cursor_captured);
            window.set_cursor_visibility(!cursor_captured);
        }
        _ => (),
    }
}

fn get_cursor_capture(windows: &ResMut<Windows>) -> bool {
    match windows.get_primary() {
        Some(window) => window.cursor_locked(),
        _ => false,
    }
}

pub fn mouse_move_input_update(
    windows: ResMut<Windows>,

    mut state: Local<MouseState>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mut camera_query: Query<(&CameraInput, &mut Transform)>,
    mut player_query: Query<(&PlayerInput, &mut Transform)>,
) {
    for (_, mut player_transform) in player_query.iter_mut() {
        for (_, mut camera_transform) in camera_query.iter_mut() {

            if get_cursor_capture(&windows) {
                let quat = player_transform.rotation;
                let rotation_mat = Mat3::from_quat(quat);

                let mouse_motion_events =
                    state.mouse_motion_event_reader.iter(&mouse_motion_events);

                for MouseMotion { delta } in mouse_motion_events {
                    let yaw_magnitude = -ROTATION_RATE * delta.y;
                    let pitch_magnitude = -ROTATION_RATE * delta.x;

                    let yaw = Quat::from_axis_angle(rotation_mat.x_axis, yaw_magnitude);
                    let pitch = Quat::from_axis_angle(rotation_mat.y_axis, pitch_magnitude);

                    player_transform.rotation = yaw * pitch * player_transform.rotation;
                    player_transform.rotation = player_transform.rotation.normalize();
                }
            }

            camera_transform.rotation = player_transform.rotation;

            let forward = player_transform.forward();
            let up = player_transform.rotation * Vec3::unit_y();
            camera_transform.translation = player_transform.translation + forward * 10.0 + up * 2.5;
        }
    }
}

pub const PLAYER_WEAPON_COOLDOWN_DURATION: f64 = 0.8;

pub const PLAYER_BARREL_LENGTH: f32 = 1.2 * crate::PLAYER_SHIP_RADIUS;

pub fn mouse_button_input_update(
    // Systems needed to spawn bullets
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    // For bullet sound effects
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,

    // For input
    time: Res<Time>,
    mut windows: ResMut<Windows>,
    mouse_button_input: Res<Input<MouseButton>>,

    mut player_weapon_cooldown: Local<Cooldown>,

    mut player_query: Query<(&PlayerInput, &Transform, &mut Velocity)>,
) {
    let lmb_pressed = mouse_button_input.pressed(MouseButton::Left);
    if lmb_pressed && !get_cursor_capture(&windows) {
        set_cursor_capture(&mut windows, true);
        return;
    }

    if lmb_pressed && player_weapon_cooldown.over(&time) {
        for (_, transform, velocity) in player_query.iter_mut() {
            let player_position = transform.translation;
            let player_velocity = velocity.velocity;

            let quat = transform.rotation;
            let rotation_mat = Mat3::from_quat(quat);

            // player is looking down the negative-z axis
            let player_facing = -rotation_mat.z_axis;

            fire_bullet(
                commands,
                &mut meshes,
                &mut materials,
                &asset_server,
                &audio,
                player_position,
                player_velocity,
                player_facing,
                PLAYER_BARREL_LENGTH,
            );
            player_weapon_cooldown.reset(&time, PLAYER_WEAPON_COOLDOWN_DURATION);
        }
    }
}

const THRUSTER_SOUND_DURATION: f64 = 2.5;

pub fn keyboard_input_update(
    // For input
    time: Res<Time>,
    mut windows: ResMut<Windows>,
    key_input: Res<Input<KeyCode>>,

    // For thruster sound effects
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut thruster_sound_cooldown: Local<Cooldown>,

    camera_query: Query<(&CameraInput, &Transform)>,
    mut player_query: Query<(&PlayerInput, &Transform, &mut Velocity)>,
) {
    // Key bindings to return the cursor to the user
    {
        let escape_pressed = key_input.pressed(KeyCode::Escape);

        let lwin_pressed = key_input.pressed(KeyCode::LWin);
        let rwin_pressed = key_input.pressed(KeyCode::RWin);
        let win_pressed = lwin_pressed || rwin_pressed;

        let lalt_pressed = key_input.pressed(KeyCode::LAlt);
        let ralt_pressed = key_input.pressed(KeyCode::RAlt);
        let alt_pressed = lalt_pressed || ralt_pressed;

        let tab_pressed = key_input.pressed(KeyCode::Tab);
        let alt_tab_pressed = alt_pressed && tab_pressed;

        if escape_pressed || win_pressed || alt_tab_pressed {
            set_cursor_capture(&mut windows, false);
        }
    }

    for (_, transform) in camera_query.iter() {
        let quat = transform.rotation;
        let rotation_mat = Mat3::from_quat(quat);

        // Camera is looking down the negative-z axis
        let forward = -rotation_mat.z_axis;

        for (_, _transform, mut velocity) in player_query.iter_mut() {
            let mut acceleration = 0.0;
            if key_input.pressed(KeyCode::W) {
                acceleration += 10.0;
            }

            if key_input.pressed(KeyCode::S) {
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
