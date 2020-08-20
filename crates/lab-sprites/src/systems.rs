
use lab_entities::prelude::*;
use bevy::prelude::*;
use crate::*;

use crate::Sprite as LabSprite;

pub fn text_despawn(
    mut commands: Commands,
    mut query : Query<(Entity, &TextureAtlasSprite, &world::Despawn, &Timer)>
){
    for (e, sprite, _dspawn, timer) in &mut query.iter(){
        if timer.finished {
            commands.despawn(e);
        }
    }
}



pub fn load_world_sprites_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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

    let alphabet_texture_handle = asset_server
    .load_sync(
        &mut textures,
        "resources/fonts/alphabet.png",
    )
    .unwrap();

    let texture = textures.get(&texture_handle).unwrap();
    let player_texture = textures.get(&player_texture_handle).unwrap();
    let npc_texture = textures.get(&npc_texture_handle).unwrap();
    let ab_texture = textures.get(&alphabet_texture_handle).unwrap();

    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 4, 4);
    let player_texture_atlas = TextureAtlas::from_grid(player_texture_handle, player_texture.size, 7, 1);
    let ab_texture_atlas = TextureAtlas::from_grid(alphabet_texture_handle, ab_texture.size, 15, 8);
    let npc_texture_atlas = TextureAtlas::from_grid(alphabet_texture_handle, npc_texture.size, 1, 1);
    
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let player_texture_atlas_handle = texture_atlases.add(player_texture_atlas);
    let npc_texture_atlas_handle = texture_atlases.add(npc_texture_atlas);
    let ab_texture_atlas_handle = texture_atlases.add(ab_texture_atlas);

    let mut sprite_lib = SpriteLibrary::new();
    
    sprite_lib.add(LabSprite::new("gravel",  0, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(LabSprite::new("wall",  1, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(LabSprite::new("floor", 2, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(LabSprite::new("tile", 3, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(LabSprite::new("gravel_h", 4, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(LabSprite::new("brick", 5, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(LabSprite::new("brick_door_closed", 6, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(LabSprite::new("chair", 7, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(LabSprite::new("gravel_v", 8, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(LabSprite::new("brick_window", 9, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(LabSprite::new("brick_door_open", 10, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(LabSprite::new("shelf", 11, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(LabSprite::new("brick_window_broken", 12, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(LabSprite::new("bed", 13, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(LabSprite::new("table", 14, texture_atlas_handle.clone(), 16, 16));
    sprite_lib.add(LabSprite::new("fridge", 15, texture_atlas_handle.clone(), 16, 16));

    let letters = [" ","!","\"","#","$","%","&","'","(",")","*","+",",","-",".", "/","0","1","2","3","4",
                             "5","6","7","8","9",":",";","<","=",">","?","@",
                             "a","b","c","d","e","f","g","h","i","j","k","l","m","n"
                            ,"o","p","q","r","s","t","u","v","w","x","y","z"];
    for n in 0..letters.len() {
        let label = String::from(format!("l_{}", letters[n]));
        println!("Adding letter {}", label);
        sprite_lib.add(LabSprite::new(Box::leak(label.into_boxed_str()), n as u32, ab_texture_atlas_handle.clone(), 24, 24));
    }
    sprite_lib.add(LabSprite::new("player", 0, player_texture_atlas_handle.clone(), player_texture.size.x() as u32, player_texture.size.y() as u32));
    sprite_lib.add(LabSprite::new("npc", 0, npc_texture_atlas_handle.clone(), npc_texture.size.x() as u32, npc_texture.size.y() as u32));

    commands
        .insert_resource(sprite_lib);

}