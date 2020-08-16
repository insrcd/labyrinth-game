mod player;
mod world;
mod assets;
mod scripting;

use bevy::{
    prelude::*,
    render::{camera::Camera, pass::ClearColor},
    sprite::collide_aabb::{collide, Collision},
    input::{keyboard::KeyCode, Input}, type_registry::TypeRegistry, math,
};

use assets::*;
use player::*;
use world::*;
use scripting::*;

use world::Tile;
enum GameState {
    Init,
    MainMenu,
    CharacterScreen,
    Map
}

// resource for current location


fn main() {
    App::build()
    .add_default_plugins()
    .add_startup_system(setup.system())
    .add_startup_system(load_world_sprites.system())
    .add_startup_system(simple_map.system())
    .add_system(keyboard_input_system.system())
    .add_system(collision_detection.system())
    .add_system(make_room.system())
    .add_system(add_player.system())
    .add_system(add_tiles.system())
    .add_startup_system(save_world.thread_local_system())
    
    //.add_system(test.system())
    .run();
}

fn test (
    player: &Player,
    name: &Named
) {
    println!("{} {}", player, name.0)
}



fn add_player(mut commands: Commands,
    sprites : ResMut<assets::SpriteLibrary>,
    mut query: Query<(Added<Player>, &Named, &world::Location)>
) {
    for (_player, name , loc) in &mut query.iter() {
        // new player was added, lets render them!
        let sprite = sprites.get("player");
        
        println!("got sprite {} for {} at {:?}", sprite.name, name.0, loc);
        
        commands
        .spawn(SpriteSheetComponents {
            translation: Translation(Vec3::new(-300., loc.1, 100.)),
            scale: Scale(4.0),
            draw: Draw { is_visible: true, is_transparent: true, ..Default::default() },
            sprite: TextureAtlasSprite::new(sprite.atlas_sprite),
            texture_atlas: sprite.atlas_handle.clone(),
            ..Default::default()
        }).with(Player);
    }
}
// adds the sprites for the tiles
fn make_room (
    mut commands: Commands,
    sprites : ResMut<assets::SpriteLibrary>,
    texture_atlases: Res<Assets<TextureAtlas>>,    
    mut query: Query<(Entity, &Tile, &Visible, &Location, Without<Draw, (Entity,)>)>,
    mut p_query: Query<(Entity, Added<Pushable>, &Visible, &Location)>,
) {
    for (e, push, vis, &loc) in &mut p_query.iter() {
        let sprite = sprites.get("chair");
        
        commands.insert(e, SpriteSheetComponents {
            translation: Translation(Vec3::new(loc.0, loc.1, loc.2)),
            scale: Scale(6.0),
            draw: Draw { is_visible: true, is_transparent: true, ..Default::default() },
            sprite: TextureAtlasSprite::new(sprite.atlas_sprite),
            texture_atlas: sprite.atlas_handle.clone(),
            ..Default::default()
        });
    }
    for (e, tile, vis, loc, _w) in &mut query.iter() {
        println!("Adding a tile entity {:?} {:?}", tile.0, loc);    

        let sprite = match tile.0 {
            TileType::Wall => sprites.get("wall"),
            _ => sprites.get("floor"),
        };

        commands.insert(e, SpriteSheetComponents {
            translation: Translation(Vec3::new(loc.0, loc.1, loc.2)),
            scale: Scale(6.0),
            draw: Draw { is_visible: true, ..Default::default() },
            sprite: TextureAtlasSprite::new(sprite.atlas_sprite),
            texture_atlas: sprite.atlas_handle.clone(),
            ..Default::default()
        });
    
    }
}
// generate a simple map

