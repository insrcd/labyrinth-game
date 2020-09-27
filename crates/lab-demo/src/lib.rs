use bevy::prelude::*;

mod systems;

use systems::*;

    pub struct DemoPlugin;

    /// Load a demo that displays the basic functionality of the
    /// Game framework. The demo uses the basic map builder
    impl Plugin for DemoPlugin {
        fn build(&self, app: &mut AppBuilder) {
            app.add_startup_system_to_stage(
                lab_core::stages::POST_INIT,
                create_simple_map_system.system(),
            );
        }
    }
