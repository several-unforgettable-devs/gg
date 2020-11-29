use bevy::prelude::*;
use crate::input::PlayerInput;
use crate::velocity::Velocity;

pub struct MotionTrailPlugin;

const RESPAWN_RATE:f32 = 0.2;
const TRAIL_SIZE:f32 = 0.05;
const MAX_TRAIL_DIST:f32 = 10.0;

struct TrailManagement {
    reset_timer: Timer 
}

impl TrailManagement {
    fn new() -> TrailManagement {
        TrailManagement{ reset_timer: Timer::from_seconds(RESPAWN_RATE,true)}
    }
}

struct TrailObj
{
    radial_offset:f32,
    angle:f32, // in radians, 0-degress is 'up' on ship.
}

impl Plugin for MotionTrailPlugin
{
    fn build(&self, app: &mut AppBuilder)
    {
        app.add_startup_system(setup_trail)
            .add_resource(TrailManagement::new())
            .add_system(update_trail);
    }
    // span 1 quad, re-draw very 60ms
}

fn make_trail_obj(commands: &mut Commands,
    trail_material: &Handle<StandardMaterial>,
    trail_mesh: &Handle<Mesh>,
    angle_in_degrees: f32
) {
    commands.spawn(PbrBundle {
        mesh: trail_mesh.clone(),
        material: trail_material.clone(),
        transform: Transform {
            ..Default::default()
        },
        ..Default::default()
    })
        .with(TrailObj{ radial_offset: 2.5, angle: f32::to_radians(angle_in_degrees) });
}

fn setup_trail(commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let trail_material = materials.add(Color::rgb(0.75, 0.75, 1.0).into());
    /*let trail_mesh = meshes.add(Mesh::from(shape::Quad {
        size: Vec2::new(0.2, 0.2),
        flip: false
    }));*/
    let trail_mesh = meshes.add(Mesh::from(shape::Cube {
        size: TRAIL_SIZE
    }));
    make_trail_obj(commands, &trail_material, &trail_mesh, 45.0);
    make_trail_obj(commands, &trail_material, &trail_mesh, -45.0);
}

fn build_trail_quat(velocity: &Vec3) -> Quat {
    let velocity_copy = velocity.clone().normalize();

    let quat_vector = Vec3::unit_x().cross(velocity_copy).normalize();
    let quat_angle = velocity_copy.x.acos();
    return Quat::from_axis_angle(quat_vector, quat_angle);
}


const MAX_TRAIL_DIST_SQ:f32 = MAX_TRAIL_DIST * MAX_TRAIL_DIST;

fn update_trail(
    time : Res<Time>,
    mut trail_mgr : ResMut<TrailManagement>,
    mut trail_query : Query<(&TrailObj, &mut Transform)>,
    player_query : Query<(&PlayerInput, &Transform, &Velocity)>
) {
    trail_mgr.reset_timer.tick(time.delta_seconds);
    
    //let respwan = trail_mgr.reset_timer.just_finished();

    for (_, player_transform, velocity_info) in player_query.iter() {
        let player_up = player_transform.rotation * Vec3::unit_y();
        let player_forward = player_transform.forward();
        let player_velocity = velocity_info.velocity.length();
        //let player_velocity_dir = velocity_info.velocity.clone().normalize();
        let trail_quat = build_trail_quat(&velocity_info.velocity);
        for (trail_info, mut transform) in trail_query.iter_mut() {

            let vector_from_player = transform.translation - player_transform.translation;
            //let respawn:bool = ;

            let new_translation = 
                if vector_from_player.length_squared() > MAX_TRAIL_DIST_SQ {
                    let rotation_quat = Quat::from_axis_angle(player_forward, trail_info.angle);
                    let offset_direction = rotation_quat.mul_vec3(player_up);
                    player_transform.translation + offset_direction * trail_info.radial_offset
                } else {
                    transform.translation
                };

            *transform = Transform{
                translation: new_translation,
                rotation: trail_quat,
                scale: Vec3::new(player_velocity, 1.0, 1.0),
            };
            

            //*transform = transform_copy;
        }
    }
    

    
}