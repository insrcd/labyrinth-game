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