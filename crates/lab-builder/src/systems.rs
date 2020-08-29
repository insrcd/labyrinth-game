use bevy::{prelude::*, 
    render::{camera::Camera}};

use lab_entities::prelude::*;
use lab_sprites::*;
use lab_world::*;
use crate::{BuilderSettings, MovingTile};
use lab_input::{Mouse, SelectedTile, ScrollState, MouseClickEvent, MouseState};
use lab_core::prelude::{Location, WorldLocation};


pub fn make_world_catalog_system(
    mut sprite_library: ResMut<SpriteLibrary>,
    mut palette: ResMut<WorldCatalog>
)  {
    for sprite in sprite_library.iter() {
        //println!("Adding sprite {:?}", sprite);

        if let Some(_) = palette.components.get(&sprite.name){
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
pub fn update_tile_system (mouse : ResMut<Mouse>,
    windows: Res<Windows>, 
    palette: Res<WorldCatalog>, 
    scroll_state: Res<ScrollState>, 
    selected_tile: Res<SelectedTile>, 
    mut camera_query: Query<(&Camera, Changed<Translation>)>,
    mut m_tile_query: Query<(&MovingTile, &mut Translation, &Draw)>,
    mut f_tile_query: Query<(&FreeTile, &mut Translation, &Draw)>){
    
    
    let window = windows.iter().last().unwrap();

    for (c, t) in &mut camera_query.iter(){
        if *(c.name.as_ref()).unwrap_or(&"".to_string()) != "UiCamera" {
        
            let camera_offset_x = t.x();
            let camera_offset_y = t.y();
            for (_ft, mut t, _d) in &mut f_tile_query.iter(){        
                let mouse_tile = palette.tiles_in_category(&selected_tile.category)[selected_tile.tile];
                let sprite_width = mouse_tile.sprite.size().x() * scroll_state.current_scale;
                let sprite_height = mouse_tile.sprite.size().y() * scroll_state.current_scale;

                let new_loc = Vec3::new((window.width as f32 / 2.) - sprite_width + camera_offset_x - 20., -(window.height as f32 / 2.)+sprite_height + camera_offset_y, 100.);

                *t.0.x_mut() = new_loc.x();
                *t.0.y_mut() = new_loc.y();
            }
        }
    }
    for (_ft, mut t, _d) in &mut m_tile_query.iter(){
        *t.0.x_mut() = mouse.position.x();
        *t.0.y_mut() = mouse.position.y();
    }
    
}

pub fn add_tiles_to_world_system (
    mut commands: Commands,
    settings : ResMut<BuilderSettings>,
    selected_tile: Res<SelectedTile>, 
    scroll_state: Res<ScrollState>, 
    palette: Res<WorldCatalog>,
    mouse_input: Res<Input<MouseButton>>,
    mouse : ResMut<Mouse>,
    mouse_events : ResMut<Events<MouseClickEvent>>,
    mut mouse_click : ResMut<MouseState>,
    mut interaction_query: Query<(Entity, &SpriteInfo, &mut Translation, &Scale, &Draw)>,
    mut moving_tile_query: Query<(Entity, &MovingTile, &mut Translation)>
) {    
    for clicks in &mut mouse_click.click_events.iter(&mouse_events) {
        match clicks.button {
            MouseButton::Left => {
                let x = clicks.world_position.x();
                let y = clicks.world_position.y();

                let components = palette.tiles_in_category(&selected_tile.category)[selected_tile.tile as usize];
        
                let st = selected_tile.clone();                    

                
                for (entity, _tt, mut t) in &mut moving_tile_query.iter() {
                    *t = Translation::new(x, y, st.level);
                    commands.remove_one::<MovingTile>(entity);
                    return
                }

                if settings.move_mode {
                    for (entity, si, 
                            t, scale, _d) in &mut interaction_query.iter() {
                        let true_location = mouse.position;
                        
                        let (x1, y1) = ( t.x() - (si.width/2) as f32* scale.0,  t.y() - (si.height / 2 )as f32* scale.0) ;
                        let (x2, y2) = (t.x() + ((si.width/2) as f32 * scale.0), t.y() + ((si.height/2) as f32 * scale.0));
                        
                        //println!("mouse click: {:?} tile location: ({:?},{:?}) ({:?},{:?})",mouse.position, x1,y1,x2,y2);

                        if  true_location.x() >= x1 && true_location.x() <= x2 
                            && true_location.y() >= y1 && true_location.y() <= y2 {
                                println!("Click on sprite {}", si.name);

                                commands.insert_one(entity, MovingTile);

                                return;
                        }
                    }
                }

                let mut clone = components.clone();
                let sprite: SpriteInfo = clone.sprite.clone();

                clone.location = Location(x, y, st.level,  WorldLocation::World);
                        
                commands
                    .spawn(sprite.to_components( Vec3::new(x,y,st.level), Scale(scroll_state.current_scale)))
                    .with_bundle(clone); 
            }
            MouseButton::Right => {}
            MouseButton::Middle => {}
            MouseButton::Other(_) => {}
            
        }
    }
}
pub struct FreeTile;

pub fn builder_settings_system (
    mut settings : ResMut<BuilderSettings>,
    keyboard_input: Res<Input<KeyCode>>, 
){
    if keyboard_input.just_pressed(KeyCode::M) {
        (*settings).move_mode =  settings.move_mode == false;
    }
}

pub fn builder_keyboard_system (
    mut commands: Commands,
    windows : Res<Windows>,
    scroll : Res<ScrollState>,
    keyboard_input: Res<Input<KeyCode>>, 
    mut selected_tile: ResMut<SelectedTile>, 
    mut palette: ResMut<WorldCatalog>, 
    mut camera_query: Query<(&Camera, &Translation)>,
    mut free_tile: Query<(Entity, &FreeTile)>) {

    let mut camera_offset_x : f32 = 0.;
    let mut camera_offset_y : f32 = 0.;
    

    let window = windows.iter().last().unwrap();

    for (c, t) in &mut camera_query.iter(){
        if *(c.name.as_ref()).unwrap_or(&"".to_string()) != "UiCamera" {
            camera_offset_x = t.x();
            camera_offset_y = t.y();
        }
    }
        
    if keyboard_input.just_pressed(KeyCode::Apostrophe) {
        let categories = palette.tile_categories();
        let pos = categories.iter().position(|&s| s == selected_tile.category).unwrap();

        selected_tile.tile = 0;
        selected_tile.category = palette.tile_categories()[(pos + 1) % categories.len()].to_string();
        println!("Selected category: {}", selected_tile.category);
    }
    
    if keyboard_input.just_pressed(KeyCode::Semicolon) {
        let categories = palette.tile_categories();
        
        let pos = categories.iter().position(|&s| s == selected_tile.category).unwrap();
        
        selected_tile.tile = 0;

        selected_tile.category  = if pos != 0 {
            categories[(pos - 1)].to_string()
        } else {            
            categories[palette.tile_categories().len() -1].to_string()
        };

        println!("Selected category: {}", selected_tile.category);
    }

    if keyboard_input.just_pressed(KeyCode::RBracket) {
        selected_tile.tile = change_selected_sprite(&mut commands, 1,
             &mut palette, 
             &mut free_tile,
             (*selected_tile).category.as_ref(), 
             selected_tile.tile, 
             (*scroll).current_scale,
             (window.width as f32, window.height as f32),
             (camera_offset_x, camera_offset_y));
    } 
    if keyboard_input.just_pressed(KeyCode::LBracket) {
        selected_tile.tile = change_selected_sprite(&mut commands, -1,
            &mut palette, 
            &mut free_tile,
            (*selected_tile).category.as_ref(), 
            selected_tile.tile, 
            (*scroll).current_scale,
            (window.width as f32, window.height as f32),
        (camera_offset_x, camera_offset_y));
    }  
    if keyboard_input.just_pressed(KeyCode::Add) {
        selected_tile.level += 1.;
        println!("Level changed to {}",selected_tile.level.clone());         
    } 
    if keyboard_input.just_pressed(KeyCode::Subtract) {
        selected_tile.level -= 1.;
        println!("Level changed to {}",selected_tile.level.clone());        
    }
}

fn change_selected_sprite(commands : &mut Commands, 
    change : i32, 
    palette: &ResMut<WorldCatalog>, 
    free_tile: &mut Query<(Entity, &FreeTile)>,
    category : &str, 
    tile : usize,
    current_scale : f32,
    window_size: (f32, f32),
    camera_offset: (f32, f32)) -> usize {
        let len = palette.tiles_in_category(category).len() as i32;

        let mut idx = (tile as i32 + change) % len as i32;

        if idx < 0 {
            idx = len + idx;
        }        

        let mouse_tile = palette.tiles_in_category(category)[idx as usize];
        
        println!("Changed to tile {:?}", mouse_tile.sprite);
        
        let scaled_size = mouse_tile.sprite.scaled_size(current_scale);
        let scaled_location = Vec3::new(window_size.0/2.-scaled_size.x()- 20. + camera_offset.0, 
            -(window_size.1 as f32 / 2.)+scaled_size.y() + camera_offset.1, 
            100.);
        if let Some((entity, _t)) = &mut free_tile.iter().into_iter().last() {
            let comps = mouse_tile.sprite.to_components(scaled_location, Scale(current_scale));
                        
            commands                
                .insert(*entity, (comps.sprite, comps.texture_atlas.clone(), Scale(current_scale), comps.translation, FreeTile));
            
        } else {
            commands                
                .spawn(mouse_tile.sprite.to_components(scaled_location,Scale(current_scale)))
                .with(FreeTile);
        }

        idx as usize
}