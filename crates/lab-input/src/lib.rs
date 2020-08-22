use bevy::{
    prelude::*,
    render::{camera::Camera},
    input::{keyboard::KeyCode, Input, mouse::{MouseButtonInput, MouseMotion} },
};

mod systems;
use systems::*;
use lab_entities::world::*;
use lab_entities::player;

use std::time::Duration;

mod menu;

pub mod prelude {
    pub use crate::*;
    pub use menu::*;
}

#[derive(Clone, Properties, Debug)]
pub struct SelectedTile {
    pub name: String,
    #[property(ignore)]
    pub tile_type: TileType,
    pub level : f32,
    pub hardness: f32,
    pub hit_points: u32,
    pub tile: u32
}

impl Default for SelectedTile {
    fn default() -> SelectedTile {
        SelectedTile { tile_type: TileType::Floor , level: 0., name: "Undefined".to_string(), hardness: 0., hit_points: 0, tile: 0}
    }
}

pub struct InputPlugin;

#[allow(dead_code)]
pub mod stage {
    pub const INPUT: &'static str = "input";
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<SelectedTile>()
            .init_resource::<State>()
            .add_system(systems::player_movement_system.system())
            .add_system(systems::track_mouse_movement_system.system());
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub struct State {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    mouse_motion_event_reader: EventReader<MouseMotion>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}
pub struct Mouse {
    pub position: Vec2
}

pub struct MouseClick {
    pub timestamp: i64,
    pub button : MouseButton,
    pub ui_position: Vec2,
    pub map_position: Vec3
}