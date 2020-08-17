
use bevy::{prelude::*, math::Vec2, ecs::Bundle, prelude::Properties, render::camera::Camera, type_registry::TypeRegistry};
use crate::player;

pub mod stage {
    pub const WORLD: &'static str = "world";
}

pub mod settings {
    pub const TILE_SIZE : f32 = 96.;
}

/// Plugin that will setup all of the rules of the world.
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(npc_move.system())
            .add_system(make_room.system())
            .add_system(add_player_sprites.system())
            .add_system(save_world.thread_local_system())
            .add_system(collision_detection.system());
    }
}

use rand::{
    distributions::{Distribution, Standard},
    Rng,
}; 


const world_tile_size : f32 = 96.;

use crate::{Named, player::*, assets};
#[derive(Clone, Debug, Copy, PartialEq, Properties, Default)]
pub struct Location (pub f32, pub f32, pub f32);

impl Location {
    pub fn from_translation(translation : Translation) -> Location {
        Location(translation.x(), translation.y(), translation.z())
    }
}

#[derive(Clone, PartialEq)]
pub struct Area(pub f32, pub f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Wall(Hardness),
    Floor,
    Lava,
    Bar,
    Grass,
    Key
}

#[derive(Copy, Clone, Debug)]
pub struct Visible;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Hardness (pub f32);

#[derive(Bundle, Copy, Clone, Debug)]
pub struct TileComponents {
    pub hardness: Hardness,
    pub tile_type: TileType,
    pub location: Location,
    pub visible: Visible
}

