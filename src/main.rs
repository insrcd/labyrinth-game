
use bevy::{
    prelude::*,
    render::{camera::Camera, pass::ClearColor},
    input::{keyboard::KeyCode, Input}, type_registry::TypeRegistry,
};


use serde::{Deserialize, Serialize};

mod player;
mod world;
mod assets;
mod scripting;


use assets::*;
use player::*;
use world::*;
use scripting::*;

enum GameState {
    Init,
    MainMenu,
    CharacterScreen,
    Map
}

#[derive(Debug)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
    Unknown
}

#[derive(Properties, Serialize)]
struct Named(pub String);

impl Named {
    fn new(name : &str) -> Named {
        Named(name.to_string())
    }
}

impl Default for Named {
    fn default() -> Self {
        Named ("No Name".to_string())
    }
}

// resource for current location
pub fn collide(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2, d: bool) -> Option<Collision> {
    let a_min = a_pos.truncate() - a_size / 2.0;
    let a_max = a_pos.truncate() + a_size / 2.0;

    let b_min = b_pos.truncate() - b_size / 2.0;
    let b_max = b_pos.truncate() + b_size / 2.0;

    if (d){
        println!("a: {} {} b: {} {}", a_min, a_max,b_min,b_max);
    }
    // check to see if the two rectangles are intersecting
    if a_min.x() <= b_max.x()
        && a_max.x() >= b_min.x()
        && a_min.y() <= b_max.y()
        && a_max.y() >= b_min.y()
    {
        println!("Intersecting");
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
            (None, None) => Some(Collision::Unknown),
        }
    } else {
        None
    }
}

