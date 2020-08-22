
use bevy::{ prelude::*, type_registry::TypeRegistry, render::camera::Camera};
use lab_entities::prelude::*;

use lab_entities::{
    world::{Interaction, InteractionResult, Location },
    player:: { Direction as Dir },
    Named
};

use lab_sprites::*;

use crate::settings::*;


struct TileLoader;

impl TileLoader  {
    pub fn sprite_for_tiletype<'a>(tile_type: &TileType, sprites: &'a SpriteLibrary) -> &'a lab_sprites::Sprite {
        match tile_type {
            TileType::Wall(_) => sprites.get("wall").clone().unwrap(),
            TileType::Brick(_) => sprites.get("brick").unwrap(),
            TileType::BrickDoorOpen => sprites.get("brick_door_open").unwrap(),
            TileType::BrickDoorClosed(_) => sprites.get("brick_door_closed").unwrap(),
            TileType::BrickWindow(_) => sprites.get("brick_window").unwrap(),
            TileType::BrickWindowBroken => sprites.get("brick_window_broken").unwrap(),
            TileType::Floor => sprites.get("floor").unwrap(),
            TileType::Lava => sprites.get("floor").unwrap(),
            TileType::Bar => sprites.get("floor").unwrap(),
            TileType::Grass => sprites.get("floor").unwrap(),
            TileType::Chair => sprites.get("chair").unwrap(),
            TileType::Shelf => sprites.get("shelf").unwrap(),
            TileType::Bed => sprites.get("bed").unwrap(),
            TileType::Table => sprites.get("table").unwrap(),
            TileType::Fridge => sprites.get("fridge").unwrap(),
            TileType::Key => sprites.get("floor").unwrap(),
            TileType::Mug => sprites.get("mug").unwrap(),
        }
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

pub fn add_interaction_sprites_system(mut commands: Commands,
    sprites : ResMut<SpriteLibrary>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(Entity, Added<Player>, &Named, &Location, &lab_sprites::MoveAnimation)>,
    mut npc_query: Query<(Entity, Added<NonPlayer>, &Named, &Location, &lab_sprites::Sprite)>
) {
    for (e, _player, name , loc, animate) in &mut query.iter() {
        // new player was added, lets render them!
        if let Some(sprite) = sprites.get("player"){
            println!("got sprite {} for {} at {:?}", sprite.name, name.0, loc);

            commands
                .insert(e, animate.down[0].to_components((*loc).into(), Scale(6.)))
                .insert_one(e, Movement(*loc, *loc, Dir::Stationary));
        }
    }
    
    for (e, _npc, name , loc, s) in &mut npc_query.iter() {
        // new player was added, lets render them!
        if let Some(sprite) = sprites.get("npc"){
            println!("got sprite {} for {} at {:?}", sprite.name, name.0, loc);
            //add_sprite(&asset_server, &mut materials, "resources/sprites/hat-guy.png", loc)
            commands
                .insert(e, s.to_components((*loc).into(), Scale(3.)))
                .insert(e, (Moveable, MoveTimer(Timer::from_seconds(1.5)), Movement(*loc, *loc, Dir::Stationary)));
            }
    }
}

