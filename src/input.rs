use bevy::{
    prelude::*,
    render::{camera::Camera},
    input::{keyboard::KeyCode, Input, mouse::{MouseButtonInput, MouseMotion} },
};
use crate::world::*;
use crate::{assets::SpriteLibrary, player};
use std::time::Duration;

pub struct SelectedTile {
    tile_type: TileType,
    level : f32
}

impl Default for SelectedTile {
    fn default() -> SelectedTile {
        SelectedTile { tile_type:TileType::Floor , level: 0.}
    }
}
pub struct InputPlugin;

#[allow(dead_code)]
pub mod stage {
    pub const INPUT: &'static str = "input";
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<SelectedTile>()
            .add_system(keyboard_input_system.system())
            .add_system(track_mouse_movement.system());
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub struct State {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    mouse_motion_event_reader: EventReader<MouseMotion>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}
pub struct Mouse {
    pub position: Vec2
}
pub struct MouseClick {
    ui_position: Vec2,
    map_position: Vec3
}

fn track_mouse_movement(
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut state: ResMut<State>,
    windows: Res<Windows>,
    mut mouse_query: Query<&mut Mouse>,
    mut camera_query: Query<(&Camera, &Translation)>) {
        let mut camera_offset_x : f32 = 0.;
        let mut camera_offset_y : f32 = 0.;
        
        for (c, t) in &mut camera_query.iter(){
            if *(c.name.as_ref()).unwrap_or(&"".to_string()) == "UiCamera" {
                camera_offset_x = t.x();
                camera_offset_y = t.y();
            }
        }

        /*for window in windows.iter() {
            println!("{:?}",window);
        }*/

        let window = windows.iter().last().unwrap();
        let x_window_offset = window.width;
        let y_window_offset = window.height;
        
        for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
            //println!("{},{} - {},{}", camera_offset_x, camera_offset_y, event.position.x(), event.position.y() );

            for mut mouse in &mut mouse_query.iter(){
                mouse.position = Vec2::new(event.position.x() + camera_offset_x - (x_window_offset/2) as f32, event.position.y() + camera_offset_y - (y_window_offset/2) as f32);
            }
        }
}

fn add_tiles_system (
    mut commands: Commands,
     selected_tile: Res<SelectedTile>, 
    input: Res<Input<KeyCode>>, 
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_query: Query<&Mouse>,
    mut query: Query<(&crate::player::Player, &Translation, &crate::player::Moving)>
) {    
    let tile_size = crate::world::settings::TILE_SIZE;

    for mouse in &mut mouse_query.iter(){
        if mouse_input.just_pressed(MouseButton::Left) {
            let mut x = mouse.position.x() ;
            let mut y = mouse.position.y() ;
            
            println!("Mouse at {:?},{:?}", x, y);

            let grid_x = x  / tile_size;
            let grid_y = y  / tile_size;
            
            println!("{},{}", grid_x as i32 % 96, grid_y as i32 % 96);
            
            x = grid_x.round() * tile_size;
            y = grid_y.round() * tile_size;

            
            println!("Placing tile at {:?},{:?}", x, y);

            
            let mut interaction : fn (Attributes) -> (bool,TileType) = |_| { (false, TileType::Key ) };
            let hardness = match selected_tile.tile_type {
                TileType::Wall(h ) =>  {
                    h
                }, 
                TileType::Brick(h ) =>  {
                    h
                }, TileType::BrickWindow(h ) =>  {
                    interaction = |_| { ( true, TileType::BrickWindowBroken ) };
                    h
                }, TileType::BrickDoorClosed(h ) =>  {
                    interaction = |_| { ( true, TileType::BrickDoorOpen ) };
                    h
                }, 
                _ => Hardness(0.)
            };
            



            commands.spawn(TileComponents {
                hardness: hardness,
                tile_type: selected_tile.tile_type,
                location: Location(x, y, selected_tile.level),
                interaction: crate::world::Interaction { call: interaction },
                ..Default::default()
            });
        }
    }
    
    for (_p, t, m) in &mut query.iter(){
        
        if input.just_pressed(KeyCode::F2) {
            let mut x = f32::abs ( t.0.x() );
            let mut y = f32::abs ( t.0.y() );

            if t.0.x() < 0. {
                x = 0. - (x + (x as u32 % 96)  as f32)
            } else {
                x -= (x as u32 % 96) as f32
            }
            if t.0.y() < 0. {
                y = 0. - (y + (y as u32 % 96)  as f32)
            } else {
                y -= (y as u32 % 96) as f32
            }
            println!("({},{}) ({},{})",x,y,t.0.x(),t.0.y());

            match m.2 {
                player::Direction::Left => x -= tile_size,
                player::Direction::Up => x += tile_size,
                player::Direction::Down =>  y -= tile_size,
                player::Direction::Right =>  y += tile_size,
                player::Direction::Stationary =>  x += tile_size
            }

            let loc =  Location(x, y, 1.);
            
            println!("Adding tile to {:?}", loc);
            
            commands.spawn(TileComponents {
                hardness: Hardness(1.),
                tile_type: selected_tile.tile_type,
                location: loc,
                ..Default::default()
            });
        }
    }
}




