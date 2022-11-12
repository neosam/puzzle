use bevy::prelude::*;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Part {
    pub center_position: Vec2,
    pub size: Vec2,
}
