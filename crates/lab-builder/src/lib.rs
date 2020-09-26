use bevy::prelude::*;

pub mod maps;
pub mod systems;
pub mod text;

use lab_core::prelude::*;
use lab_entities::player::NonPlayer;
use lab_sprites::SpriteInfo;
use systems::*;

pub mod prelude {
    pub use crate::*;
    pub use maps::*;
    pub use systems::*;
    pub use text::*;
}

#[derive(Copy, Clone, PartialEq)]
pub enum RelativePosition {
    LeftOf,
    RightOf,
    Above,
    Below,
    Current,
}

pub struct BuilderPlugin;

impl Plugin for BuilderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<BuilderSettings>()
            // system to init the tile palette
            .add_startup_system_to_stage(
                lab_core::stages::POST_INIT,
                make_world_catalog_system.system(),
            )
            // system that will add tiles on click
            .add_system_to_stage(
                lab_core::stages::PRE_UPDATE,
                add_tiles_to_world_system.system(),
            )
            .add_system(builder_keyboard_system.system())
            .add_system(update_tile_system.system())
            // System for changing builder settings
            .add_system(builder_settings_system.system());
    }
}

#[derive(Default)]
pub struct BuilderSettings {
    pub move_mode: bool,
}

/// Mark a tile as moving (i.e. being dragged)
pub struct MovingTile;

#[derive(Bundle, Default, Debug)]
pub struct MobComponents {
    pub npc: NonPlayer,
    pub named: Named,
    pub movement: Movement,
    pub sprite: SpriteInfo,
    pub inventory: Inventory,
    pub state: ObjectState,
    pub timer: Timer,
    pub location: Location,
    pub zoomable: Zoomable,
    pub interactable_type: InteractableType
}

impl Clone for MobComponents {
    fn clone(&self) -> Self {
        MobComponents {
            npc: self.npc.clone(),
            named: self.named.clone(),
            movement: self.movement.clone(),
            sprite: self.sprite.clone(),
            inventory: self.inventory.clone(),
            state: self.state.clone(),
            timer: self.timer.clone(),
            location: self.location.clone(),
            zoomable: self.zoomable.clone(),
            interactable_type: self.interactable_type.clone()
        }
    }
}

impl MobComponents {
    #[allow(dead_code)]
    fn new(name: String) -> Self {
        MobComponents {
            named: Named(name),
            ..Default::default()
        }
    }
}
