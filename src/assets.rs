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


pub fn load_world_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) { 
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

    let texture = textures.get(&texture_handle).unwrap();
    let player_texture = textures.get(&player_texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 4, 4);
    
    let player_texture_atlas = TextureAtlas::from_grid(player_texture_handle, player_texture.size, 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let player_texture_atlas_handle = texture_atlases.add(player_texture_atlas);

    let mut sprite_lib = SpriteLibrary::new();

    sprite_lib.add("wall", Sprite::new("wall", 1, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add("floor", Sprite::new("floor", 2, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add("player", Sprite::new("player", 0, player_texture_atlas_handle.clone(), player_texture.size.x() as u32, player_texture.size.y() as u32));

    commands
    .insert_resource(sprite_lib)
    .spawn(Camera2dComponents::default())
    .spawn(UiCameraComponents::default())
    .spawn((Player, Named("Adam".to_string()), Location(0., 0., 0.)));
/*    .spawn(SpriteSheetComponents {
        translation: Translation(Vec3::new(0.0, 0.0, 0.0)),
        scale: Scale(6.0),
        sprite: TextureAtlasSprite::new(1 as u32),
        texture_atlas: texture_atlas_handle,
        ..Default::default()
    }).with(Tile(TileType::Wall)).with(Named("wall".to_string()))
    .spawn(SpriteSheetComponents {
        translation: Translation(Vec3::new(16.0*6., 16.0*6., 0.0)),
        scale: Scale(6.0),
        sprite: TextureAtlasSprite::new(2 as u32),
        texture_atlas: texture_atlas_handle,
        ..Default::default()
    }).with(Tile(TileType::Floor)).with(Named("rock".to_string()));*/

    /*
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            translation:  Translation(Vec3::new(300.0, 100.0, 0.0)),
            ..Default::default()
        });*/
}