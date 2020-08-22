
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
pub fn static_text_system(
    mut commands: Commands,
    mut query : Query<(Entity, &StationaryLetter, &mut Translation)>,    
    mut player_query : Query<(Entity, &Player, Changed<Movement>)>
){
    for (e, _player, movement) in &mut player_query.iter(){

        for (e, _letter, mut translation) in &mut query.iter(){

            let old_loc = movement.0;
            let new_loc = movement.1;

            let x_change = old_loc.0 - new_loc.0;
            let y_change = old_loc.1 - new_loc.1;

            *translation.x_mut() -= x_change;
            *translation.y_mut() -= y_change;
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

    let player_sprite_labels = &[
        "move_down_1","move_down_2","move_up_1","move_up_2","move_left_1","move_left_2","dead",
        "head_down","left_stop","move_up_3","look_far_up","look_far_up_2","move_right_1","move_right_2",
        "laugh_1","laugh_2","pensive","oops","ready_left","jump_left", "pensive_left",
        "climb_1","climb_2","surprise","yay","yay_2","yay_3","tied_up"];

    sprite_lib.catalog_sprites(&asset_server, &mut textures, 
        &mut texture_atlases, "resources/sprites/player_sprites.png", player_sprite_labels, (7,4));
    
        
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
