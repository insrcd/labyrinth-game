
use bevy::{ prelude::*, type_registry::TypeRegistry, render::camera::Camera};
use lab_entities::prelude::*;

use lab_entities::{
    world::{Interaction, InteractionResult, Location },
    player:: { Direction as Dir },
    Named
};

use lab_sprites::{ SpriteInfo, SpriteLibrary, TileAnimation, StationaryLetter };

use crate::{settings::*};
use lab_core::{Zoomable, Moveable};

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
    mut placeable: Query<(Entity, &Interactable, &mut Translation, &Interaction, &mut TileAnimation)>,
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
    mut wall_query: Query<(Entity, &mut TileType, &Hardness, &mut TileAttributes, &mut Translation, &Interaction, &SpriteInfo)>,
    mut moveables: Query<Without<Player,(&Moveable, &mut Translation, Mutated<Movement>, &SpriteInfo)>>,
    mut player_moved: Query<With<Player,(Entity, &Scale, &mut Translation, Mutated<Movement>, &Inventory, &SpriteInfo)>>
) {
    let mut player_collision: Option<Translation> = None;
    
    // tile based collision
    for (tile_entity, _tt, hardness, 
            tile_attributes, tile_translation, i,  
                tile_sprite) in &mut wall_query.iter() {
        if hardness.0 == 0. || tile_attributes.hit_points == 0 {
            continue;
        }

        for ( _m, mut move_transition, movement, move_sprite) in &mut moveables.iter() {
            println!("Checking for collision");

            let collision = collide(move_transition.0, 
                move_sprite.size(), 
                tile_translation.0, 
                tile_sprite.size(), 
                false);
            
            if let Some(collision) = collision {
                match collision {
                    _ => { 
                        if let InteractionResult::ChangeTile(attr) = (i.call)(Attributes {
                            interaction_location: Some(Location::from(*tile_translation)),
                            inventory: None,
                            player: None,
                            player_location: Some(movement.0.into()),
                            tile_attributes: Some(*tile_attributes)
                        }) {      
                            println!("Got change tile for NPC : {:?}", attr);                    
                            
                            commands.insert(tile_entity, (Location::from(*tile_translation), 
                                attr, TextureAtlasSprite::new(tile_sprite.atlas_sprite)));
                        }
                    
                        *move_transition.0.x_mut() = (movement.0).0;
                        *move_transition.0.y_mut() = (movement.0).1;
                    }
                }
            } else {
            //    player_collision = Some(move_transition.clone());
            }
        } 
        for (e, scale, mut move_translation, movement, inventory, sprite) in &mut player_moved.iter() {
            // I reduce the player bounding box by a few pixels to allow for closer interaction.
            // this can probably be a non-constant based on tile size.
            let collision = collide(move_translation.0, 
                (sprite.size() * scale.0) - Vec2::new(16.* scale.0,16.* scale.0),  tile_translation.0, tile_sprite.size() * scale.0, false);
            
            if let Some(collision) = collision {
                match collision {
                    _ => { 
                        // run the lambda that tells us what to do if a collision happens with a tile
                        // if the transition says to change, then change.
                        if let InteractionResult::ChangeTile(attr) = (i.call)(Attributes {
                            interaction_location: Some(Location::from(*tile_translation)),
                            inventory: Some(inventory.clone()),
                            player: Some(e),
                            player_location: Some(movement.0.into()),
                            tile_attributes: Some(*tile_attributes)
                        }) {      
                            println!("Got change tile: {:?}", attr); 

                            commands.insert(tile_entity, (Location::from(*tile_translation), 
                                attr, TextureAtlasSprite::new(sprite.atlas_sprite)));
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
   
}

pub fn save_world_system(world: &mut World, resources: &mut Resources) {
    /*let type_registry = resources.get::<TypeRegistry>();
    let input = resources.get::<Input<KeyCode>>();
    let scene = Scene::from_world(&world, &type_registry.component.read());
    
    use std::fs;

    // Scenes can be serialized like this:
    if input.just_pressed(KeyCode::F1) {
        let scene_ron = scene
        .serialize_ron(&type_registry.property.read())
        .unwrap();
        fs::write("scenes/saved.scn", scene_ron).expect("Unable to write file");
    }*/
}

pub struct MoveTimer (pub Timer);
pub struct DialogTimer (pub Timer);

/**
 * I tried using camera scale, but it doesn't seem to be able to zoom in when scale < 1
 */
pub fn zoom_system(
    mut scroll : ResMut<lab_input::ScrollState>,
    mut query : Query<(&mut Scale, &mut Translation, &Zoomable)>
) {
    for (mut scale, mut trans, _tt) in &mut query.iter(){
        if scroll.y != 0.{

            // ease in the zoom by about .25 of the scroll intensity
            let ease : f32 = 0.25;

            let factor = (scroll.y.clone() * ease)+ 1. ;

            println!("{}", factor);

            *scale = Scale( scale.0 * factor );

            *trans.x_mut() *= factor;
            *trans.y_mut() *= factor;

            scroll.current_scale = scale.0;
        } 
    }
}
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


pub fn sprite_despawn_system(
    mut commands: Commands,
    mut query : Query<(Entity, &Draw,&SpriteInfo, &lab_core::Despawn, &Timer, &mut Translation)>
){
    for (e, sprite,s, _dspawn, timer, mut translation) in &mut query.iter(){
        if timer.finished {
            //commands.remove_one::<Draw>(e);
            *translation.x_mut() -= 5000.;commands.remove_one::<Draw>(e);
        }
    }
}

pub fn static_text_system(
    mut commands: Commands,
    mut query : Query<(Entity, &StationaryLetter, &mut Translation)>,    
    mut player_query : Query<(Entity, &Player, Changed<Movement>)>
){
    for (e, _player, movement) in &mut player_query.iter(){

        for (e, _letter, mut translation) in &mut query.iter(){

            let old_loc = movement.0;
            let new_loc = movement.1;

            let x_change = old_loc.0 - new_loc.0;
            let y_change = old_loc.1 - new_loc.1;

            *translation.x_mut() -= x_change;
            *translation.y_mut() -= y_change;
        }
    }
}