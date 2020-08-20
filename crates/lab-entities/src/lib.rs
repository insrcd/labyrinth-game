use bevy::{prelude::*};

pub mod player;
pub mod systems;
pub mod objs;
pub mod npcs;
pub mod world;

pub mod prelude {
    pub use crate::{
        npcs::*,
        objs::*,
        player::*,
        systems::*,
        world::*,
        *
    };     
}



use serde::Serialize;

#[derive(Properties, Serialize, Debug)]
pub struct Named(pub String);

impl Default for Named {
    fn default() -> Self {
        Named ("No Name".to_string())
    }
}