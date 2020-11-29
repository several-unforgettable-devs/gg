use bevy::prelude::*;

use crate::audio::play_sound;
use crate::collision::*;
use crate::gravity::*;
use crate::velocity::*;

struct Bullet;

// Relative to the shooter
pub const RELATIVE_BULLET_SPEED: f32 = 40.;
pub const BULLET_RADIUS: f32 = 0.1;
pub const BULLET_MASS: f32 = 0.1;

pub fn fire_bullet(
    // Systems needed to spawn the bullet
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,

    // To play sound effect
    asset_server: &Res<AssetServer>,
    audio: &Res<Audio>,

    // Info to spawn the bullet
    shooter_position: Vec3,
    shooter_velocity: Vec3,
    shooter_facing: Vec3,
    shooter_barrel_length: f32,
) {
    play_sound(
        &asset_server,
        &audio,
        "audio/LaserShot.mp3",
    );

    let bullet_velocity = shooter_velocity + RELATIVE_BULLET_SPEED * shooter_facing;
    let bullet_position = shooter_position + shooter_barrel_length * shooter_facing;

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: BULLET_RADIUS,
                ..Default::default()
            })),
            material: materials.add(Color::rgb(1.0, 0.2, 0.2).into()),
            transform: Transform::from_translation(bullet_position),
            ..Default::default()
        })
        .with(Bullet)
        .with(Gravity { mass: BULLET_MASS })
        .with(Collision {
            mass: BULLET_MASS,
            radius: BULLET_RADIUS,
            ctype: EntityType::Earth,
        })
        .with(Velocity {
            velocity: bullet_velocity,
        });
}
