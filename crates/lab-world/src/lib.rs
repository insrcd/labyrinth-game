use bevy::{prelude::*};

mod systems;

use systems::*;

pub struct DemoPlugin;

impl Plugin for WorldP {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(simple_map.system());
    }
}
