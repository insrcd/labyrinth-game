use prelude::ItemHandle;

pub mod player;
pub mod systems;
pub mod objs;
pub mod npcs;

pub mod prelude {
    pub use crate::{
        npcs::*,
        objs::*,
        player::*,
        systems::*,
        *
    };     
}

