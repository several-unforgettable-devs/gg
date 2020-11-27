use bevy::prelude::*;

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
    Won
}

fn player_control_update(
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
    position: Vec3
) {
    commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 2.0, ..Default::default() })),
            material: materials.add(Color::rgb(1., 0.9, 0.9).into()),
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .with(EarthMarker);
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

    add_ship(&mut commands, &mut meshes, &mut materials, Vec3::new(4.0, 0., 0.0));
    add_earth(&mut commands, &mut meshes, &mut materials, Vec3::new(40.0, 0.0, 0.0));
    
}

fn calc_dist_sq(pos1: Vec3,
    pos2: Vec3
) -> f32 {
    let diff = pos1 - pos2;
    diff.dot(diff)
}

fn test_end_condition( mut game_state: ResMut<GameState>,
    player_query: Query<(&PlayerControl, &Transform)>,
    earth_query: Query<(&EarthMarker, &Transform)>, 
) {

    if *game_state != GameState::Running {
        return;
    }

    const VICTORY_DISTANCE: f32 = 10.0;
    let distance_sq:f32 = VICTORY_DISTANCE * VICTORY_DISTANCE;

    for (_, player_transform) in player_query.iter() {
       
        for (_, earth_transform) in earth_query.iter() {

            if calc_dist_sq(player_transform.translation, earth_transform.translation) <= distance_sq {
                // insert end of game message here!!
                println!("YOU WIN!!");
                *game_state = GameState::Won;
            }
        }
    }
}