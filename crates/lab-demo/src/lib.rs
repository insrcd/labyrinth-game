use bevy::{prelude::*};

mod systems;

use systems::*;

use bevy::prelude::*;

pub struct DemoPlugin;

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage(lab_core::stage::POST_INIT,create_simple_map_system.system());
    }
}