fn keyboard_input_system(
    mut commands: Commands,
    windows : Res<Windows>,
    keyboard_input: Res<Input<KeyCode>>, 
    mut selected_tile: ResMut<SelectedTile>, 
    lib : Res<SpriteLibrary>,
    mut query: Query<(&player::Player, &mut Translation, &mut player::Moving)>) {

    let player_speed = 48.;

    let mut movement = player::Direction::Stationary;
    
    use strum::IntoEnumIterator; 


    let window = windows.iter().last().unwrap();

    if keyboard_input.just_pressed(KeyCode::RBracket) {
        let mut tile_types :  Vec<TileType> = Vec::new();

        for ty in TileType::iter() {
            tile_types.push(ty);
        }
    
        let idx = tile_types.iter().position(|x| *x == selected_tile.tile_type );

        match idx {
            Some(i) => {
                let final_type = match tile_types[(i+1) % (tile_types.len()-1)] {
                    TileType::Brick(_) => TileType::Brick(Hardness(1.)),
                    TileType::BrickWindow(_) => TileType::BrickWindow(Hardness(1.)),
                    TileType::BrickDoorClosed(_) => TileType::BrickDoorClosed(Hardness(1.)),
                    TileType::Wall(_) => TileType::Wall(Hardness(1.)),
                    _ => tile_types[(i+1) % (tile_types.len()-1)]
                };
                selected_tile.tile_type = final_type;

                println!("Tile Selected: {:?}", final_type);
            },
            None => {}
        }
    }

    if keyboard_input.just_pressed(KeyCode::LBracket) {
        let mut tile_types :  Vec<TileType> = Vec::new();

        for ty in TileType::iter() {
            tile_types.push(ty);
        }
    
        let idx = tile_types.iter().position(|x| *x == selected_tile.tile_type );
        match idx {
            Some(mut i) => {
                if i == 0 {
                    i = tile_types.len() -1;
                }

                let final_type = match tile_types[i-1] {
                    TileType::Brick(_) => TileType::Brick(Hardness(1.)),
                    TileType::BrickWindow(_) => TileType::BrickWindow(Hardness(1.)),
                    TileType::BrickDoorClosed(_) => TileType::BrickDoorClosed(Hardness(1.)),
                    TileType::Wall(_) => TileType::Wall(Hardness(1.)),
                    _ => tile_types[i-1]
                };

                selected_tile.tile_type = final_type;
                
                println!("Tile Selected: {:?}", final_type);
            },
            None => {}
        }
    }

    if keyboard_input.just_pressed(KeyCode::Add) {
        selected_tile.level += 1.;
        lib.write_despawning_text(&mut commands, "Welcome to Labyrinth, the Game!".to_string(), 
                        Duration::from_secs(5), 
                        Vec3::new(16. - (window.width/2) as f32, 16. - (window.height/2) as f32, 100.)
                    )
    }
    if keyboard_input.just_pressed(KeyCode::Subtract) {
        selected_tile.level += 1.;
        lib.write_despawning_text(&mut commands, "Welcome to Labyrinth, the Game!".to_string(), 
                        Duration::from_secs(5), 
                        Vec3::new(16. - (window.width/2) as f32, 16. - (window.height/2) as f32, 100.)
                    )
    }

    if keyboard_input.just_pressed(KeyCode::W) {
        movement = player::Direction::Up;
    }

    if keyboard_input.just_pressed(KeyCode::S) {
        movement = player::Direction::Down;
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        movement = player::Direction::Left;
    }
    if keyboard_input.just_pressed(KeyCode::D) {
        movement = player::Direction::Right;
    }

    for (_player, mut loc, mut moving) in &mut query.iter() {   
        let old_loc = Location::from_translation(*loc);

        match movement {
            player::Direction::Up => *loc.0.y_mut() += player_speed,
            player::Direction::Down => *loc.0.y_mut() -= player_speed,
            player::Direction::Left => *loc.0.x_mut() -= player_speed,
            player::Direction::Right => *loc.0.x_mut() += player_speed,
            player::Direction::Stationary => {
            }
        }

        if movement != player::Direction::Stationary {
            *moving = player::Moving(old_loc, Location::from_translation(*loc), movement);
        }
    }      
}