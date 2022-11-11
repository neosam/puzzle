use bevy::prelude::*;
use rand::prelude::*;

use crate::component;
use crate::resource;
use crate::util;

pub fn setup(
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
    let materials = resource::Materials {
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
            let mesh = meshes.add(util::generate_tile_mesh(
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
            commands.spawn_bundle(object).insert(component::Tile);
        }
    }

    commands.insert_resource(materials);
    commands.insert_resource(resource::Drag {
        start_pos: Vec2::ZERO,
        in_progress: false,
    });
    commands.insert_resource(resource::ZIndexState { z_index_state: 2.0 })
}

pub fn select_tile(
    mut tile_query: Query<
        (
            Entity,
            &Transform,
            &GlobalTransform,
            &mut Handle<StandardMaterial>,
        ),
        With<component::Tile>,
    >,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut mouse_motion_events: EventReader<CursorMoved>,
    materials: Res<resource::Materials>,
    mut commands: Commands,
    mut drag: ResMut<resource::Drag>,
) {
    if drag.in_progress {
        return;
    }
    for event in mouse_motion_events.iter() {
        drag.start_pos = event.position;
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
                    commands.entity(entity).insert(component::Selected {
                        transform: *tile_transform,
                    });
                } else {
                    *material = materials.default_tile.clone();
                    commands.entity(entity).remove::<component::Selected>();
                }
            }
        }
    }
}

pub fn drag_start_end(
    mut drag: ResMut<resource::Drag>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut z_index_state: ResMut<resource::ZIndexState>,
    mut selected_query: Query<&mut Transform, With<component::Selected>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        drag.in_progress = true;
        for mut transform in selected_query.iter_mut() {
            transform.translation.z = z_index_state.z_index_state;
            z_index_state.z_index_state += 0.001;
        }
    }
    if mouse_button_input.just_released(MouseButton::Left) {
        drag.in_progress = false;
    }
}

pub fn drag_update(
    drag: Res<resource::Drag>,
    mut mouse_motion_events: EventReader<CursorMoved>,
    camera_query: Query<(&Camera, &GlobalTransform, &bevy::render::camera::Projection)>,
    mut selected_query: Query<(&mut Transform, &component::Selected)>,
) {
    if !drag.in_progress {
        return;
    }
    let (camera, camera_transform, projection): (
        &Camera,
        &GlobalTransform,
        &bevy::render::camera::Projection,
    ) = camera_query.single();
    let projection = match projection {
        bevy::render::camera::Projection::Orthographic(x) => x,
        _ => return,
    };
    for event in mouse_motion_events.iter() {
        let cursor_screen_position = event.position;
        let cursor_world_coordinate = util::viewport_to_world_coordinate(
            projection,
            camera,
            camera_transform,
            cursor_screen_position.as_uvec2(),
        );
        let initial_world_coordinate = util::viewport_to_world_coordinate(
            projection,
            camera,
            camera_transform,
            drag.start_pos.as_uvec2(),
        );
        let world_coordinate_offset = cursor_world_coordinate - initial_world_coordinate;
        for (mut transform, selected) in selected_query.iter_mut() {
            let z = transform.translation.z;
            transform.translation = util::add_z(
                util::discard_z(selected.transform.translation) + world_coordinate_offset,
                z,
            );
        }
    }
}
