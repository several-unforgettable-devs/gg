use bevy::prelude::*;
use bevy::render::camera::PerspectiveProjection;

mod asteroids;
use crate::asteroids::*;
mod audio;
use crate::audio::*;
mod collision;
mod cooldown;
use crate::collision::*;
mod debug;
use debug::{change_text_system, infotext_system};
mod gravity;
use crate::gravity::*;
mod input;
use crate::input::*;
mod velocity;
use crate::velocity::*;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(GameState::Running)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(setup_audio)
        .add_startup_system(infotext_system)
        .add_system(keyboard_input_update)
        .add_system(mouse_input_update)
        .add_system(velocity_update)
        .add_system(collision_update)
        .add_system(gravity_update)
        .add_system(change_text_system)
        .add_system(skybox_update)
        .run();
}

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum GameState {
    Running,
    Paused,
    Lost,
    Won,
}

struct SkyboxState;

fn skybox_update(
    mut skybox_query: Query<(&SkyboxState, &mut Transform)>,
    player_query: Query<(&PlayerInput, &Transform)>,
) {
    for (_, mut skybox_transform) in skybox_query.iter_mut() {
        for (_, player_transform) in player_query.iter() {
            skybox_transform.translation = player_transform.translation;
        }
    }
}

struct EarthMarker;

fn add_ship(commands: &mut Commands, asset_server: Res<AssetServer>, position: Vec3) {
    let ship_mesh_handle =
        asset_server.load("models/ship/player/PlayerShip01_AA.gltf#Mesh0/Primitive0");

    let ship_material_handle =
        asset_server.load("models/ship/player/PlayerShip01_AA.gltf#Material0");

    let ship_scale = 2.;

    commands
        .spawn(PbrBundle {
            mesh: ship_mesh_handle,
            material: ship_material_handle,
            transform: Transform {
                translation: position,
                scale: Vec3::splat(ship_scale),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(PlayerInput)
        .with(Gravity { mass: 4. })
        .with(Collision {
            mass: 4.,
            radius: 2.,
            ctype: CollisionType::Player,
        })
        .with(Velocity::default());
}

fn add_earth(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    let earth_radius = 4.;
    let earth_mass = earth_radius * earth_radius;
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: earth_radius,
                ..Default::default()
            })),
            material: materials.add(Color::rgb(0.2, 0.2, 1.0).into()),
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .with(EarthMarker)
        .with(Gravity { mass: earth_mass })
        .with(Collision {
            mass: earth_mass,
            radius: earth_radius,
            ctype: CollisionType::Earth,
        })
        .with(Velocity::default());
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let skybox_mesh_handle = asset_server.load("models/skybox/skybox.gltf#Mesh0/Primitive0");
    let skybox_material_handle = asset_server.load("models/skybox/skybox.gltf#Material0");

    commands
        .spawn(PbrBundle {
            mesh: skybox_mesh_handle,
            material: skybox_material_handle.clone(),
            transform: Transform::from_translation(Vec3::new(-3.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(SkyboxState)
        // Light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(5.0, 10.0, 10.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            perspective_projection: PerspectiveProjection {
                far: 1100.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(CameraInput);

    add_ship(commands, asset_server, Vec3::new(-40.0, 0., 0.0));

    add_earth(
        commands,
        &mut meshes,
        &mut materials,
        Vec3::new(0.0, 0.0, 0.0),
    );

    add_asteroids(commands, &mut meshes, &mut materials);
}
