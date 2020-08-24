use bevy::{
    prelude::*,
    render::{camera::Camera},
    input::{keyboard::KeyCode, Input, mouse::{MouseButtonInput, MouseMotion, MouseWheel} },
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
    pub tile: usize,
    pub category : String
}

impl Default for SelectedTile {
    fn default() -> SelectedTile {
        SelectedTile { tile_type: TileType::Floor , level: 0., name: "Undefined".to_string(), hardness: 0., hit_points: 0, tile: 0, category: "world".to_string()}
    }
}

pub struct InputPlugin;

#[allow(dead_code)]
pub mod stage {
    pub const INPUT: &'static str = "input";
}

pub struct ScrollTimer(Timer);
pub struct ScrollState {
    pub y : f32,
    pub x : f32,
    pub current_scale: f32
}

impl Default for ScrollState {

    fn default() -> Self {
        ScrollState {
            y: 0.,
            x: 0.,
            current_scale: 1.
        }
    }
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<SelectedTile>()
            .init_resource::<State>()
            .init_resource::<ScrollState>()
            .init_resource::<Mouse>()
            .add_startup_system(input_timers.system())
            .add_system(systems::player_movement_system.system())
            .add_system(systems::track_mouse_movement_system.system())
            .add_system(systems::mouse_wheel_system.system());
    }
}

fn input_timers (mut commands : Commands) {
    commands.spawn((ScrollTimer(Timer::from_seconds(0.1, false)),));
}
#[allow(dead_code)]
#[derive(Default)]
pub struct State {
    pub mouse_button_event_reader: EventReader<MouseButtonInput>,
    pub mouse_motion_event_reader: EventReader<MouseMotion>,
    pub cursor_moved_event_reader: EventReader<CursorMoved>,
    pub mouse_wheel_event_reader: EventReader<MouseWheel>
}
#[derive(Default)]
pub struct Mouse {
    pub position: Vec2
}

pub struct MouseClick {
    pub timestamp: i64,
    pub button : MouseButton,
    pub ui_position: Vec2,
    pub map_position: Vec3
}