use std::f32::consts::{PI, TAU};

use bevy::{prelude::*, render::{mesh::Indices, render_resource::PrimitiveTopology, view::window}, sprite::MaterialMesh2dBundle, window::PrimaryWindow, ecs::query};
use rand::Rng;

const BOID_COLOR:Color = Color::PURPLE;
const BOID_COUNT:i32 = 10;
const BOID_MIN_SPEED: f32 = 20.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_boids_system)
        .add_systems((
            move_forward_system,
            speed_up_slow_boids_system,
            wrap_around_system,
            alignment,
        ))
        // .add_system(turn_system)
        .run();
}

pub fn setup_camera(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
}
fn create_boid_mesh() -> Mesh{

    let vertices = [
        ([0.0, 1.0, 0.0]),
        ([-1.0, -1.0, 0.0]),
        ([1.0, -1.0, 0.0]),
    ];

    let indices = Indices::U32(vec![0, 2, 1, 0, 3, 2]);

    let positions: Vec<_> = vertices.iter().map(|p| *p).collect();

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(indices));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh
}

#[derive(Component)]
struct SpecialTemp {}

#[derive(Component)]
struct Boid {}
#[derive(Component)]
struct Velocity(Vec2);
fn spawn_boids_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for _ in 0..BOID_COUNT {
        spawn_boid(
            0.0,
            0.0,
            Vec2 {
                x: rand::thread_rng().gen_range((-2.0*BOID_MIN_SPEED)..(2.0*BOID_MIN_SPEED)),
                y: rand::thread_rng().gen_range((-2.0*BOID_MIN_SPEED)..(2.0*BOID_MIN_SPEED)),
            },
            &mut commands,
            &mut meshes,
            &mut materials,
        );
        let svelocity = Vec2 { x: 1.0, y: 0.0 };
        commands.spawn((
            MaterialMesh2dBundle {
                transform: Transform{
                    translation : Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                    rotation : Quat::from_rotation_z(-svelocity.x.atan2(svelocity.y)),
                    scale : Vec3::splat(20.)
                },
                mesh: meshes.add(create_boid_mesh()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                ..default()
            },
            Boid {},
            SpecialTemp{},
            Velocity(svelocity),
        ));
    }
}

fn spawn_boid(
    x: f32,
    y: f32,
    velocity: Vec2,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            transform: Transform{
                translation : Vec3 { x, y, z: 0.0 },
                rotation : Quat::from_rotation_z(-velocity.x.atan2(velocity.y)),
                scale : Vec3::splat(20.)
            },
            mesh: meshes.add(create_boid_mesh()).into(),
            material: materials.add(ColorMaterial::from(BOID_COLOR)),
            ..default()
        },
        Boid {},
        Velocity(velocity),
    ));
}


fn move_forward_system(
    mut query: Query<(&mut Transform, &Velocity)>,
    timer: Res<Time>
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.0.x * timer.delta_seconds();
        transform.translation.y += velocity.0.y * timer.delta_seconds();
    }
}
fn speed_up_slow_boids_system(
    mut query: Query<&mut Velocity>
) {
    for mut velocity in query.iter_mut() {
        if velocity.0.length() < BOID_MIN_SPEED {
            velocity.0 = velocity.0.normalize() * BOID_MIN_SPEED;
        }
    }
}
fn wrap_around_system(
    window_query: Query<&Window, With<PrimaryWindow>>,
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

// fn alignment(
//     mut query: Query<(&mut Velocity, &Transform)>
// ) {
//     for (mut my_velocity, my_position) in query.iter_mut() {
//         query.iter().filter(|other_boid|{
//             let other_position = other_boid.1.translation;
//             false
//         });
//     }
// }