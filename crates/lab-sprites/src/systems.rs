use bevy::prelude::*;
use crate::*;

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
        &mut texture_atlases, "resources/sprites/world.png", world_sprite_labels, (4,4), "world".to_string());

    sprite_lib.catalog_sprites(&asset_server, &mut textures, 
        &mut texture_atlases, "resources/sprites/roguelikecreatures.png", &["mob"], (8,9),"mobs".to_string());
    
    sprite_lib.catalog_sprites(&asset_server, &mut textures, 
            &mut texture_atlases, "resources/sprites/roguelikeitems.png", &["item"], (13,14), "items".to_string());


    sprite_lib.catalog_sprites(&asset_server, &mut textures, 
            &mut texture_atlases, "resources/sprites/walk_left.png", &["walk_left"], (13,1) ,"walk_left".to_string());

    sprite_lib.catalog_sprites(&asset_server, &mut textures, 
                &mut texture_atlases, "resources/sprites/walk_right.png", &["walk_right"], (13,1) ,"walk_right".to_string());
        
    let letters = &[" ","!","\"","#","$","%","&","'","(",")","*","+",",","-",".", "/","0","1","2","3","4",
        "5","6","7","8","9",":",";","<","=",">","?","@",
        "a","b","c","d","e","f","g","h","i","j","k","l","m","n"
        ,"o","p","q","r","s","t","u","v","w","x","y","z"];

    sprite_lib.catalog_sprites(&asset_server, &mut textures, 
        &mut texture_atlases, "resources/fonts/alphabet.png", letters, (15,8), 
        "letters".to_string());

    sprite_lib.catalog_sprites(&asset_server, &mut textures, 
            &mut texture_atlases, "resources/sprites/dungeon_tiles.png", &["dw_right_top","dw_center_top","dw_left_top","dw_left","dw_center","dw_right","dw_right_bottom","dw_right_center","dw_left_bottom"], (3,3),"dungeon".to_string());

    // placeholders for animated sprites

    commands
        .insert_resource(sprite_lib);
    
    println!("Done loading world sprites");
}
