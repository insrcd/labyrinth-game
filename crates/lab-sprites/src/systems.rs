use bevy::prelude::*;
use crate::*;


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

    sprite_lib.catalog_sprites(&asset_server, &mut textures, 
            &mut texture_atlases, "resources/sprites/dungeon_tiles.png", &["dw_right_top","dw_center_top","dw_left_top","dw_left","dw_center","dw_right","dw_right_bottom","dw_right_center","dw_left_bottom"], (3,3));

    // placeholders for animated sprites
    sprite_lib.catalog_sprites( &asset_server, &mut textures, 
        &mut texture_atlases, "resources/sprites/mug.png", &["mug"], (1,1));
    sprite_lib.catalog_sprites( &asset_server, &mut textures, 
        &mut texture_atlases, "resources/sprites/player_sprites.png", &["npc"], (1,1));

    commands
        .insert_resource(sprite_lib);
    
    println!("Done loading world sprites");
}
