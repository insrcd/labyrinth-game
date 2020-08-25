use bevy::{prelude::*};

pub mod systems;
pub mod maps;
pub mod text;

use systems::*;
use lab_core;

pub mod prelude {
    pub use systems::*;
    pub use maps::*;
    pub use text::*;
    pub use crate::*;
}

pub enum RelativePosition {
    LeftOf,
    RightOf,
    Above,
    Below,
    Current
}

pub struct BuilderPlugin; 

impl Plugin for BuilderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .init_resource::<BuilderSettings>()
        // system to init the tile palette
        .add_startup_system_to_stage(lab_core::stage::POST_INIT, make_tile_palette_system.system())
        // system that will add tiles on click
        .add_system_to_stage(lab_core::stage::PRE_UPDATE, add_tiles_to_world_system.system())
        .add_system(builder_keyboard_system.system())
        .add_system(update_tile_system.system())
        // System for changing builder settings
        .add_system(builder_settings_system.system());
    }
}

#[derive(Default)]
pub struct BuilderSettings {
    pub move_mode: bool
}

