pub mod npcs;
pub mod objs;
pub mod player;
pub mod systems;

pub mod prelude {
    pub use crate::{npcs::*, objs::*, player::*, systems::*, *};
}
