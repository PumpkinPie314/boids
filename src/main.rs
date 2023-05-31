use bevy::{prelude::*, sprite::MaterialMesh2dBundle, render::{mesh::Indices, render_resource::PrimitiveTopology}, ecs::component};
use rand::Rng;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_boids)
        .add_system(update_boid_pos)
        .add_system(print_boid_info)
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
pub struct Boid{
    predator: bool 
}

#[derive(Component, Clone, Debug, PartialEq)]
struct Direction(Vec2);


fn update_boid_pos(){
    return
}

pub fn print_boid_info(boid_info: Query<&Boid, &MaterialMesh2dBundle<M>>) {
    for boid in boid_info.iter() {
        println!("Pos:{:?}\t Vel: {:?}",boid. , boid.velocity);
    };
}

fn spawn_boids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Boid {
            predator: false
        },
        MaterialMesh2dBundle {
            mesh: meshes.add(create_boid_mesh()).into(),
            transform: Transform::from_xyz(rand::random(), y, 0).with_scale(Vec3::splat(50.)),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        }
    ));
}