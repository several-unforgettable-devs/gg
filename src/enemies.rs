use bevy::prelude::*;
use rand::Rng;

use crate::collision::*;
use crate::velocity::*;
use crate::boid::*;
use crate::cooldown::*;
use crate::input::*;
use crate::bullets::*;
pub use crate::EntityType;

pub const ENEMY_WEAPON_COOLDOWN_DURATION: f64 = 2.5;
pub const ENEMY_BARREL_LENGTH: f32 = 1.2 * crate::PLAYER_SHIP_RADIUS;
pub const ENEMY_TARGETING_DISTANCE: f32 = 50.0;

#[derive(Clone, Copy, Default)]
pub struct Enemy {
    pub enemy_weapon_cooldown: Cooldown,
}

#[derive(Clone, Copy)]
struct EnemyData {
    entity: Entity,
    enemy: Enemy,
    transform: Transform,
    velocity: Vec3,
}

pub fn enemies_update(
    commands: &mut Commands,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    asset_server: Res<AssetServer>,
    audio: Res<Audio>,

    time: Res<Time>,
    mut enemy_query: Query<(Entity, &mut Enemy, &Transform, &mut Velocity)>,
    mut player_query: Query<(&PlayerInput, &Transform)>,
) {
    let enemies: Vec<EnemyData> = enemy_query
        .iter_mut()
        .map(|(ent, e, t, v)| EnemyData {
            entity: ent,
            enemy: *e,
            transform: *t,
            velocity: v.velocity,
        })
        .collect();

    for i in 0..enemies.len() {
        if enemies[i].enemy.enemy_weapon_cooldown.over(&time) {
            for (_, player_transform) in player_query.iter_mut() {
                let player_position = player_transform.translation;

                if (player_position - enemies[i].transform.translation).length() < ENEMY_TARGETING_DISTANCE {
                    let enemy_facing = (player_position - enemies[i].transform.translation).normalize();
                    fire_bullet(
                        commands,
                        &mut meshes,
                        &mut materials,
                        &asset_server,
                        &audio,
                        enemies[i].transform.translation,
                        enemies[i].velocity,
                        enemy_facing,
                        ENEMY_BARREL_LENGTH,
                    );

                    match enemy_query.get_component_mut::<Enemy>(enemies[i].entity) {
                        Ok(mut enemy) => {
                            (*enemy).enemy_weapon_cooldown.reset(&time, ENEMY_WEAPON_COOLDOWN_DURATION);
                        }
                        _ => (),
                    }

                    break;
                }
            }
        }
    }
}

pub fn add_enemies(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();

    let ship_scale = 2.;

    let ship_swarm_count_min = 2;
    let ship_swarm_count_max = 10;
    let mut current_enemy_count = 0;

    while current_enemy_count < 100 {
        let mut swarm_count = rng.gen_range(ship_swarm_count_min, ship_swarm_count_max);

        if current_enemy_count + swarm_count > 100 {
            swarm_count = 100 - current_enemy_count;
        }

        let swarm_span: i32 = ((swarm_count as f64).sqrt()) as i32;
        let mut swarm_position = Vec3::new(rng.gen_range(-150.0, 150.0), rng.gen_range(-150.0, 150.0), rng.gen_range(-150.0, 150.0));
        let initial_swarm_position = swarm_position;

        for i in 0..swarm_count {
            let ship_mesh_handle =
                asset_server.load("models/ship/player/PlayerShip01_AA.gltf#Mesh0/Primitive0");

            let ship_material_handle =
                asset_server.load("models/ship/player/PlayerShip01_AA.gltf#Material0");
            current_enemy_count += 1;
            commands
                .spawn(PbrBundle {
                    mesh: ship_mesh_handle,
                    material: ship_material_handle,
                    transform: Transform {
                        translation: swarm_position,
                        scale: Vec3::splat(ship_scale),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with(Collision {
                    mass: 4.,
                    radius: 2.,
                    etype: EntityType::Alien,
                })
                .with(Boid::default())
                .with(Enemy::default())
                .with(Velocity { velocity: Vec3::new(rng.gen_range(-5.0, 5.0),
                    rng.gen_range(-5.0, 5.0),
                    rng.gen_range(-5.0, 5.0))});
            
            if i % swarm_span == 0 {
                swarm_position.y += 5.0;
                swarm_position.x = initial_swarm_position.x;
            }
            else {
                swarm_position.x += 5.0;
            }
        }
    }
}