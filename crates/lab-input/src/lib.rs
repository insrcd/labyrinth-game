use bevy::{input::mouse::*, prelude::*};

use lab_core::prelude::*;
use lab_entities::player;

mod systems;
mod menu;

pub mod prelude {
    pub use crate::*;
    pub use menu::*;
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<MouseClickEvent>()
            .init_resource::<SelectedTile>()
            .init_resource::<State>()
            .init_resource::<ScrollState>()
            .init_resource::<Mouse>()
            .init_resource::<MouseState>()
            .add_startup_system(input_timers.system())
            .add_system(systems::player_movement_system.system())
            .add_system_to_stage(stage::EVENT_UPDATE, systems::track_mouse_movement_system.system())
            .add_system_to_stage(stage::EVENT_UPDATE, systems::mouse_wheel_system.system())
            .add_system_to_stage(stage::EVENT_UPDATE, systems::mouse_click_system.system());
    }
}

#[derive(Clone, Properties, Debug)]
pub struct SelectedTile {
    pub name: String,
    pub level : f32,
    pub hardness: f32,
    pub hit_points: u32,
    pub tile: usize,
    pub category : String
}

impl Default for SelectedTile {
    fn default() -> SelectedTile {
        SelectedTile { level: 0., name: "Undefined".to_string(), hardness: 0., hit_points: 0, tile: 0, category: "world".to_string()}
    }
}

pub struct InputPlugin;

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
/// Resource for quick access to the current mouse position
#[derive(Default, Debug)]
pub struct Mouse {
    pub position : Vec2,
    pub ui_position : Vec2,
    pub world_position: Vec3
}

pub struct MouseClickEvent {
    pub timestamp: f64,
    pub button : MouseButton,
    pub ui_position: Vec2,
    pub world_position: Vec3
}

#[derive(Default)]
pub struct MouseState {
    pub click_events: EventReader<MouseClickEvent>, 
}