use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};
use std::collections::HashMap;

#[derive(Clone)]
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

    pub fn add(&mut self, name : &'static str, sprite: Sprite){
        self.library.as_mut().insert(name, sprite);
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

pub struct Location (pub f32, pub f32, pub f32);
pub struct Tile (pub TileType);
#[derive(Debug)]
pub enum TileType {
    Wall,
    Floor,
    Lava,
    Bar,
    Grass
}
pub struct Collidable;
pub struct Visible;

/*
struct Map {
    tiles: &[Tile],
    placeable: &[Placeable]
}*/
