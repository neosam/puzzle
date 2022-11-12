use bevy::prelude::*;

use crate::component;

#[derive(Bundle)]
pub struct PartBundle {
    pub part: component::Part,

    #[bundle]
    pub spatial_bundle: SpatialBundle,
}
