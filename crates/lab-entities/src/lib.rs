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

#[derive(Clone, Debug, Default)]
pub struct Inventory {
    pub items: Vec<ItemHandle>
}


impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            items: Vec::<ItemHandle>::new()
        }
    }
    pub fn has(&self, predicate: fn (&ItemHandle) -> bool) -> bool {
        self.items.iter().any(|i| predicate(i))
    }
}