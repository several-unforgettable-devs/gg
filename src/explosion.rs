use bevy::prelude::*;
use bevy::core::Timer;

use crate::velocity::Velocity;

use ezing;

pub struct ExplosionEvent {
    position: Vec3,
    velocity: Vec3
}

impl ExplosionEvent {
    pub fn new(pos: Vec3, velocity: Vec3) -> ExplosionEvent {
        ExplosionEvent{ position: pos, velocity: velocity }
    }
}

pub struct ExplosionPlugin;

fn lerp(t:f32, a:f32, b:f32) -> f32 {
    a + (b-a) * t
}

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_resource(ExplosionSpawner::new())
        .add_event::<ExplosionEvent>()
        .add_startup_system(explosion_setup)
        .add_system(explosion_update);  
    }
    // span 1 quad, re-draw very 60ms
}

// Setting
const EXPLOSION_MAX_SCALE:f32 = 3.;
const EXPLOSION_TIME_DURATION_IN_SECONDS:f32 = 0.3;


// internal constants
const EXPLOSION_INIT_SCALE:f32 = 0.25;

struct ExplosionSpawner {
    expl_material: Handle<StandardMaterial>,
    expl_mesh: Handle<Mesh>,
}



impl ExplosionSpawner {

    fn new() -> ExplosionSpawner {
        ExplosionSpawner{ 
            expl_material: Default::default(), 
            expl_mesh: Default::default(),
        }
    }

    fn spawn_explosion(&self,
        commands: &mut Commands,
        event: &ExplosionEvent,
    ) {
        commands.spawn(PbrBundle {
            mesh: self.expl_mesh.clone(),
            material: self.expl_material.clone(),
            transform: Transform {
                translation: event.position,
                ..Default::default()
            },
            ..Default::default()
        })
            .with(ExplosionInstance{ timer: Timer::from_seconds(EXPLOSION_TIME_DURATION_IN_SECONDS, false)})
            .with(Velocity::new(event.velocity));
    }

    fn setup(
        &mut self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        self.expl_material = materials.add(Color::rgb(1.0, 0.1, 0.1).into());
        self.expl_mesh = meshes.add(Mesh::from(shape::Icosphere::default()));
    }
}

struct ExplosionInstance {
    timer: Timer,
}

fn explosion_setup(
    mut explostion_spawner: ResMut<ExplosionSpawner>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    explostion_spawner.setup(&mut meshes, &mut materials);
}

fn explosion_update(
    time: Res<Time>,
    mut explosion_queue: Local<EventReader<ExplosionEvent>>,
    explosion_events: Res<Events<ExplosionEvent>>,
    spawner : Res<ExplosionSpawner>,
    commands: &mut Commands,
    mut explosions_query: Query<(Entity, &mut ExplosionInstance, &mut Transform)>
) {
    
    for event in explosion_queue.iter(&explosion_events) {
        spawner.spawn_explosion(commands, event);
    }

    for (entity, mut explosion_info, mut transform) in explosions_query.iter_mut() {

        explosion_info.timer.tick(time.delta_seconds);

        let normalized_time = explosion_info.timer.elapsed() / EXPLOSION_TIME_DURATION_IN_SECONDS;
        let new_scale = lerp(ezing::sine_out(normalized_time), EXPLOSION_INIT_SCALE, EXPLOSION_MAX_SCALE);
        transform.scale = Vec3::splat(new_scale);

        if explosion_info.timer.just_finished() {
            commands.despawn(entity);
        }
    }
}