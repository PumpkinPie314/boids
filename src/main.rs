use std::f32::consts::{PI, TAU};

use bevy::{prelude::*, render::{mesh::Indices, render_resource::PrimitiveTopology}, sprite::MaterialMesh2dBundle, transform::commands};
use rand::Rng;

const BOID_COLOR:Color = Color::PURPLE;
const SPEED:f32 = 100.0;
const BOID_COUNT:i32 = 10;


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
        ([-1.0, 1.0, 0.0]),
        ([-1.0, -1.0, 0.0]),
        ([1.0, 0.0, 0.0]),
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

fn spawn_boids_system(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_boid(
        0.0,
        0.0,
        PI/2.0,
        commands,
        meshes,
        materials,
    )
}

fn spawn_boid(
    x: f32,
    y: f32,
    angle: f32,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            transform: Transform{
                translation : Vec3 { x, y, z: 0.0 },
                rotation : Quat::from_rotation_z(angle),
                scale : Vec3::splat(20.)
            },
            mesh: meshes.add(create_boid_mesh()).into(),
            material: materials.add(ColorMaterial::from(BOID_COLOR)),
            ..default()
        },
        Boid {},
    ));
}


fn update_posistions_system(
    mut query: Query<&mut Transform, With<Boid>>,
    timer: Res<Time>
) {
    for mut boid in query.iter_mut() {
        let (_, angle) = boid.rotation.to_axis_angle();
        boid.translation.x = boid.translation.x + angle.cos()*SPEED*timer.delta_seconds();
        boid.translation.y = boid.translation.y + angle.sin()*SPEED*timer.delta_seconds();
        boid.rotate_z(0.01);
        println!("{}", angle/TAU)
    }
}