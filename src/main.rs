use bevy::{prelude::*, sprite::MaterialMesh2dBundle, render::{mesh::Indices, render_resource::PrimitiveTopology}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_boids)
        .run();
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



fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(create_boid_mesh()).into(),
        transform: Transform::default().with_scale(Vec3::splat(50.)),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..default()
    });
}