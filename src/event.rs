use bevy::prelude::*;

pub struct SelectEvent {
    pub part_entity: Entity,
    pub transform: Transform,
}
