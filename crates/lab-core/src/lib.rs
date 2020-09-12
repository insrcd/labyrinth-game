
pub use bevy::{prelude::*, render::camera::*, input::mouse::MouseButtonInput};

pub use crate::interaction::*;
pub use crate::tiles::*;
pub use crate::world::*;
pub use crate::ui::*;

use rand::distributions::{Standard, Distribution};
use rand::Rng;

mod systems;
mod interaction;
mod tiles;
mod world;
mod ui;

pub mod prelude {
    pub use crate::*;
}

pub mod stages {
    /// Stage for initializing resources (used for startup systems)
    pub const INIT: &'static str = "init";
    /// Stage after initializing resources (used for startup systems)
    pub const POST_INIT: &'static str = "post_init";
    pub const PRE_UPDATE: &'static str = "pre_update";
    /// Default stage
    pub const UPDATE: &'static str = "update";
    /// Stage for processing after an update
    pub const VALIDATION: &'static str = "validation";
    /// Stage after update / processing    
    pub const POST_UPDATE: &'static str = "postupdate";
}
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<WorldSettings>()
            .init_resource::<AdventureLog>()
            .init_resource::<InputTimer>();
    }
}

use serde::Serialize;

#[derive(Properties, Serialize, Debug, Clone, PartialEq)]
pub struct Named(pub String);

impl Default for Named {
    fn default() -> Self {
        Named ("No Name".to_string())
    }
}

#[derive(Debug)]
pub struct InputTimer(pub Timer);
impl Default for InputTimer {
    fn default() -> Self {
        InputTimer(Timer::from_seconds(0.1, false))
    }
}
pub struct Despawn;
#[derive(Debug, Clone, Copy)]
pub struct Moveable;
/// This defines an entity as zoomable. It will be modified by the zoom system.
#[derive(Debug, Clone, Copy, Properties, Default)]
pub struct Zoomable;

pub struct StaticLocation;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct MenuItem {
    pub name: String
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct MenuDefinition {
    pub items : Vec<MenuItem>
}

pub struct WorldSettings {
    pub tile_size: f32,
    pub base_player_speed: f32,
    pub base_npc_speed: f32,
    pub base_scale: f32
}

impl Default for WorldSettings {
    fn default() -> Self {
        WorldSettings {
            tile_size: 16.,
            base_player_speed: 8.,
            base_npc_speed: 8.,
            base_scale: 5.
        }
    }
}

pub struct AdventureLog {
    pub logs : Vec<String>
}

impl Default for AdventureLog {
    fn default() -> Self {
        AdventureLog {
            logs: Vec::new()
        }
    }
}

impl AdventureLog {
    pub fn add_message(&mut self, log : String) -> &mut AdventureLog {

        // don't repeat the messages
        if self.logs.len() == 0 || self.logs[self.logs.len()-1] != log {
            self.logs.push(log);
        }

        self
    }

    /// Get a log line from the bottom of the log
    pub fn last(&self, i : usize) -> Option<&str> {
        if i < self.logs.len() {
            Some(&self.logs[self.logs.len()-1-i][..])
        } else {
            None
        }
    }

    pub fn make(&mut self, commands: &mut Commands, font_handle:  Handle<Font>,  length : u32) -> &mut AdventureLog {
        for n in 1..length+1 {
            let e = Entity::new();
            commands.spawn_as_entity(e, TextComponents {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {bottom:Val::Px(20. + (length-n) as f32 * 25.), left:Val::Px(5.), ..Default::default()},
                    ..Default::default()
                },
                text: Text {
                    value: "".to_string(),
                    font: font_handle,
                    style: TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                },
                draw: Draw {is_visible: true, ..Default::default()},
                ..Default::default()
            }).with_bundle((Named(format!("log_line_{}", length-n).to_string()), 
                            StaticLocation, )
                    ); 
        }
        self
    }
}


#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Movement { 
    pub start: Location, 
    pub end:  Location, 
    pub direction: CardinalDirection, 
    pub legal: Option<bool> 
}

impl Default for Movement {
    fn default() -> Movement {
        Movement {
            start: Location::default(),
            end: Location::default(),
            direction: CardinalDirection::None,
            legal: None
        }
    }
}

impl Movement {
    pub fn new(start: Location, end: Location, direction: CardinalDirection) -> Self {
        Movement {
            start: start,
            end: end,
            direction: direction,
            legal: None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CardinalDirection {
    None,
    North,
    South,
    East,
    West
}


impl Distribution<CardinalDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CardinalDirection {
        match rng.gen_range(0, 5) {
            0 => CardinalDirection::North,
            1 => CardinalDirection::South,
            2 => CardinalDirection::East,
            3 => CardinalDirection::West,
            _ => CardinalDirection::None
        }
    }
}