fn main() {
    App::build()
    .add_default_plugins()
    .register_component::<Named>()
    .register_component::<Player>()
    .add_startup_system(setup.system())
    .add_startup_system(load_world_sprites.system())
    .add_startup_system(simple_map.system())
    .add_system(keyboard_input_system.system())
    .add_system(make_room.system())
    .add_system(add_player.system())
    .add_system(save_world.thread_local_system())
    .add_system(collision_detection.system())
    
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
    for (player, name , loc) in &mut query.iter() {
        // new player was added, lets render them!
        let sprite = sprites.get("player");
        
        println!("got sprite {} for {} at {:?}", sprite.name, name.0, loc);
        let p = *player;
        commands
        .spawn(SpriteSheetComponents {
            translation: Translation(Vec3::new(96., 96., 30.)),
            scale: Scale(3.0),
            draw: Draw { is_visible: true, is_transparent: true, ..Default::default() },
            sprite: TextureAtlasSprite::new(sprite.atlas_sprite),
            texture_atlas: sprite.atlas_handle.clone(),
            ..Default::default()
        }).with(p).with(Moving(*loc, *loc, player::Direction::Stationary));
    }
}
// adds the sprites for the tiles
fn make_room (
    mut commands: Commands,
    sprites : ResMut<assets::SpriteLibrary>,   
    mut query: Query<(Entity, Added<TileType>, &Visible, &Location)>,
    mut p_query: Query<(Entity, Added<Pushable>, &Visible, &Location)>,
) {
    for (e, _push, vis, &loc) in &mut p_query.iter() {
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
    for (e, tile, _vis, loc) in &mut query.iter() {
        println!("Adding a tile entity {:?} {:?} {:?}", *tile, loc,e);    

        let sprite = match *tile {
            TileType::Wall(_) => sprites.get("wall"),
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

    let mut mb = MapBuilder::new(Vec2::new(96.,96.), Location(0.,0.,0.));

    mb.add_tiles(RelativePosition::RightOf, 5, TileType::Wall(Hardness(1.)));
    mb.add_tiles(RelativePosition::Below, 2, TileType::Wall(Hardness(1.)));
    mb.add_tiles(RelativePosition::Below, 1, TileType::Floor);
    mb.add_tiles(RelativePosition::Below, 2, TileType::Wall(Hardness(1.)));
    mb.add_tiles(RelativePosition::LeftOf, 5, TileType::Wall(Hardness(1.)));
    mb.add_tiles(RelativePosition::Above, 5, TileType::Wall(Hardness(1.)));

    mb.add_tiles_to_area(Location(0.,0.,0.), Area(5., 5.), TileType::Floor);

    for comp in mb.iter() {
        println!("{:?}", comp);
        commands.spawn(comp.clone());
    }

    commands.spawn((Pushable, Location(96.*2.,96.*2.,2.), Visible));
}

fn collision_detection(
    mut camera_query: Query<(&Camera, &mut Translation)>,
    mut wall_query: Query<(&Hardness, &mut Translation)>,
    mut pushable: Query<(&Pushable, &mut Translation)>,
    mut player_moved_query: Query<(&Player, &mut Translation, Mutated<Moving>)>
) {


    for (_p, mut player_translation, m) in &mut player_moved_query.iter() {
        for (push, mut push_translation) in &mut pushable.iter() {             
            let collision = collide(player_translation.0, Vec2::new(48.,48.), push_translation.0, Vec2::new(32.,32.0), false);
            if let Some(collision) = collision {
                println!("Collision pushed {:?} {:?}", collision, *m);
                match collision {
                    Collision::Left => *push_translation.0.x_mut() = (m.1).0 + 48., 
                    Collision::Right =>*push_translation.0.x_mut() = (m.1).0 - 48.,
                    Collision::Top =>*push_translation.0.y_mut() = (m.1).1 - 48.,
                    Collision::Bottom => *push_translation.0.y_mut() = (m.1).1 + 48.,
                    // the collision in bevy didn't accounts for squares that interact exactly
                    Collision::Unknown => {
                        match m.2 {
                            player::Direction::Right => *push_translation.0.x_mut() = (m.1).0 + 48.,
                            player::Direction::Left => *push_translation.0.x_mut() = (m.1).0 - 48.,
                            player::Direction::Down => *push_translation.0.y_mut() = (m.1).1 - 48.,
                            player::Direction::Up => *push_translation.0.y_mut() = (m.1).1 + 48.,
                            player::Direction::Stationary => {}
                        }
                    }
                }
            } 
        }
        for (hardness, mut tile_translation) in &mut wall_query.iter() {
            //println!("{:?} {:?}", hardness, tile_translation.0);
            if hardness.0 == 0. {
                continue;
            }

            //println!("{} {}",player_translation.0, tile_translation.0);

            let collision = collide(player_translation.0, Vec2::new(96.,96.), tile_translation.0, Vec2::new(48.,48.0), false);
            
            if let Some(collision) = collision {
                match collision {
                    _ => { 
                        *player_translation.0.x_mut() = (m.0).0;
                        *player_translation.0.y_mut() = (m.0).1;
                    }
                }
            } else {     
                // move the camera if the player moves.
                for (_c, mut cam_trans) in &mut camera_query.iter(){  
                    *cam_trans.0.x_mut() = player_translation.0.x();             
                    *cam_trans.0.y_mut() = player_translation.0.y();
                }
            }
        }
    }
}

fn save_world(world: &mut World, resources: &mut Resources) {
    let type_registry = resources.get::<TypeRegistry>().unwrap();
    let input = resources.get::<Input<KeyCode>>().unwrap();
    let scene = Scene::from_world(&world, &type_registry.component.read().unwrap());
    
    use std::fs;

    // Scenes can be serialized like this:
    if input.just_pressed(KeyCode::F1) {
        let scene_ron = scene
        .serialize_ron(&type_registry.property.read().unwrap())
        .unwrap();
        fs::write("scenes/saved.scn", scene_ron).expect("Unable to write file");
    }
}

fn keyboard_input_system(
    mut commands : Commands,
    keyboard_input: Res<Input<KeyCode>>, 
    mut query: Query<(&Player, &mut Translation, &mut Moving)>) {

    let player_speed = 48.;

    let mut movement = player::Direction::Stationary;

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
            *moving = Moving(old_loc, Location::from_translation(*loc), movement);
        }
    }      
}

fn setup (
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    Player::add_to_world(commands, "Adam");
}