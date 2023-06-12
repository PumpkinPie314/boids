use bevy::prelude::*;
use bevy::render::{render_resource::PrimitiveTopology, mesh::Indices};
use rand::Rng;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup_camera)
    .add_startup_system(spawn_boids_system)
    .add_system(movement_system)
    .add_system(wrap_around_system)

    .add_system(flocking_system)
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
            }.normalize() * Vec3::splat(100.0))
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

struct Intruction{
    boid: Entity,
    direction: Vec3,
    factor: f32,
}

const VISION :f32 = 80.0;
const SIZE :f32 = 20.0;

fn flocking_system(
    mut query: Query<(Entity, &Transform, &mut Velocity)>
){
    let mut queue: Vec<Intruction> = vec![];
    for (boid_a, transform_a, _velocity_a) in query.iter() {
        let position_a = transform_a.translation;
        let mut nabor_count: u32 = 0;

        let mut average_velocity = Vec3::ZERO;//alignment
        let mut average_nabor_position = Vec3::ZERO;//alignment

        for (boid_b, transform_b, velocity_b) in query.iter() {
            if boid_a == boid_b {continue}; 
            let position_b = transform_b.translation;
            if position_a.distance(position_b) > VISION {continue};
            if position_a.distance(position_b) < SIZE {
                let away_direction = position_a - position_b;
                queue.push(Intruction {
                    boid: boid_a,
                    direction: away_direction,
                    factor: 0.3,
                })
            };
            nabor_count += 1;
            average_velocity += velocity_b.0;
            average_nabor_position += position_b
        }
        if nabor_count == 0 {continue};
        average_velocity /= Vec3::splat(nabor_count as f32);
        queue.push(Intruction {
            boid: boid_a,
            direction: average_velocity,
            factor: 0.2,
        });
        average_nabor_position /= Vec3::splat(nabor_count as f32);
        let tward_nabors = average_nabor_position - position_a;
        queue.push(Intruction {
            boid: boid_a,
            direction: tward_nabors,
            factor: 0.2,
        });

    };
    for (boid, _transform, mut velocity) in query.iter_mut() {
        for instruction in queue.iter() {
            if instruction.boid != boid {continue};

            let speed = velocity.0.length();
            velocity.0 += instruction.direction * Vec3::splat(instruction.factor);
            velocity.0 = velocity.0.normalize() * speed;
        }
    }
}