use bevy::prelude::*;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(player_control_update.system())
        .run();
}


fn player_control_update(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerControl, &mut Transform, &mut  Velocity)>,
) {
    for (_, mut transform, mut _velocity) in query.iter_mut() {
        let mut direction = 0.0;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += 1.0;
        }

        // move the paddle horizontally
        let new_z = transform.translation.z() + (time.delta_seconds * direction * 10.0);

        // bound the paddle within the walls
        let new_z = new_z.min(380.0).max(-380.0);

        *transform.translation.z_mut() = new_z;
    }
}
#[derive(Debug, Default, PartialEq, Clone, Copy, Properties)]
struct Velocity{
    pub velocity: Vec3,
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
        .with(PlayerControl);
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
        // Camera
        .spawn(Camera3dComponents {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7.0, 20.0, 4.0),
            )),
            ..Default::default()
        })
        // Light
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });

    add_tank(&mut commands, &mut meshes, &mut materials);
    add_earth(&mut commands, &mut meshes, &mut materials);
}