// adds the sprites for the tiles
pub fn add_world_sprites_system (
    mut commands: Commands,
    sprites : ResMut<SpriteLibrary>,   
    mut query: Query<(Entity, &TileType, &Visible, &Location, Without<Draw,(&Visible,)>)>,
) {
    for (e, tile, &_vis, &loc, _w) in &mut query.iter() {
        println!("Adding a tile entity {:?} {:?} {:?}", *tile, loc,e);    

        let sprite = TileLoader::sprite_for_tiletype(&tile, &sprites);

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
pub fn object_interaction_system (
    mut commands: Commands,
    sprites : ResMut<SpriteLibrary>,   
    mut camera_query: Query<(&Camera, &mut Translation)>,
    mut placeable: Query<(Entity, &Interactable, &mut Translation, &Interaction)>,
    mut moveables: Query<Without<Player,(&Moveable, &mut Translation, Mutated<Movement>)>>,
    mut player_moved: Query<With<Player,(Entity, &mut Translation, Mutated<Movement>)>>
) {
}
/// Collision detection system
/// 
pub fn tile_interaction_system (
    mut commands: Commands,
    sprites : ResMut<SpriteLibrary>,   
    mut camera_query: Query<(&Camera, &mut Translation)>,
    mut wall_query: Query<(Entity, &mut TileType, &Hardness, &mut Translation, &Interaction)>,
    mut moveables: Query<Without<Player,(&Moveable, &mut Translation, Mutated<Movement>)>>,
    mut player_moved: Query<With<Player,(Entity, &mut Translation, Mutated<Movement>, &Inventory)>>
) {
    let mut player_collision: Option<Translation> = None;
    
    // tile based collision
    for (tile_entity, _tt, hardness, tile_translation, i) in &mut wall_query.iter() {
        if hardness.0 == 0. {
            continue;
        }

        for ( _m, mut move_transition, movement) in &mut moveables.iter() {
            let collision = collide(move_transition.0, Vec2::new(WORLD_TILE_SIZE,WORLD_TILE_SIZE), tile_translation.0, Vec2::new(48.,48.0), false);
            
            if let Some(collision) = collision {
                match collision {
                    _ => { 
                        if let InteractionResult::ChangeTile(tile_type) = (i.call)(Attributes {
                            interaction_location: Some(Location::from(*tile_translation)),
                            inventory: None,
                            player: None,
                            player_location: Some(movement.0.into())
                        }) {      
                            println!("Got change tile for NPC");                          
                            let sprite = TileLoader::sprite_for_tiletype(&tile_type, &sprites);
                            
                            commands.insert(tile_entity, TileComponents {
                                tile_type:tile_type, 
                                location: Location::from(*tile_translation),
                                hardness: Hardness(0.),
                                ..Default::default()
                            });

                            commands.insert_one(tile_entity, TextureAtlasSprite::new(sprite.atlas_sprite));
                        }
                    
                        *move_transition.0.x_mut() = (movement.0).0;
                        *move_transition.0.y_mut() = (movement.0).1;
                    }
                }
            } else {
            //    player_collision = Some(move_transition.clone());
            }
        } 
        for (e, mut move_translation, movement, inventory) in &mut player_moved.iter() {
            
            let collision = collide(move_translation.0, 
                Vec2::new(WORLD_TILE_SIZE,WORLD_TILE_SIZE),  tile_translation.0, Vec2::new(48.,48.0), false);
            
            if let Some(collision) = collision {
                match collision {
                    _ => { 
                        // run the lambda that tells us what to do if a collision happens with a tile
                        // if the transition says to change, then change.
                        if let InteractionResult::ChangeTile(tile_type) = (i.call)(Attributes {
                            interaction_location: Some(Location::from(*tile_translation)),
                            inventory: Some(inventory.clone()),
                            player: Some(e),
                            player_location: Some(movement.0.into())
                        }) {      
                            println!("Got change tile");                          
                            let sprite = TileLoader::sprite_for_tiletype(&tile_type, &sprites);
                            
                            commands.insert(tile_entity, TileComponents {
                                tile_type:tile_type, 
                                location: Location::from(*tile_translation),
                                hardness: Hardness(0.),
                                ..Default::default()
                            });

                            commands.insert_one(tile_entity, TextureAtlasSprite::new(sprite.atlas_sprite));
                        }
                        
                        // reset the sprite back to where it moved from

                        *move_translation.0.x_mut() = (movement.0).0;
                        *move_translation.0.y_mut() = (movement.0).1;
                    }
                }
            } else {     
                
                for (_c, mut cam_trans) in &mut camera_query.iter(){  
                    *cam_trans.0.x_mut() = move_translation.0.x();             
                    *cam_trans.0.y_mut() = move_translation.0.y();
                }
                
            }
        }
    }
    /*
    if let Some(movement) = player_collision {
        for (_c, mut cam_trans) in &mut camera_query.iter(){  
            *cam_trans.0.x_mut() = movement.0.x();             
            *cam_trans.0.y_mut() = movement.0.y();
        }
    }*/
    /*
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
                            Dir::Right => *push_translation.0.x_mut() = (m.1).0 + 48.,
                            Dir::Left => *push_translation.0.x_mut() = (m.1).0 - 48.,
                            Dir::Down => *push_translation.0.y_mut() = (m.1).1 - 48.,
                            Dir::Up => *push_translation.0.y_mut() = (m.1).1 + 48.,
                            Dir::Stationary => {}
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
                            let sprite = TileLoader::sprite_for_tiletype(&ret.1, &sprites);

                            commands.insert(e, TileComponents {
                                tile_type: ret.1, 
                                location: Location::from(*tile_translation),
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
    }*/
}

pub fn save_world_system(world: &mut World, resources: &mut Resources) {
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

pub struct MoveTimer (pub Timer);
pub struct DialogTimer (pub Timer);

/// Move all NPCs in the scene every 1.5 seconds
pub fn npc_move_system(time: Res<Time>, mut query: Query<(&NonPlayer, &mut MoveTimer, &mut Translation, &mut Movement, &mut Moveable)>) {
    for (_npc, mut timer, mut trans, mut m, _mm) in &mut query.iter() {
        timer.0.tick(time.delta_seconds);
        if  timer.0.finished {
            let old_loc = Location::from(*trans);
            let direction = rand::random::<Dir>();

            match direction {
                Dir::Left => *trans.0.x_mut() -= WORLD_TILE_SIZE,
                Dir::Up => *trans.0.y_mut() += WORLD_TILE_SIZE,
                Dir::Down =>  *trans.0.y_mut() -= WORLD_TILE_SIZE,
                Dir::Right => *trans.0.x_mut() += WORLD_TILE_SIZE,
                Dir::Stationary =>  {}
            }

            *m = Movement(old_loc, Location::from(*trans), direction);

            timer.0.reset();
        }
    }
}