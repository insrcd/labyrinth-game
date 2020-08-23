use bevy::{prelude::*, 
    render::{camera::Camera}, input::mouse::MouseWheel,};

use lab_entities::prelude::*;
use lab_sprites::*;
use lab_entities::player;
use std::time::Duration;
use crate::TilePalette;
use lab_input::{Mouse, SelectedTile, State};


pub fn make_tile_palette_system(
    mut sprite_library: ResMut<SpriteLibrary>,
    mut palette: ResMut<TilePalette>
)  {
    println!("Making palette from {} sprites", sprite_library.len());
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

pub fn add_tiles_to_world_system (
    mut commands: Commands,
    selected_tile: Res<SelectedTile>, 
    palette: Res<TilePalette>,
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_query: Query<&Mouse>
) {    
    let tile_size = lab_world::settings::TILE_SIZE;

    for mouse in &mut mouse_query.iter(){
        if mouse_input.just_pressed(MouseButton::Left) {
            let st = selected_tile.clone();

            let mut x = mouse.position.x() ;
            let mut y = mouse.position.y() ;
            
            println!("Mouse at {:?},{:?}", x, y);

            let grid_x = x  / tile_size;
            let grid_y = y  / tile_size;
            
            println!("{},{}", grid_x as i32 % 96, grid_y as i32 % 96);
            
            x = grid_x.round() * tile_size;
            y = grid_y.round() * tile_size;

             if let Some(components) = palette.components.get(&st.name.clone()){
                 
                println!("Placing tile at {:?},{:?}", x, y);

                let mut clone = components.clone();

                clone.location = Location(x, y, st.level,  world::WorldLocation::World);
                
                commands.spawn(clone)
                    .spawn(components.sprite.to_components( Vec3::new(x,y,st.level.clone()), Scale(6.)));
             }
        }
    }
}



pub fn builder_keyboard_system (
    mut commands: Commands,
    windows : Res<Windows>,
    keyboard_input: Res<Input<KeyCode>>, 
    mut selected_tile: ResMut<SelectedTile>, 
    mut palette: ResMut<TilePalette>, 
    lib : Res<SpriteLibrary>,
    mut query: Query<(&player::Player, &mut Translation, &mut player::Movement)>,
    mut camera_query: Query<(&Camera, &Translation)>) {
    let mut camera_offset_x : f32 = 0.;
    let mut camera_offset_y : f32 = 0.;
    
    for (c, t) in &mut camera_query.iter(){
        if *(c.name.as_ref()).unwrap_or(&"".to_string()) == "UiCamera" {
            camera_offset_x = t.x();
            camera_offset_y = t.y();
        }
    }

    let window = windows.iter().last().unwrap();

    let text_duration: u64 = 750 ;

    let mut write_message = |message| {
        lib.write_despawning_text(&mut commands, message, 
        Duration::from_millis(text_duration), 
                        Vec3::new(16. + camera_offset_x - (window.width/2) as f32, 16. +camera_offset_y - (window.height/2) as f32, 100.)
                    );
    };

    let count = palette.components.len() as u32;
        
    if keyboard_input.just_pressed(KeyCode::RBracket) {
       if  selected_tile.tile == 0 {
           selected_tile.tile = count -1;
       }  else {
           selected_tile.tile = selected_tile.tile - 1;
       }

       if let Some((idx, name)) = palette.tile_names().enumerate().nth(selected_tile.tile as usize) {
            selected_tile.name = name.clone();
            
            write_message(format!("Tile changed to {}", name)); 
       }
    } else if keyboard_input.just_pressed(KeyCode::LBracket) {
        selected_tile.tile = selected_tile.tile + 1 % count; 
         
        if let Some((idx, name)) = palette.tile_names().enumerate().nth(selected_tile.tile as usize) {
            selected_tile.name = name.clone();
             write_message(format!("Tile changed to {}", name)); 
        }

    } else if keyboard_input.just_pressed(KeyCode::Add) {
        selected_tile.level += 1.;
        write_message(format!("Level changed to {}",selected_tile.level.clone()));         
    } else if keyboard_input.just_pressed(KeyCode::Subtract) {
        selected_tile.level -= 1.;
        write_message(format!("Level changed to {}",selected_tile.level.clone()));         
    }
}
