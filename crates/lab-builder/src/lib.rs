use bevy::{prelude::*};

pub mod systems;
pub mod maps;
pub mod text;

use systems::*;
use lab_sprites::SpriteLibrary;
use std::collections::{btree_map::{Keys, Values}, BTreeMap};
use lab_entities::prelude::TileComponents;

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

pub struct InputPlugin;

#[allow(dead_code)]
pub mod stage {
    pub const INPUT: &'static str = "builder";
}

pub struct BuilderPlugin; 

impl Plugin for BuilderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_resource(TilePalette::default())
        .add_startup_system(make_tile_palette_system.system())
        .add_system(add_tiles_to_world_system.system())
        .add_system(builder_keyboard_system.system());
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
}

fn make_tile_palette_system(
    mut sprite_library: ResMut<SpriteLibrary>,
    mut palette: ResMut<TilePalette>
)  {
    for sprite in sprite_library.iter() {
        println!("Adding sprite {:?}", sprite);

        if let Some(comp) = palette.components.get(&sprite.name){
           // already added
            println!("Duplicate sprite detected sprite {:?}", sprite);
        } else {
            palette.components.insert(sprite.name.clone(), TileComponents {
                sprite: sprite.clone(),
                ..Default::default()
            });
        }

    }
}