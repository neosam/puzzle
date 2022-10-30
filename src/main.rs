// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        projection: OrthographicProjection {
            scale: 1.0 / 30.0,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    };
    commands.spawn_bundle(camera);

    let texture: Handle<Image> = asset_server.load("icon.png");
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(texture),
        ..Default::default()
    });

    let width = 4;
    let height = 4;
    let space = 2.3;

    for x in 0..width {
        for y in 0..height {
            let mesh = meshes.add(generate_tile_mesh(
                Vec2::new(x as f32 / width as f32, y as f32 / height as f32),
                Vec2::new(
                    (x + 1) as f32 / width as f32,
                    (y + 1) as f32 / height as f32,
                ),
            ));

            let object = PbrBundle {
                mesh,
                material: material.clone(),
                transform: Transform::from_xyz(
                    space * x as f32 - width as f32 / 2.0,
                    space * (height - y) as f32 - height as f32 / 2.0,
                    0.0,
                ),
                ..Default::default()
            };
            commands.spawn_bundle(object);
        }
    }
}

fn generate_tile_mesh(upper_left: Vec2, lower_right: Vec2) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    bevy::log::info!("Texture coordinates: {}, {}", upper_left, lower_right);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [1.0, 1.0, 0.0],
            [-1.0, 1.0, 0.0],
            [-1.0, -1.0, 0.0],
            [1.0, -1.0, 0.0],
        ],
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            [lower_right.x, upper_left.y],
            [upper_left.x, upper_left.y],
            [upper_left.x, lower_right.y],
            [lower_right.x, lower_right.y],
        ],
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ],
    );
    mesh.set_indices(Some(Indices::U32(vec![0, 1, 2, 0, 2, 3])));
    mesh
}
