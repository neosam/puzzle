// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod component;
mod resource;
mod system;
mod util;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(system::setup)
        .add_system(system::select_tile)
        .add_system(system::drag_start_end)
        .add_system(system::drag_update)
        .run();
}
