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
        .add_system(select_tile)
        .run();
}

#[derive(Component)]
pub struct Tile;

pub struct Materials {
    default_tile: Handle<StandardMaterial>,
    highlighted_tile: Handle<StandardMaterial>,
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
    let materials = Materials {
        default_tile: materials.add(StandardMaterial {
            base_color_texture: Some(texture.clone()),
            emissive_texture: Some(texture.clone()),
            emissive: Color::WHITE,
            perceptual_roughness: 1.0,
            ..Default::default()
        }),
        highlighted_tile: materials.add(StandardMaterial {
            base_color_texture: Some(texture),
            base_color: Color::ALICE_BLUE,
            ..Default::default()
        }),
    };

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
                material: materials.default_tile.clone(),
                transform: Transform::from_xyz(
                    space * x as f32 - width as f32 / 2.0,
                    space * (height - y) as f32 - height as f32 / 2.0,
                    0.0,
                ),
                ..Default::default()
            };
            commands.spawn_bundle(object).insert(Tile);
        }
    }

    commands.insert_resource(materials);
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

pub fn select_tile(
    mut tile_query: Query<(&GlobalTransform, &mut Handle<StandardMaterial>), With<Tile>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut mouse_motion_events: EventReader<CursorMoved>,
    materials: Res<Materials>,
) {
    for event in mouse_motion_events.iter() {
        bevy::log::info!("Mouse event");
        let (camera, camera_transform): (&Camera, &GlobalTransform) = camera_query.single();
        for (tile_transform, mut material) in tile_query.iter_mut() {
            if let (Some(lower_left), Some(upper_right)) = (
                camera.world_to_viewport(
                    camera_transform,
                    tile_transform.translation() + Vec3::new(-1.0, -1.0, 0.0),
                ),
                camera.world_to_viewport(
                    camera_transform,
                    tile_transform.translation() + Vec3::new(1.0, 1.0, 0.0),
                ),
            ) {
                if event.position.x >= lower_left.x
                    && event.position.y >= lower_left.y
                    && event.position.x <= upper_right.x
                    && event.position.y <= upper_right.y
                {
                    bevy::log::info!("Hover!");
                    *material = materials.highlighted_tile.clone();
                } else {
                    *material = materials.default_tile.clone();
                }
            }
        }
    }
}
