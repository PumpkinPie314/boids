use std::f32::consts::{PI, TAU};

use bevy::{prelude::*, render::{mesh::Indices, render_resource::PrimitiveTopology}, sprite::MaterialMesh2dBundle, transform::{commands, self}};
use rand::Rng;

const BOID_COLOR:Color = Color::PURPLE;
const BOID_COUNT:i32 = 10;
const BOID_MIN_SPEED: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_boids_system)
        .add_systems((
            move_forward_system,
            //speed_up_slow_boids_system,

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
            x: rand::thread_rng().gen_range(-10.0..10.0),
            y: rand::thread_rng().gen_range(-10.0..10.0),
        },
        &mut commands,
        &mut meshes,
        &mut materials,
    )
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