impl Default for TileComponents {
    fn default() -> Self {
        TileComponents {
            hardness: Hardness(0.),
            tile_type: TileType::Key,
            location: Location::default(),
            visible: Visible
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Moveable;

#[derive(Debug, Clone, Copy)]
pub struct Solid;


#[derive(Debug)]
pub struct InteractionResult {
    message: String,
    colliding_entity : Option<Entity>
}
pub struct MapBuilder {
    pub tile_size : Vec2,
    pub current_location : Location,
    pub tiles : Vec<TileComponents>
}

pub enum RelativePosition {
    LeftOf,
    RightOf,
    Above,
    Below
}

impl MapBuilder {
    pub fn new(tile_size : Vec2, starting_location: Location) -> MapBuilder {
        MapBuilder {
            tile_size : tile_size.clone(),
            current_location : starting_location,
            tiles: Vec::new()
        }
    }
    pub fn add_tiles_to_area(&mut self, loc : Location, area: Area, tile_type: TileType){
                 

        for x in 0..area.0 as u32 {
            for y in 0..area.1 as u32 {                
                self.tiles.push(TileComponents {
                   tile_type: tile_type, 
                   location: Location(loc.0 + (x as f32 * self.tile_size.x()), loc.1 - (y as f32 * self.tile_size.y()), loc.2),
                   hardness: Hardness(0.),
                   ..Default::default()
                });            
            }
        }
    }
    pub fn add_tiles(&mut self, pos : RelativePosition, count : u32, tile_type: TileType){
       

        for _ in 0..count {
            let mut loc = &self.current_location;
            let location = match pos {
                RelativePosition::LeftOf => {                                    
                    Location(loc.0 - self.tile_size.x(), loc.1, loc.2)
                }
                RelativePosition::RightOf => {
                    Location(loc.0 + self.tile_size.x(), loc.1, loc.2)
                }
                RelativePosition::Above => {
                    Location(loc.0, loc.1 + self.tile_size.y(), loc.2)
                }
                RelativePosition::Below => {
                    Location(loc.0, loc.1 - self.tile_size.y(), loc.2)
                }
            };

            let hardness = match tile_type {
                TileType::Wall(h ) =>  {
                    h
                }, 
                _ => Hardness(0.),
            };
            
            self.tiles.push(TileComponents {
                tile_type: tile_type, 
                location: location,
                hardness: hardness,
                ..Default::default()
             });

            self.current_location = location;
        }
    }

    pub fn iter(&mut self) -> std::slice::Iter<'_, TileComponents> {
        self.tiles.iter()
    }

}

fn add_player_sprites(mut commands: Commands,
    sprites : ResMut<assets::SpriteLibrary>,
    mut query: Query<(Entity, Added<Player>, &Named, &Location)>,
    mut npc_query: Query<(Entity, Added<NonPlayer>, &Named, &Location)>
) {
    for (e, player, name , loc) in &mut query.iter() {
        // new player was added, lets render them!
        let sprite = sprites.get("player");
        
        println!("got sprite {} for {} at {:?}", sprite.name, name.0, loc);

        let p = *player;
        commands
        .insert(e, SpriteSheetComponents {
            translation: Translation(Vec3::new(world_tile_size, world_tile_size, 30.)),
            scale: Scale(3.0),
            draw: Draw { is_visible: true, is_transparent: true, ..Default::default() },
            sprite: TextureAtlasSprite::new(sprite.atlas_sprite),
            texture_atlas: sprite.atlas_handle.clone(),
            ..Default::default()
        }).insert_one(e, Moving(*loc, *loc, player::Direction::Stationary));
    }
    for (e, nonplayer, name , loc) in &mut npc_query.iter() {
        // new player was added, lets render them!
        let sprite = sprites.get("npc");
        
        println!("got sprite {} for {} at {:?}", sprite.name, name.0, loc);
        let p = *nonplayer;
        commands
        .insert(e,SpriteSheetComponents {
            translation: Translation(Vec3::new(loc.0, loc.1, 30.)),
            scale: Scale(3.0),
            draw: Draw { is_visible: true, is_transparent: true, ..Default::default() },
            sprite: TextureAtlasSprite::new(sprite.atlas_sprite),
            texture_atlas: sprite.atlas_handle.clone(),
            ..Default::default()
        }).insert(e, (Moveable, Timer::from_seconds(1.5), Moving(*loc, *loc, player::Direction::Stationary)));
    }
}

// adds the sprites for the tiles
fn make_room (
    mut commands: Commands,
    sprites : ResMut<assets::SpriteLibrary>,   
    mut query: Query<(Entity, &TileType, &Visible, &Location, Without<Draw,(&Visible,)>)>,
    mut p_query: Query<(Entity, Added<Moveable>, &Visible, &Location)>,
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
    for (e, tile, &_vis, &loc, _w) in &mut query.iter() {
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

#[derive(Debug)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
    Unknown
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

/// Collision detection system
/// 
fn collision_detection(
    mut camera_query: Query<(&Camera, &mut Translation)>,
    mut wall_query: Query<(&Hardness, &mut Translation)>,
    mut moveables: Query<(&Moveable, &mut Translation)>,
    mut player_moved_query: Query<(&Player, &mut Translation, Mutated<Moving>)>,
    mut nonplayer_moved_query: Query<(&NonPlayer, &mut Translation, Mutated<Moving>)>,
) {

    for (p, mut move_transition, m) in &mut nonplayer_moved_query.iter() {
        println!("NPC Moved");
        for (hardness, mut tile_translation) in &mut wall_query.iter() {
            if hardness.0 == 0. {
                continue;
            }

            let collision = collide(move_transition.0, Vec2::new(world_tile_size,world_tile_size), tile_translation.0, Vec2::new(48.,48.0), false);
            
            if let Some(collision) = collision {
                match collision {
                    _ => { 
                        println!("Collision!");
                        *move_transition.0.x_mut() = (m.0).0;
                        *move_transition.0.y_mut() = (m.0).1;
                    }
                }
            } 
        }
    }
    for (p, mut move_transition, m) in &mut player_moved_query.iter() {
        for (push, mut push_translation) in &mut moveables.iter() {             
            let collision = collide(move_transition.0, Vec2::new(48.,48.), push_translation.0, Vec2::new(32.,32.0), false);
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

            let collision = collide(move_transition.0, Vec2::new(world_tile_size,world_tile_size), tile_translation.0, Vec2::new(48.,48.0), false);
            
            if let Some(collision) = collision {
                match collision {
                    _ => { 
                        *move_transition.0.x_mut() = (m.0).0;
                        *move_transition.0.y_mut() = (m.0).1;
                    }
                }
            } else {     
                // move the camera if the player moves.
                for (_c, mut cam_trans) in &mut camera_query.iter(){  
                    *cam_trans.0.x_mut() = move_transition.0.x();             
                    *cam_trans.0.y_mut() = move_transition.0.y();
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


impl Distribution<player::Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> player::Direction {
        match rng.gen_range(0, 5) {
            0 => player::Direction::Up,
            1 => player::Direction::Down,
            2 => player::Direction::Left,
            3 => player::Direction::Right,
            _ => player::Direction::Stationary
        }
    }
}

fn npc_move(mut query: Query<(&NonPlayer, &mut Timer, &mut Translation, &mut Moving, &mut Moveable)>) {
    for (np, mut timer, mut trans, mut m, _mm) in &mut query.iter() {
        if  timer.finished {
            let old_loc = Location::from_translation(*trans);
            let direction = rand::random::<player::Direction>();

            match (direction)  {
                player::Direction::Left =>  {
                    *trans.0.x_mut() -= world_tile_size;
                },
                player::Direction::Up => *trans.0.y_mut() += world_tile_size,
                player::Direction::Down =>  *trans.0.y_mut() -= world_tile_size,
                player::Direction::Right => *trans.0.x_mut() += world_tile_size,
                player::Direction::Stationary =>  {}
            }

            *m = Moving(old_loc, Location::from_translation(*trans), direction);

            timer.reset();
        }
    }
}