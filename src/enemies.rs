use bevy::prelude::*;
use rand::Rng;

use crate::collision::*;
use crate::velocity::*;
use crate::boid::*;
pub use crate::EntityType;

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
                    etype: EntityType::Asteroid,
                })
                .with(Boid::default())
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