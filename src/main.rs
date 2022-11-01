// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::{
    prelude::*,
    render::{
        camera::Projection,
        mesh::{Indices, PrimitiveTopology},
    },
};
use rand::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(select_tile)
        .add_system(drag_start_end)
        .add_system(drag_update)
        .run();
}

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Selected {
    pub transform: Transform,
}

pub struct Drag {
    pub start_pos: Vec2,
    pub in_progress: bool,
}

pub struct ZIndexState {
    pub z_index_state: f32,
}

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
        transform: Transform::from_xyz(0.0, 0.0, 200.0).looking_at(Vec3::ZERO, Vec3::Y),
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

    let mut tiles = Vec::with_capacity(width * height);

    for x in 0..width {
        for y in 0..height {
            let mesh = meshes.add(generate_tile_mesh(
                Vec2::new(x as f32 / width as f32, y as f32 / height as f32),
                Vec2::new(
                    (x + 1) as f32 / width as f32,
                    (y + 1) as f32 / height as f32,
                ),
            ));
            tiles.push(mesh);
        }
    }

    let mut rng = rand::thread_rng();
    for x in 0..width {
        for y in 0..height {
            let index = rng.gen::<usize>() % tiles.len();
            let mesh = tiles.remove(index);
            let object = PbrBundle {
                mesh,
                material: materials.default_tile.clone(),
                transform: Transform::from_xyz(
                    space * x as f32 - width as f32 / 2.0,
                    space * (height - y) as f32 - height as f32 / 2.0,
                    (y * width + x) as f32 / (width * height) as f32 + 1.0,
                ),
                ..Default::default()
            };
            commands.spawn_bundle(object).insert(Tile);
        }
    }

    commands.insert_resource(materials);
    commands.insert_resource(Drag {
        start_pos: Vec2::ZERO,
        in_progress: false,
    });
    commands.insert_resource(ZIndexState { z_index_state: 2.0 })
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
    mut tile_query: Query<
        (
            Entity,
            &Transform,
            &GlobalTransform,
            &mut Handle<StandardMaterial>,
        ),
        With<Tile>,
    >,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut mouse_motion_events: EventReader<CursorMoved>,
    materials: Res<Materials>,
    mut commands: Commands,
    mut drag: ResMut<Drag>,
) {
    if drag.in_progress {
        return;
    }
    for event in mouse_motion_events.iter() {
        drag.start_pos = event.position.clone();
        let (camera, camera_transform): (&Camera, &GlobalTransform) = camera_query.single();
        for (entity, tile_transform, tile_global_transform, mut material) in tile_query.iter_mut() {
            if let (Some(lower_left), Some(upper_right)) = (
                camera.world_to_viewport(
                    camera_transform,
                    tile_global_transform.translation() + Vec3::new(-1.0, -1.0, 0.0),
                ),
                camera.world_to_viewport(
                    camera_transform,
                    tile_global_transform.translation() + Vec3::new(1.0, 1.0, 0.0),
                ),
            ) {
                if event.position.x >= lower_left.x
                    && event.position.y >= lower_left.y
                    && event.position.x <= upper_right.x
                    && event.position.y <= upper_right.y
                {
                    *material = materials.highlighted_tile.clone();
                    commands.entity(entity).insert(Selected {
                        transform: tile_transform.clone(),
                    });
                } else {
                    *material = materials.default_tile.clone();
                    commands.entity(entity).remove::<Selected>();
                }
            }
        }
    }
}

pub fn drag_start_end(
    mut drag: ResMut<Drag>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut z_index_state: ResMut<ZIndexState>,
    mut selected_query: Query<&mut Transform, With<Selected>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        drag.in_progress = true;
        for mut transform in selected_query.iter_mut() {
            transform.translation.z = (*z_index_state).z_index_state;
            z_index_state.z_index_state += 0.001;
        }
    }
    if mouse_button_input.just_released(MouseButton::Left) {
        drag.in_progress = false;
    }
}

pub fn drag_update(
    drag: Res<Drag>,
    mut mouse_motion_events: EventReader<CursorMoved>,
    camera_query: Query<(&Camera, &GlobalTransform, &Projection)>,
    mut selected_query: Query<(&mut Transform, &Selected)>,
) {
    if !drag.in_progress {
        return;
    }
    let (camera, camera_transform, projection): (&Camera, &GlobalTransform, &Projection) =
        camera_query.single();
    let projection = match projection {
        Projection::Orthographic(x) => x,
        _ => return,
    };
    for event in mouse_motion_events.iter() {
        let cursor_screen_position = event.position;
        let cursor_world_coordinate = viewport_to_world_coordinate(
            projection,
            camera,
            camera_transform,
            cursor_screen_position.as_uvec2(),
        );
        let initial_world_coordinate = viewport_to_world_coordinate(
            projection,
            camera,
            camera_transform,
            drag.start_pos.as_uvec2(),
        );
        let world_coordinate_offset = cursor_world_coordinate - initial_world_coordinate;
        for (mut transform, selected) in selected_query.iter_mut() {
            let z = transform.translation.z;
            transform.translation = add_z(
                discard_z(selected.transform.translation) + world_coordinate_offset,
                z,
            );
        }
    }
}

pub fn discard_z(vec: Vec3) -> Vec2 {
    Vec2::new(vec.x, vec.y)
}
pub fn add_z(vec: Vec2, z: f32) -> Vec3 {
    Vec3::new(vec.x, vec.y, z)
}

pub fn viewport_to_world_coordinate(
    projection: &OrthographicProjection,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    position: UVec2,
) -> Vec2 {
    let projection_size = Vec2::new(
        projection.right - projection.left,
        projection.top - projection.bottom,
    );
    let viewport = camera.physical_viewport_rect().unwrap();
    let viewport_size = camera.physical_viewport_size().unwrap();
    let relative_position = Vec2::new(
        (position.x - viewport.0.x) as f32 / viewport_size.x as f32,
        (position.y - viewport.0.y) as f32 / viewport_size.y as f32,
    );
    let origin_position = (relative_position * projection_size
        + Vec2::new(projection.left, projection.bottom))
        * projection.scale
        * 2.0;
    origin_position + discard_z(camera_transform.translation())
}
