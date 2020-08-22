use bevy::{prelude::*};

pub mod systems;
pub mod maps;
pub mod text;

use systems::*;
use lab_sprites::SpriteLibrary;
use std::collections::HashMap;
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
        .add_resource(TilePalette { sprites: Vec::new(), interactions: HashMap::new() })
        .add_startup_system(make_tile_palette_system.system())
        .add_system(add_tiles_to_world_system.system())
        .add_system(builder_keyboard_system.system());
    }
}

pub struct TilePalette {
    pub components: HashMap<String, TileComponents>
}

impl TilePalette {
    
}

fn make_tile_palette_system(
    mut sprite_library: ResMut<SpriteLibrary>,
    mut palette: ResMut<TilePalette>
)  {
    for sprite in sprite_library.iter() {
        println!("Adding sprite {:?}", sprite);

        if let Some(comp) = palette.components.get(sprite.name){

           
        } else {
            palette.components.insert(TileComponents {
                tile_attributes: 
            })
        }

    }
}