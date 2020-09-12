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

    let bump = Arc::new(TileInteraction { caller: |ctx| {            
        TileInteractionResult::Block(ctx.source).into()
    }, description:"Bump" });

    // setup some basic interactions
    palette.interactions.insert(tiles::WALL.into(), bump.clone());
    palette.interactions.insert(tiles::BRICK.into(), bump.clone());

    palette.interactions.insert(tiles::BRICK_DOOR.into(), 
        Arc::new(
            TileInteraction { caller: |ctx| {            
        let comps = ctx.world_catalog.components.get(tiles::BRICK_DOOR_OPEN).expect("Open brick door tile cannot be found");        
        
        TileInteractionResult::ChangeSprite(ctx.destination, comps.sprite.clone()).into()

    }, description:"Open a door." }));

    if let Some(mut tiles) = palette.components.get_mut(tiles::WALL) {
        // walls are hard
        tiles.state.set_int("hardness".into(), 1);
        tiles.state.set_int("hit_points".into(), 800);     
        
        palette.interactions.insert("solid".into(), Arc::new(TileInteraction { caller: |ctx| {            
            TileInteractionResult::Block(ctx.source).into()
        }, description:"Bump" }));
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
            let itype = ctx.interaction_query.get::<InteractableType>(ctx.source).ok();
            
            if let Some(t) = itype {
                if let InteractableType::Player = *t {
                    return vec![TileInteractionResult::Message("Hello, you are my enemy. Lets fight.".into()),
                    TileInteractionResult::Block(ctx.source)];
                }
            }
            //TileInteractionResult::Block(ctx.source).into()
            TileInteractionResult::None.into()
        },
            description: "Enemy Interaction",}));
    }
    if let Some(tiles) = palette.components.get(tiles::BRICK_DOOR) {
        // open doors
        let mut new_tile = tiles.clone();
        new_tile.state.set_int("hardness".into(), 1);
        palette.components.insert("locked_door".into(), new_tile);
        palette.interactions.insert( "locked_door".into(),Arc::new(TileInteraction { caller: |ctx| {    
            
            let comps = ctx.world_catalog.components.get(tiles::BRICK_DOOR_OPEN).expect("Open brick door tile cannot be found");        
            
            let inventory = ctx.interaction_query.get::<Inventory>(ctx.source).unwrap();
            
            for e in (*inventory).0.iter() {
                let item = ctx.item_query.get::<Named>(*e).unwrap();
                if item.0 == "Key To Building 2" {
                    return vec![
                        TileInteractionResult::ChangeSprite(ctx.destination, comps.sprite.clone()),
                        TileInteractionResult::Message("You have the key, Unlocked the door!".into())];
                }
            }

            vec![TileInteractionResult::Block(ctx.source), TileInteractionResult::Message("The door is locked, maybe there's a key somewhere".into())]
        },
            description: "Open Door",}));
    }
    
    if let Some(mut tiles) = palette.components.get_mut(tiles::BRICK_WINDOW) {

        palette.interactions.insert(tiles::BRICK_WINDOW.into(), Arc::new(TileInteraction { 
            caller: |ctx| {  
                let comps = ctx.world_catalog.components.get(tiles::BRICK_WINDOW_OPEN).expect("Open brick door tile cannot be found");        
                let itype = ctx.interaction_query.get::<InteractableType>(ctx.source).ok();
                
                println!("{:?} interacted with {:?} for window", ctx.source, ctx.destination);
                // if a non-player hits a window, crash it if not block it 
                if let Some(source_type) = itype {
                    return match *source_type {
                        InteractableType::Item |  InteractableType::Npc => TileInteractionResult::ChangeSprite(ctx.destination,comps.sprite.clone()).into(),
                        _ => vec![TileInteractionResult::Block(ctx.source),TileInteractionResult::Message("The window looks breakable.".to_string())]
                    };
                } else {
                    vec![TileInteractionResult::Block(ctx.source),TileInteractionResult::Message("The window looks breakable.".to_string())]
                }
            }, description: "Break Window" }));
    }
    if let Some(mut tiles) = palette.components.get_mut(tiles::ITEM) {
        // break windows
        palette.interactions.insert(
            tiles::ITEM.into(), 
            Arc::new( TileInteraction { description: "Get Item", caller: |ctx| { 
                // demoooo   
                let item = ItemComponents { 
                    name: Named("Key To Building 2".into()),
                    weight: Weight(0.1),
                    item_type: ItemType::Key,
                    item_slot: ItemSlot::LeftHand,
                    handle: ItemHandle { item_id : 1}
                };
                println!("{:?} interacted with {:?} for key", ctx.source, ctx.destination);
                let itype = ctx.interaction_query.get::<InteractableType>(ctx.source).ok();
            
                if let Some(t) = itype {
                    if let InteractableType::Player = *t {
                    return vec![ 
                        TileInteractionResult::AddItem(ctx.source, item),
                        TileInteractionResult::Despawn, 
                        TileInteractionResult::Message(format!("You picked up the key").into())
                        ]
                    };
                }
                
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
            .with_bundle(comp.sprite.to_components(comp.location.into(), Scale(1.)))
            .with_bundle(Interactable::new(InteractableType::Tile));
        //println!("Spawning entity {:?} {:?}", comp, commands.current_entity());
            
    } 

    for mob in mb.mobs.iter() {
        commands.spawn(mob.clone())
            .with_bundle(mob.sprite.to_components(mob.location.into(), Scale(1.)))
            .with_bundle(Interactable::new(InteractableType::Npc));
    }

    //commands.spawn((Moveable, Location(TILE_SIZE*2.,TILE_SIZE*2.,2.), Visible));
}
