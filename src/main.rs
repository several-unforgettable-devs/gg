use std::env;

use bevy::prelude::*;
use bevy::render::camera::PerspectiveProjection;

mod asteroids;
use crate::asteroids::*;
mod audio;
use crate::audio::*;
mod bullets;
mod collision;
mod cooldown;
use crate::collision::*;
mod debug;
mod enemies;
use crate::enemies::*;
mod gravity;
use crate::gravity::*;
mod input;
use crate::input::*;
mod velocity;
use crate::velocity::*;
mod trail;
mod game_messaging;
mod boid;
use crate::boid::*;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum KeyboardLayout {
    QWERTY,
    Colemak,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let keyboard_layout = if args.contains(&String::from("-colemak")) {
        KeyboardLayout::Colemak
    } else {
        KeyboardLayout::QWERTY
    };

    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(GameState::Running)
        .add_resource(keyboard_layout)
        .add_plugins(DefaultPlugins)
        //
        // Startup
        .add_startup_system(setup)
        .add_startup_system(setup_audio)
        //.add_startup_system(infotext_system)
        //
        // Input
        .add_system(keyboard_input_update)
        .add_system(mouse_button_input_update)
        .add_system(mouse_move_input_update)
        //
        // Gameplay simulation
        .add_system(velocity_update)
        .add_system(collision_update)
        .add_system(gravity_update)
        .add_system(boid_update)
        //
        // Visuals/UI
        //.add_system(change_text_system)
        .add_system(skybox_update)
        // 
        // Trail
        .add_plugin(trail::MotionTrailPlugin)
        // 
        // Game messaging
        .add_plugin(game_messaging::GameMessagePlugin)
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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum EntityType {
    // Passive objects
    Asteroid,
    Earth,

    // Projectiles
    Bullet,

    // Ships
    Alien,
    Player,
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

pub const PLAYER_SHIP_RADIUS: f32 = 2.;
fn add_ship(commands: &mut Commands, asset_server: &Res<AssetServer>, position: Vec3) {
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
            etype: EntityType::Player,
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
        .with(Gravity { mass: earth_mass })
        .with(Collision {
            mass: earth_mass,
            radius: earth_radius,
            etype: EntityType::Earth,
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

    add_ship(commands, &asset_server, Vec3::new(0.0, 0.0, 100.0));

    add_earth(
        commands,
        &mut meshes,
        &mut materials,
        Vec3::new(0.0, 0.0, 0.0),
    );

    add_asteroids(commands, &mut meshes, &mut materials);
    add_enemies(commands, &asset_server);
}
