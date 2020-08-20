use bevy::{prelude::*};

use lab_entities::prelude::*;
use lab_sprites::*;
use lab_input::*;
use lab_entities::player;
use std::time::Duration;

pub fn add_tiles_to_world_system (
    mut commands: Commands,
     selected_tile: Res<SelectedTile>, 
    input: Res<Input<KeyCode>>, 
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_query: Query<&Mouse>,
    mut query: Query<(&player::Player, &Translation, &player::Moving)>
) {    
    let tile_size = lab_world::settings::TILE_SIZE;

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
                interaction: lab_entities::world::Interaction { call: interaction },
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

pub fn builder_keyboard_system (
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
}