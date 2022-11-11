use bevy::prelude::*;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Selected {
    pub transform: Transform,
}
