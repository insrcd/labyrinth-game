use bevy::{prelude::*};

mod systems;

use systems::*;


use crate::world::*;

use bevy::prelude::*;

pub struct DemoPlugin;

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(simple_map.system());
    }
}
