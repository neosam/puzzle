use bevy::prelude::*;

pub struct Drag {
    pub start_pos: Vec2,
    pub in_progress: bool,
}

pub struct ZIndexState {
    pub z_index_state: f32,
}

pub struct Materials {
    pub default_tile: Handle<StandardMaterial>,
    pub highlighted_tile: Handle<StandardMaterial>,
}
