use bevy::{prelude::*, render::{mesh::Indices, render_resource::PrimitiveTopology}, sprite::MaterialMesh2dBundle, transform::commands};
use rand::Rng;

const BOID_COLOR:Color = Color::PURPLE;
const SPEED:f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_boid)
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

fn spawn_boid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            transform: Transform{
                translation : Vec3 { x: 0.0, y: 1.0, z: 0.0 },
                rotation : Quat::from_rotation_z(0.0),
                scale : Vec3::splat(50.)
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
) {
    for mut boid in query.iter_mut() {
        boid.translation.x = boid.translation.x + 1.0;
        println!("{:?}",boid)
    }
}