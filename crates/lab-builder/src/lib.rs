use bevy::{prelude::*};

pub mod systems;
pub mod maps;
pub mod text;

use systems::*;
use lab_sprites::SpriteLibrary;
use std::collections::{btree_map::{Keys, Values}, BTreeMap};
use lab_entities::prelude::TileComponents;
use lab_core;

pub mod prelude {
    pub use systems::*;
    pub use maps::*;
    pub use text::*;
    pub use crate::*;
}

pub enum RelativePosition {
    LeftOf,
    RightOf,
    Above,
    Below,
    Current
}

pub struct BuilderPlugin; 

impl Plugin for BuilderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_resource(TilePalette::default())
        .add_startup_system_to_stage(lab_core::stage::POST_INIT, make_tile_palette_system.system())
        .add_system_to_stage(lab_core::stage::PRE_UPDATE, add_tiles_to_world_system.system())
        .add_system(builder_keyboard_system.system())
        .add_system(update_tile_system.system())
        .add_system(select_tile_system.system());
    }
}

#[derive(Default, Clone)]
pub struct TilePalette {
    pub components: BTreeMap<String, TileComponents>
}

impl TilePalette {
    pub fn get_interaction(&self, name: String) -> Option<lab_entities::world::Interaction> {
        match self.components.get(&name) {
            Some(comps) => Some(comps.interaction),
            None => None
        }
    }

    pub fn tile_names(&self) -> Keys<'_, String, TileComponents>{
        self.components.keys()
    }

    pub fn iter(&self) -> Values<'_, String, TileComponents> {
        self.components.values()
    }

    pub fn tile_categories(&self) -> Vec<&str> {
        let mut categories : Vec<&str> = self.components.values().map(|m| &m.sprite.category[..]).collect();
        
        categories.sort();
        categories.dedup();
        
        categories
    }

    pub fn tiles_in_category(&self, category : &str) -> Vec<&TileComponents> {
        self.components.values().filter(|p| p.sprite.category == category).collect()        
    }

    pub fn update( &mut self, comp : &TileComponents) {

        if let Some(tc) = self.components.get_mut(&comp.sprite.name) {
           *tc = comp.clone();
        } else {
            self.components.insert(comp.sprite.name.clone(), comp.clone());
        }


    }
}