fn simple_map(mut commands: Commands) {

    let mut mb = MapBuilder::new(Vec2::new(96.,96.), Location(-450.,300.,0.));

    mb.add_tiles(RelativePosition::RightOf, 5, TileType::Wall);
    /*mb.add_tiles(RelativePosition::Below, 2, TileType::Wall);
    mb.add_tiles(RelativePosition::Below, 1, TileType::Floor);
    mb.add_tiles(RelativePosition::Below, 2, TileType::Wall);
    mb.add_tiles(RelativePosition::LeftOf, 5, TileType::Wall);
    mb.add_tiles(RelativePosition::Above, 5, TileType::Wall);*/

    mb.add_tiles_to_area(Location(-450.,300.,0.), Area(5., 5.), TileType::Floor);

    for (tile, location) in mb.iter() {
        commands.spawn((Visible, tile.clone(), location.clone()));
    }

    commands.spawn((Pushable, Location(-450. + 96.*2.,300. - 96.*2.,50.), Visible));
}
fn collidea(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> Option<Collision> {
    let a_min = a_pos.truncate() - a_size / 2.0;
    let a_max = a_pos.truncate() + a_size / 2.0;

    let b_min = b_pos.truncate() - b_size / 2.0;
    let b_max = b_pos.truncate() + b_size / 2.0;

    println!("{},{} - {},{}",a_min, a_max, b_min,b_max);

    // check to see if the two rectangles are intersecting
    if a_min.x() < b_max.x()
        && a_max.x() > b_min.x()
        && a_min.y() < b_max.y()
        && a_max.y() > b_min.y()
    {
        // check to see if we hit on the left or right side
        let (x_collision, x_depth) =
            if a_min.x() < b_min.x() && a_max.x() > b_min.x() && a_max.x() < b_max.x() {
                (Some(Collision::Left), b_min.x() - a_max.x())
            } else if a_min.x() > b_min.x() && a_min.x() < b_max.x() && a_max.x() > b_max.x() {
                (Some(Collision::Right), a_min.x() - b_max.x())
            } else {
                (None, 0.0)
            };

        // check to see if we hit on the top or bottom side
        let (y_collision, y_depth) =
            if a_min.y() < b_min.y() && a_max.y() > b_min.y() && a_max.y() < b_max.y() {
                (Some(Collision::Bottom), b_min.y() - a_max.y())
            } else if a_min.y() > b_min.y() && a_min.y() < b_max.y() && a_max.y() > b_max.y() {
                (Some(Collision::Top), a_min.y() - b_max.y())
            } else {
                (None, 0.0)
            };

        // if we had an "x" and a "y" collision, pick the "primary" side using penetration depth
        match (x_collision, y_collision) {
            (Some(x_collision), Some(y_collision)) => {
                if y_depth < x_depth {
                    Some(y_collision)
                } else {
                    Some(x_collision)
                }
            }
            (Some(x_collision), None) => Some(x_collision),
            (None, Some(y_collision)) => Some(y_collision),
            (None, None) => None,
        }
    } else {
        None
    }
}
fn collision_detection(
    mut camera_query: Query<(&Camera, &mut Translation)>,
    mut wall_query: Query<(&Tile, &mut Translation)>,
    mut pushable: Query<(&Pushable, &mut Translation)>,
    mut sprite_query: Query<(&Player, &mut Translation)>,
    mut mquery: Query<(&Player, Mutated<Translation>)>
) {
    let mut mutated = false;

    for (_player, mut _loc) in &mut mquery.iter(){
        mutated = true;
    }

    if mutated == false {
        return;
    }    

    for (cam, mut cam_trans) in &mut camera_query.iter(){
        for (player, mut player_translation) in &mut sprite_query.iter() {
            for (push, mut push_translation) in &mut pushable.iter() {             
                let collision = collide(player_translation.0, Vec2::new(32.,32.), push_translation.0, Vec2::new(96.,96.0));
                if let Some(collision) = collision {
                    match collision {
                        Collision::Left => { 
                            *push_translation.0.x_mut() += 32.;
                        }, 
                        Collision::Right => { 
                            *push_translation.0.x_mut() -= 32.;  
                        },
                        Collision::Top => { 
                            *push_translation.0.y_mut() -= 32.; 
                        },
                        Collision::Bottom => {
                            *push_translation.0.y_mut() += 32.; 
                        },
                    }
                } 
            }
            for (tile, mut tile_translation) in &mut wall_query.iter() {
                
                if tile.0 != TileType::Wall {
                    continue;
                }
                //println!("Checking for collision with tile at {:?} {:?}",player_translation.0, tile_translation.0);
                let collision = collidea(Vec3::new(player_translation.0.x() + 16., player_translation.0.y() + 16., 0.), Vec2::new(32.,32.),
                Vec3::new(tile_translation.0.x() + 48., tile_translation.0.y() + 48., 0.), Vec2::new(96.,96.0));
                if let Some(collision) = collision {
                    match collision {
                        Collision::Left => { 
                            println!("Collided with left of tile {:?} Player: ({:?})",tile_translation.0, player_translation.0);
                            *player_translation.0.x_mut() -= 32.;
                        }, 
                        Collision::Right => { 
                            println!("Collided with right of tile {:?} Player: ({:?})",tile_translation.0,player_translation.0);
                            *player_translation.0.x_mut() += 32.;  
                        },                
                        Collision::Top => { 
                            println!("Collided with top of tile {:?} Player: ({:?})",tile_translation.0,player_translation.0);
                            *player_translation.0.y_mut() += 32.; 
                        },
                        Collision::Bottom => {
                            println!("Collided with bottom of tile {:?} Player: ({:?})",tile_translation.0,player_translation.0);
                            *player_translation.0.y_mut() -= 32.; 
                        },
                    }
                } else {       
                    *cam_trans.0.x_mut() = player_translation.0.x();             
                    *cam_trans.0.y_mut() = player_translation.0.y();
                }
            }
        }
    }

    for (push, mut push_translation) in &mut pushable.iter() {         
        for (tile, mut tile_translation) in &mut wall_query.iter() {
                
            if tile.0 != TileType::Wall {
                continue;
            }  

            let collision = collide(tile_translation.0, Vec2::new(96.,96.), push_translation.0, Vec2::new(96.,96.0));
            if let Some(collision) = collision {
                match collision {
                    Collision::Left => { 
                        *push_translation.0.x_mut() += 32.;
                    }, 
                    Collision::Right => { 
                        *push_translation.0.x_mut() -= 32.;  
                    },
                    Collision::Top => { 
                        *push_translation.0.y_mut() -= 32.; 
                    },
                    Collision::Bottom => {
                        *push_translation.0.y_mut() += 32.; 
                    },
                }
            } 
        }
    }
}

fn save_world(world: &mut World, resources: &mut Resources) {
    let type_registry = resources.get::<TypeRegistry>().unwrap();
    let input = resources.get::<Input<KeyCode>>().unwrap();
    let scene = Scene::from_world(&world, &type_registry.component.read().unwrap());

    // Scenes can be serialized like this:
    if input.just_pressed(KeyCode::F1) {
        println!(
            "{}",
            scene
                .serialize_ron(&type_registry.property.read().unwrap())
                .unwrap()
        );
    }
}

fn keyboard_input_system(
    type_registry: Res<TypeRegistry>,
    keyboard_input: Res<Input<KeyCode>>, 
    mut camera_query: Query<(&Camera, &mut Translation)>,
    mut query: Query<(&Player, &mut Translation)>) {

    let move_speed = 32.;

    for (player, mut loc) in &mut query.iter() {   
        if keyboard_input.just_pressed(KeyCode::W) {
            *loc.0.y_mut() += move_speed;
            //*cam_trans.0.y_mut() += 32.;
        }

        if keyboard_input.just_pressed(KeyCode::S) {
            *loc.0.y_mut() -= move_speed;
            //*cam_trans.0.y_mut() -= 32.;
        }

        if keyboard_input.just_pressed(KeyCode::A) {
            *loc.0.x_mut() -= move_speed;
            //*cam_trans.0.x_mut() -= 32.;
        }
        if keyboard_input.just_pressed(KeyCode::D) {
            *loc.0.x_mut() += move_speed;
            //*cam_trans.0.x_mut() += 32.;
        }
    }      

    for (camera, mut cam_trans) in &mut camera_query.iter() {    
        if keyboard_input.just_pressed(KeyCode::Up) {
            //*loc.0.y_mut() += 32.;
            *cam_trans.0.y_mut() += move_speed;
        }

        if keyboard_input.just_pressed(KeyCode::Down) {
            //*loc.0.y_mut() -= 32.;
            *cam_trans.0.y_mut() -= move_speed;    
        }

        if keyboard_input.just_pressed(KeyCode::Left) {
            //*loc.0.x_mut() -= 32.;
            *cam_trans.0.x_mut() -= move_speed;
        }
        if keyboard_input.just_pressed(KeyCode::Right) {
            //*loc.0.x_mut() += 32.;
            *cam_trans.0.x_mut() += move_speed;
        }
    }

}

fn setup (
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default());
    Player::add_to_world(commands, "Adam");
}

fn add_tiles (
    mut commands: Commands,
    input: Res<Input<KeyCode>>, 
    mut query: Query<(&Player, &Translation)>
) {
    for (p, t) in &mut query.iter(){
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

            let loc =  Location(x, y, 1.);
            
            println!("Adding wall to {:?}", loc);
            
            commands.spawn((Visible, Tile(TileType::Wall), loc));
        }
    }
}