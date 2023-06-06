use std::f32::consts::{PI, TAU};

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, render::{mesh::Indices, render_resource::PrimitiveTopology, view::PostProcessWrite}, ecs::{component, system::Command}};
use rand::Rng;

const BOID_COLOR:Color = Color::PURPLE;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_boids_system)
        .add_system(update_posistions_system)
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

#[derive(Component, Debug)]
struct Direction(Vec2);

#[derive(Component, Debug)]
struct Position(Vec2);


fn spawn_boids_system(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
){
    spawn_boid(
        Vec2 { x: 0., y: 0. },
        Vec2 { x: 1., y: 0. }.normalize(),
        commands,
        meshes, 
        materials
    )
}

fn spawn_boid(
    position: Vec2,
    direction: Vec2,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let x = position.x;
    let y = position.y;
    let angle = direction.y.atan2(direction.x);

    commands.spawn(
        MaterialMesh2dBundle {
            transform: Transform{
                translation : Vec3 { x, y, z : 0. },
                rotation : Quat::from_rotation_z(angle),
                scale : Vec3::splat(50.)
            },
            mesh: meshes.add(create_boid_mesh()).into(),
            material: materials.add(ColorMaterial::from(BOID_COLOR)),
            ..default()
        }
    )
    .insert(Position(position))
    .insert(Direction(direction));
}

fn update_posistions_system(
    mut query: Query<(&mut Position, &Direction)>,
    timer: Res<Time>
) {
    for (mut position, direction) in query.iter_mut() {
        position.0.x = position.0.x + direction.0.x;
        position.0.y = position.0.y + direction.0.y;
        println!("{:?}", position);
    }
}