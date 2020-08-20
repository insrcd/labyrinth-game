use bevy::{prelude::*};

mod player;
mod systems;
mod objs;
mod npcs;
mod world;

pub mod prelude {
    pub use crate::{
        npcs::*,
        objs::*,
        player::*,
        systems::*,
        world::*
    };     
}
