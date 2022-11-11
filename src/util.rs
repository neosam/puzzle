use bevy::prelude::*;

pub fn generate_tile_mesh(upper_left: Vec2, lower_right: Vec2) -> Mesh {
    let mut mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList);
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
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(vec![
        0, 1, 2, 0, 2, 3,
    ])));
    mesh
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
