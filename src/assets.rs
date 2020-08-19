use bevy::{
    prelude::*
};

use crate::world::Location;
use crate::player::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Sprite {
    pub name:  &'static str,
    pub atlas_sprite : u32,
    pub atlas_handle : Handle<TextureAtlas>,
    pub height: u32,
    pub width: u32
}

pub struct SpriteLibrary {
    library: Box<HashMap<&'static str, Sprite>>
}

impl SpriteLibrary {
    pub fn new () -> SpriteLibrary {
        SpriteLibrary {
            library : Box::new(HashMap::new())
        }
    }

    pub fn add(&mut self,sprite: Sprite){
        self.library.as_mut().insert(sprite.name, sprite);
    }

    pub fn get(&self, name : &str) -> Sprite {
        self.library.as_ref().get(name).unwrap().clone()
    }
}

impl Sprite {
    pub fn new (name : &'static str, sprite_idx: u32, handle: Handle<TextureAtlas>, width: u32, height: u32) -> Sprite {
         return Sprite {
             name: name.clone(),
             atlas_sprite: sprite_idx,
             atlas_handle: handle,
             width,
             height
         }
    }
}


pub fn load_world_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) { 

    // Refactor this, make grabbing textures easier.
    let texture_handle = asset_server
    .load_sync(
        &mut textures,
        "resources/sprites/world.png",
    )
    .unwrap();
    
    let player_texture_handle = asset_server
    .load_sync(
        &mut textures,
        "resources/sprites/gabe-idle-run.png",
    )
    .unwrap();
    
    let npc_texture_handle = asset_server
    .load_sync(
        &mut textures,
        "resources/sprites/sensei.png",
    )
    .unwrap();

    let texture = textures.get(&texture_handle).unwrap();
    let player_texture = textures.get(&player_texture_handle).unwrap();
    let npc_texture = textures.get(&npc_texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 4, 4);
    
    let player_texture_atlas = TextureAtlas::from_grid(player_texture_handle, player_texture.size, 7, 1);
    let npc_texture_atlas = TextureAtlas::from_grid(npc_texture_handle, npc_texture.size, 1, 1);
    
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let player_texture_atlas_handle = texture_atlases.add(player_texture_atlas);
    let npc_texture_atlas_handle = texture_atlases.add(npc_texture_atlas);

    let mut sprite_lib = SpriteLibrary::new();
    
    sprite_lib.add(Sprite::new("gravel",  0, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(Sprite::new("wall",  1, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(Sprite::new("floor", 2, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(Sprite::new("tile", 3, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(Sprite::new("gravel_h", 4, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(Sprite::new("brick", 5, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(Sprite::new("brick_door_closed", 6, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(Sprite::new("chair", 7, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(Sprite::new("gravel_v", 8, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(Sprite::new("brick_window", 9, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(Sprite::new("brick_door_open", 10, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(Sprite::new("shelf", 11, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(Sprite::new("brick_window_broken", 12, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(Sprite::new("bed", 13, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(Sprite::new("table", 14, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(Sprite::new("fridge", 15, texture_atlas_handle.clone(), 16, 16));

    sprite_lib.add(Sprite::new("player", 0, player_texture_atlas_handle.clone(), player_texture.size.x() as u32, player_texture.size.y() as u32));
    sprite_lib.add(Sprite::new("npc", 0, npc_texture_atlas_handle.clone(), npc_texture.size.x() as u32, npc_texture.size.y() as u32));

    commands
        .insert_resource(sprite_lib);

}