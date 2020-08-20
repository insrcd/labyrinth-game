

use strum_macros::EnumIter;

#[allow(dead_code)]
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
            .add_system(npc_move_system.system())
            .add_system(add_world_sprites.system())
            .add_system(add_interaction_sprites_system.system())
            .add_system(save_world_system.thread_local_system())
            .add_system(collision_detection_system.system());
    }
}

use rand::{
    distributions::{Distribution, Standard},
    Rng,
}; 


const WORLD_TILE_SIZE : f32 = 96.;

use crate::{Named, player::*, assets};

use assets::SpriteLibrary;

impl TileType {
    fn sprite_for_tiletype(&self, sprites: &SpriteLibrary) -> crate::assets::Sprite {
        match self {
            TileType::Wall(_) => sprites.get("wall"),
            TileType::Brick(_) => sprites.get("brick"),
            TileType::BrickDoorOpen => sprites.get("brick_door_open"),
            TileType::BrickDoorClosed(_) => sprites.get("brick_door_closed"),
            TileType::BrickWindow(_) => sprites.get("brick_window"),
            TileType::BrickWindowBroken => sprites.get("brick_window_broken"),
            TileType::Floor => sprites.get("floor"),
            TileType::Lava => sprites.get("floor"),
            TileType::Bar => sprites.get("floor"),
            TileType::Grass => sprites.get("floor"),
            TileType::Chair => sprites.get("chair"),
            TileType::Shelf => sprites.get("shelf"),
            TileType::Bed => sprites.get("bed"),
            TileType::Table => sprites.get("table"),
            TileType::Fridge => sprites.get("fridge"),
            TileType::Key => sprites.get("floor")
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Interaction {
    pub call : fn (Attributes) -> (bool, TileType)
}

#[derive(Bundle, Copy, Clone, Debug)]
pub struct TileComponents {
    pub hardness: Hardness,
    pub tile_type: TileType,
    pub location: Location,
    pub visible: Visible,
    pub interaction: Interaction
}

impl TileComponents {
    fn hardness_from_tile(tile_type: TileType) -> Hardness {
        match tile_type {
            TileType::Wall(h ) => h, 
            TileType::Brick(h ) =>  h,
            TileType::BrickWindow(h ) =>  h,
            TileType::BrickDoorClosed(h ) => h, 
            _ => Hardness(0.),
        }
    }
}

impl Default for TileComponents {
    fn default() -> Self {
        TileComponents {
            hardness: Hardness(0.),
            tile_type: TileType::Key,
            location: Location::default(),
            visible: Visible,
            interaction: Interaction { call: |_attributes| { (false, TileType::Key) } }
        }
    }
}

pub struct Despawn;

pub struct Attributes; /* {
    settings: HashMap<String, u32>
}*/
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

impl<'a>  MapBuilder {
    pub fn new(tile_size : Vec2, starting_location: & Location) -> MapBuilder {
        MapBuilder {
            tile_size : tile_size.clone(),
            current_location : starting_location.to_owned(),
            tiles: Vec::new()
        }
    }
    pub fn add_tiles_to_area(&mut self, loc : &Location, area: Area, tile_type: TileType){
                 

        for x in 0..area.0 as u32 {
            for y in 0..area.1 as u32 {                
                self.tiles.push(TileComponents {
                   tile_type: tile_type, 
                   location: Location(loc.0 + (x as f32 * self.tile_size.x()), loc.1 - (y as f32 * self.tile_size.y()), loc.2),
                   hardness:TileComponents::hardness_from_tile(tile_type),
                   visible: Visible,
                   ..Default::default()
                });            
            }
        }
    }
    pub fn add_tiles(&mut self, pos : RelativePosition, count : u32, tile_type: TileType){
       

        for _ in 0..count {
            let loc = self.current_location;
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
            
            self.tiles.push(TileComponents {
                tile_type: tile_type, 
                location: location.clone(),
                hardness: TileComponents::hardness_from_tile(tile_type),
                ..Default::default()
             });

            self.current_location = location.to_owned();
        }
    }

    pub fn iter(&mut self) -> std::slice::Iter<'_, TileComponents> {
        self.tiles.iter()
    }

}


fn add_sprite(asset_server: &AssetServer, assets: &mut Assets<ColorMaterial>, filename: &str, loc: &Location) -> SpriteComponents {
       
    let npc_sprite = asset_server.load(&filename).unwrap();

    SpriteComponents {
        translation: Translation(Vec3::new(loc.0, loc.1, 30.)),
        scale: Scale(3.0),
        draw: Draw { is_visible: true, is_transparent: true, ..Default::default() },
        material: assets.add(npc_sprite.into()),
        ..Default::default()
    }
}

fn add_player_sprites(mut commands: Commands,
    sprites : ResMut<assets::SpriteLibrary>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(Entity, Added<Player>, &Named, &Location)>,
    mut npc_query: Query<(Entity, Added<NonPlayer>, &Named, &Location)>
) {
    for (e, _player, name , loc) in &mut query.iter() {
        // new player was added, lets render them!
        let sprite = sprites.get("player");
        
        println!("got sprite {} for {} at {:?}", sprite.name, name.0, loc);

        commands
            .insert(e, add_sprite(&asset_server, &mut materials, "resources/sprites/sensei.png", loc))
            .insert_one(e, Moving(*loc, *loc, player::Direction::Stationary));
    }
    for (e, _npc, name , loc) in &mut npc_query.iter() {
        // new player was added, lets render them!
        let sprite = sprites.get("npc");
        
        println!("got sprite {} for {} at {:?}", sprite.name, name.0, loc);
        commands
            .insert(e, add_sprite(&asset_server, &mut materials, "resources/sprites/hat-guy.png", loc))
            .insert(e, (Moveable, Timer::from_seconds(1.5), Moving(*loc, *loc, player::Direction::Stationary)));
    }
}

// adds the sprites for the tiles
fn make_room (
    mut commands: Commands,
    sprites : ResMut<assets::SpriteLibrary>,   
    mut query: Query<(Entity, &TileType, &Visible, &Location, Without<Draw,(&Visible,)>)>,
) {
    for (e, tile, &_vis, &loc, _w) in &mut query.iter() {
        println!("Adding a tile entity {:?} {:?} {:?}", *tile, loc,e);    

        let sprite = tile.sprite_for_tiletype(&sprites);

        commands.insert(e, SpriteSheetComponents {
            translation: Translation(Vec3::new(loc.0, loc.1, loc.2)),
            scale: Scale(6.0),
            draw: Draw { is_visible: true, is_transparent: true, ..Default::default() },
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

    if  d {
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
    mut commands: Commands,
    sprites : ResMut<assets::SpriteLibrary>,   
    mut camera_query: Query<(&Camera, &mut Translation)>,
    mut wall_query: Query<(Entity, &mut TileType, &Hardness, &mut Translation, &Interaction)>,
    mut moveables: Query<(&Moveable, &mut Translation)>,
    mut player_moved_query: Query<(&Player, &mut Translation, Mutated<Moving>)>,
    mut nonplayer_moved_query: Query<(&NonPlayer, &mut Translation, Mutated<Moving>)>,
) {

    for (_p, mut move_transition, m) in &mut nonplayer_moved_query.iter() {
        for (_e, _tt, hardness, tile_translation, _i) in &mut wall_query.iter() {
            if hardness.0 == 0. {
                continue;
            }

            let collision = collide(move_transition.0, Vec2::new(WORLD_TILE_SIZE,WORLD_TILE_SIZE), tile_translation.0, Vec2::new(48.,48.0), false);
            
            if let Some(collision) = collision {
                match collision {
                    _ => { 
                    
                        *move_transition.0.x_mut() = (m.0).0;
                        *move_transition.0.y_mut() = (m.0).1;
                    }
                }
            } 
        }
    }
    for (_p, mut move_transition, m) in &mut player_moved_query.iter() {
        for (_push, mut push_translation) in &mut moveables.iter() {             
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
        for (e,_tile_type, hardness, tile_translation, i) in &mut wall_query.iter() {
            //println!("{:?} {:?}", hardness, tile_translation.0);
            if hardness.0 == 0. {
                continue;
            }

            //println!("{} {}",player_translation.0, tile_translation.0);

            let collision = collide(move_transition.0, 
                Vec2::new(WORLD_TILE_SIZE,WORLD_TILE_SIZE),  tile_translation.0, Vec2::new(48.,48.0), false);
            
            if let Some(collision) = collision {
                match collision {
                    _ => { 
                        // run the lambda that tells us what to do if a collision happens with a tile
                        let ret = (i.call)(Attributes);
                        
                        // if the transition says to change, then change.
                        if ret.0 == true {
                            let sprite = ret.1.sprite_for_tiletype(&sprites);

                            commands.insert(e, TileComponents {
                                tile_type: ret.1, 
                                location: Location::from_translation(*tile_translation),
                                hardness: Hardness(0.),
                                ..Default::default()
                             });

                             commands.insert_one(e, TextureAtlasSprite::new(sprite.atlas_sprite));
                            
                        }
                        
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

/// Move all NPCs in the scene every 1.5 seconds
fn npc_move(mut query: Query<(&NonPlayer, &mut Timer, &mut Translation, &mut Moving, &mut Moveable)>) {
    for (_npc, mut timer, mut trans, mut m, _mm) in &mut query.iter() {
        if  timer.finished {
            let old_loc = Location::from_translation(*trans);
            let direction = rand::random::<player::Direction>();

            match direction {
                player::Direction::Left => *trans.0.x_mut() -= WORLD_TILE_SIZE,
                player::Direction::Up => *trans.0.y_mut() += WORLD_TILE_SIZE,
                player::Direction::Down =>  *trans.0.y_mut() -= WORLD_TILE_SIZE,
                player::Direction::Right => *trans.0.x_mut() += WORLD_TILE_SIZE,
                player::Direction::Stationary =>  {}
            }

            *m = Moving(old_loc, Location::from_translation(*trans), direction);

            timer.reset();
        }
    }
}