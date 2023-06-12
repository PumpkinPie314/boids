use std::ops::Mul;

use bevy::prelude::*;
use bevy::render::{render_resource::PrimitiveTopology, mesh::Indices};
use rand::Rng;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_event::<UpdateVelocityEvent>()
    .add_startup_system(setup_camera)
    .add_startup_system(spawn_boids_system)
    .add_system(movement_system)
    .add_system(wrap_around_system)
    .add_system(flocking_system)
    .add_system(update_velocitys)
    .run();
}
fn setup_camera(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}

fn create_triagle_mesh() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![
        [0.0, 2.0, 0.0],
        [-1.0, -1.0, 0.0],
        [1.0, -1.0, 0.0],
    ]);
    mesh.set_indices(Some(Indices::U32(vec![0,1,2])));
    mesh
}

#[derive(Component)]
struct Velocity(Vec3);

struct UpdateVelocityEvent(Entity, Vec3, f32);

fn spawn_boids_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window>,
){
    let mut random_thread = rand::thread_rng();
    let window = window_query.get_single().unwrap();
    for _ in 0..=50 {
        commands.spawn((
            ColorMesh2dBundle {
                material: materials.add(ColorMaterial::from(Color::PURPLE)), 
                mesh: meshes.add(create_triagle_mesh()).into(),
                transform: Transform {
                    scale: Vec3::splat(10.0),
                    translation: Vec3 {
                        x: random_thread.gen_range(-window.width()..window.width()),
                        y: random_thread.gen_range(-window.height()..window.height()),
                        z: 0.0,
                    },
                    ..default()
                },
                ..default()
            },
            Velocity(Vec3{
                x: random_thread.gen_range(-1.0..1.0),
                y: random_thread.gen_range(-1.0..1.0),
                z: 0.0,
            }.normalize().mul(Vec3::splat(50.0)))
        ));
    }
}

fn movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity)>
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
        transform.rotation = Quat::from_rotation_z((-velocity.0.x).atan2(velocity.0.y));
    }
}

fn wrap_around_system(
    window_query: Query<&Window>,
    mut query: Query<&mut Transform>
) {
    let window_entity = window_query.get_single().unwrap();
    let width = window_entity.width();
    let height = window_entity.height();
    for mut boid in query.iter_mut() {
        if boid.translation.x < -width / 2.0 {
            boid.translation.x = width / 2.0
        };
        if boid.translation.x > width / 2.0 {
            boid.translation.x = -width / 2.0
        };
        if boid.translation.y < -height / 2.0 {
            boid.translation.y = height / 2.0
        };
        if boid.translation.y > height / 2.0 {
            boid.translation.y = -height / 2.0
        };
    }
}

fn flocking_system(
    mut update_velocity_event : EventWriter<UpdateVelocityEvent>,
    query: Query<(Entity, &Transform, &Velocity)>,
){
    for (entity_me, transform_me, velocity_me) in query.iter() {
        update_velocity_event.send(UpdateVelocityEvent(
            entity_me,
            Vec3 { x: 0.0, y: -1.0, z: 0.0},
            1.0
        ))
        // let position_me: Vec3 = transform_me.translation;
        // let mut nabor_count: i32 = 0;
        // let mut average_nabor_velocity: Vec3 = Vec3::ZERO;
        // for (entity_other, transform_other, velocity_other) in query.iter() {
        //     let position_other = transform_other.translation;
        //     if entity_me == entity_other {continue;};
        //     if position_me.distance(position_other) < 40.0 {
        //         let away_from_other: Vec3{
                    
        //         };
        //     };
        // };
    }
}

fn update_velocitys(
    mut update_velocity_event : EventReader<UpdateVelocityEvent>,
    mut query: Query<&mut Velocity>,
){
    for UpdateVelocityEvent(entity, direction, force) in update_velocity_event.iter() {
        let mut velocity = query.get_mut(*entity).unwrap();
        velocity.0 += *direction * Vec3::splat(*force)
    };
}