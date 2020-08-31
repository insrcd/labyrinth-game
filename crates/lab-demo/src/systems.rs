use lab_world::*;
use lab_builder::prelude::*;
use lab_entities::prelude::*;
use crate::*;
use std::{cell::RefCell, rc::Rc, sync::Arc};
use lab_core::prelude::*;

// move to a resources file of some sort.
mod tiles {
    pub const WALL : &'static str = "wall";
    pub const FLOOR : &'static str = "floor";
    pub const BRICK_DOOR : &'static str = "brick_door_closed";
    pub const BRICK_DOOR_OPEN : &'static str = "brick_door_open";
    pub const BRICK : &'static str = "brick";
    pub const BRICK_WINDOW : &'static str = "brick_window";
    pub const BRICK_WINDOW_OPEN : &'static str = "brick_window_broken";    
    //pub const NPC : &'static str = "npc_0";   
    pub const ITEM : &'static str = "item_50";   
    pub const LOCKED_DOOR : &'static str = "locked_door";   
    pub const ENEMY : &'static str = "mob_19";
}

/// Adds a simple map using the map builder for the purposes of a demo.

pub fn create_simple_map_system(mut commands: Commands, mut palette: ResMut<TilePalette>) {

    // setup some basic interactions
    palette.interactions.insert("impassible".into(), Arc::new(TileInteraction { caller: |ctx| {            
        TileInteractionResult::Block.into()
    }, description:"Bump" }));

    palette.interactions.insert("open_door".into(), Arc::new(TileInteraction { caller: |ctx| {            
        let comps = ctx.world_catalog.components.get(tiles::BRICK_DOOR_OPEN).expect("Open brick door tile cannot be found");        
        
        TileInteractionResult::ChangeSprite(comps.sprite.clone()).into()

    }, description:"Open a door." }));

    if let Some(mut tiles) = palette.components.get_mut(tiles::WALL) {
        // walls are hard
        tiles.state.set_int("hardness".into(), 1);
        tiles.state.set_int("hit_points".into(), 800);       
    }
    
    if let Some(mut tiles) = palette.components.get_mut(tiles::BRICK) {
        // brick walls are beefier
        tiles.state.set_int("hardness".into(), 5);
        tiles.state.set_int("hit_points".into(), 1000);        
    }
    if let Some(mut tiles) = palette.components.get_mut(tiles::BRICK_DOOR) {
       
        tiles.state.set_int("hardness".into(), 1);
        tiles.state.set_int("hit_points".into(), 100);        
    }
    if let Some(mut tiles) = palette.components.get_mut(tiles::ENEMY) {
        // open doors
        palette.interactions.insert(tiles::ENEMY.to_string(), Arc::new(TileInteraction { caller: |ctx| {            
            if let InteractableType::Player = ctx.source.interactable_type {
                return vec![
                    TileInteractionResult::Message("Hello, you are my enemy. Lets fight.".into()).into(),
                    TileInteractionResult::Block.into()];
            }
            TileInteractionResult::Block.into()
        },
            description: "Enemy Interaction",}));
    }
    if let Some(tiles) = palette.components.get(tiles::BRICK_DOOR) {
        // open doors
        let mut new_tile = tiles.clone();
        new_tile.state.set_int("hardness".into(), 1);
        palette.components.insert("locked_door".into(), new_tile);
        palette.interactions.insert( "locked_door".into(),Arc::new(TileInteraction { caller: |ctx| {    
            let palette = ctx.world_catalog;

            // poor state tracking right now TODO Refactor and make safer
            //let open_sprite = palette.components.get(tiles::BRICK_DOOR_OPEN).unwrap().sprite;

            /*if let Some(inventory) = ctx.source.inventory 
            {

                if inventory.has(|i| i.item_type == ItemType::Key && i.id == 1){
                    return vec![
                            TileInteractionResult::ChangeSprite(open_sprite),
                            TileInteractionResult::Message("You have the key, Unlocked the door!".into())];
                }
            }*/
            vec![TileInteractionResult::Block, TileInteractionResult::Message("The door is locked, maybe there's a key somewhere".into())]
        },
            description: "Open Door",}));
    }
    /*
    if let Some(mut tiles) = palette.components.get_mut(tiles::BRICK_WINDOW) {

        tiles.interaction = TileInteraction { caller: |ctx| {  
            let open_sprite = Some(ctx.world_catalog.unwrap().components.get(tiles::BRICK_WINDOW_OPEN).unwrap().sprite.atlas_sprite);
                 
            // if a non-player hits a window, crash it if not block it 
            if let Some(source_type) = ctx.source_type {
                return match source_type {
                    InteractableType::Item |  InteractableType::Npc => TileInteractionResult::ChangeTile(TileAttributes { hardness: 0.0, sprite_idx: open_sprite, ..Default::default()}).into(),
                    _ => vec![TileInteractionResult::Block,TileInteractionResult::Message("The window looks breakable.".to_string())]
                };
            } else {
                vec![TileInteractionResult::Block,TileInteractionResult::Message("The window looks breakable.".to_string())]
            }
        },
            description: "Break Window",}
    }*/
    if let Some(mut tiles) = palette.components.get_mut(tiles::ITEM) {
        // break windows
        palette.interactions.insert(
            tiles::ITEM.into(), 
            Arc::new( TileInteraction { description: "Get Item", caller: |ctx| { 
                // demoooo   
                let item = ItemDefinition { 
                    id : 1,
                    name: "Key To Building 2".to_string(),
                    weight: Weight(0.1),
                    item_type: ItemType::Key,
                    item_slot: ItemSlot::LeftHand
                };
                
                let name = item.name.clone();
                let mut inventory = ctx.source.inventory.unwrap().clone();
                let mut storage = (*ctx.item_storage).clone();
                // do domestuff now
                inventory.items.push(storage.forge(item));

                if ctx.source.interactable_type == InteractableType::Player {
                    return vec![ 
                        TileInteractionResult::ChangeInventory(inventory),
                        TileInteractionResult::Despawn, 
                        TileInteractionResult::Message(format!("You picked up an item: {}", name).to_string())
                        ]
                };
                
                TileInteractionResult::None.into()
            }
        }));
    }
    
    let mut mb = MapBuilder::new(palette.clone(), &Location::default());

    &mut mb
            .add_tiles(RelativePosition::RightOf, 5, tiles::WALL.to_string())
            .add_tiles(RelativePosition::Below, 5, tiles::WALL.to_string())
            .add_tiles(RelativePosition::LeftOf, 1, tiles::WALL.to_string())
            .add_tiles(RelativePosition::LeftOf, 1, tiles::BRICK_DOOR.to_string())
            .add_tiles(RelativePosition::LeftOf, 1, tiles::WALL.to_string())
            .add_tiles(RelativePosition::LeftOf, 2, tiles::WALL.to_string())
            .add_tiles(RelativePosition::Above, 5, tiles::WALL.to_string())
            .add_tiles_to_area(&Location(0.,0.,0., WorldLocation::World), Vec2::new(6., 6.), tiles::FLOOR.to_string())
            .to_blueprint("basic_house");

    mb
    .add_tiles(RelativePosition::RightOf, 5, tiles::BRICK.to_string())
    .add_tiles(RelativePosition::Below, 5, tiles::BRICK.to_string())
    .add_tiles(RelativePosition::LeftOf, 2, tiles::BRICK_WINDOW.to_string())
    .add_tiles(RelativePosition::LeftOf, 1, tiles::LOCKED_DOOR.to_string())
    .add_tiles(RelativePosition::LeftOf, 1, tiles::BRICK_WINDOW.to_string())
    .add_tiles(RelativePosition::LeftOf, 1, tiles::BRICK.to_string())
    .add_tiles(RelativePosition::Above, 5, tiles::BRICK.to_string())
    .add_tiles_to_area(&Location(0.,0.,0., WorldLocation::World), Vec2::new(6., 6.), tiles::FLOOR.to_string())
    .to_blueprint("brick_house");

    mb
    .add_tiles_to_area(&Location::default(), Vec2::new(2., 6.), tiles::FLOOR.to_string())
    .to_blueprint("walkway");


    mb
        .add_tiles_from_blueprint("basic_house")
        .add_tiles_from_blueprint("brick_house")
        .add_tiles_from_blueprint("walkway")
        .add_tiles_from_blueprint("basic_house")
        .set_position(Location(16.,0.,3., WorldLocation::World))
        .add_tiles(RelativePosition::Below, 1,  tiles::ITEM.to_string())
        .add_mobs(Location(-32.,64.,3., WorldLocation::World), 10,  tiles::ENEMY.to_string());
        //.add_tiles_from_blueprint("walkway");*/
         //.add_tiles_from_blueprint("basic_house_2");
    


    for comp in mb.iter() {
        commands.spawn(comp.clone())
            .with_bundle(comp.sprite.to_components(comp.location.into(), Scale(1.)));
            
    }

    for mob in mb.mobs.iter() {
        commands.spawn(mob.clone()).with_bundle(mob.sprite.to_components(mob.location.into(), Scale(1.)));
    }

    //commands.spawn((Moveable, Location(TILE_SIZE*2.,TILE_SIZE*2.,2.), Visible));
}
