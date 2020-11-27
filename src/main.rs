use bevy::prelude::*;

use bevy::input::mouse::{MouseMotion};

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(player_control_update.system())
        .add_system(velocity_update.system())
        .run();
}

#[derive(Default)]
struct PlayerInputState {
    mouse_motion_event_reader: EventReader<MouseMotion>,
}


fn player_control_update(
    mut input_state: Local<PlayerInputState>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerControl, &mut Transform, &mut  Velocity)>,
) {
    for (_, transform, mut velocity) in query.iter_mut() {

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
    }
    for event in input_state.mouse_motion_event_reader.iter(&mouse_motion_events) {
        println!("{:?}", event);
    }
}
#[derive(Debug, Default, PartialEq, Clone, Copy, Properties)]
struct Velocity{
    pub velocity: Vec3,
}

fn velocity_update(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut  Velocity)>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        let displacement = velocity.velocity * time.delta_seconds;
        transform.translation += displacement;
    }
}

struct PlayerControl;

fn add_tank(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
            material: materials.add(Color::rgb(1., 0.9, 0.9).into()),
            transform: Transform::from_translation(Vec3::new(4., 0., 4.)),
            ..Default::default()
        })
        .with(Velocity::default())
        .with(PlayerControl)
        .with_children(|parent| {

            // Camera
            parent.spawn(Camera3dComponents {
                transform: Transform::from_matrix(Mat4::from_rotation_translation(
                    Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                    Vec3::new(-7.0, 20.0, 4.0),
                )),
                ..Default::default()
            });
        });
}

fn add_earth(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>
) {
    commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 2.0, ..Default::default() })),
            material: materials.add(Color::rgb(1., 0.9, 0.9).into()),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..Default::default()
        });
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

    add_tank(&mut commands, &mut meshes, &mut materials);
    add_earth(&mut commands, &mut meshes, &mut materials);
}
