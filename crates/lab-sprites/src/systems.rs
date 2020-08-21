
use lab_entities::prelude::*;
use bevy::prelude::*;
use crate::*;

use crate::Sprite as LabSprite;

pub fn sprite_despawn_system(
    mut commands: Commands,
    mut query : Query<(Entity, &TextureAtlasSprite, &world::Despawn, &Timer)>
){
    for (e, sprite, _dspawn, timer) in &mut query.iter(){
        if timer.finished {
            commands.despawn(e);
        }
    }
}


// refactor this out or change to default

pub fn load_world_sprites_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) { 
    
    let mut sprite_lib = SpriteLibrary::new();

    let world_sprite_labels = &["gravel","wall","floor","tile","gravel_h","brick","brick_door_closed","chair",
            "gravel_v","brick_window","brick_door_open","shelf","brick_window_broken","bed","table","fridge"];

    sprite_lib.catalog_sprites(&asset_server, &mut textures, 
        &mut texture_atlases, "resources/sprites/world.png", world_sprite_labels, (4,4));
    
        
    let letters = &[" ","!","\"","#","$","%","&","'","(",")","*","+",",","-",".", "/","0","1","2","3","4",
        "5","6","7","8","9",":",";","<","=",">","?","@",
        "a","b","c","d","e","f","g","h","i","j","k","l","m","n"
        ,"o","p","q","r","s","t","u","v","w","x","y","z"];

    sprite_lib.catalog_sprites(&asset_server, &mut textures, 
        &mut texture_atlases, "resources/fonts/alphabet.png", letters, (15,8));

    // placeholders for animated sprites

    sprite_lib.catalog_sprites( &asset_server, &mut textures, 
        &mut texture_atlases, "resources/sprites/sensei.png", &["player"], (1,1));
    sprite_lib.catalog_sprites( &asset_server, &mut textures, 
        &mut texture_atlases, "resources/sprites/mug.png", &["mug"], (1,1));
    sprite_lib.catalog_sprites( &asset_server, &mut textures, 
        &mut texture_atlases, "resources/sprites/hat-guy.png", &["npc"], (1,1));

    commands
        .insert_resource(sprite_lib);

}
