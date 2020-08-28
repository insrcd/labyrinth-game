use lab_world::*;
use lab_builder::prelude::*;
use lab_entities::prelude::*;
use crate::*;
use std::rc::Rc;

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

    if let Some(mut tiles) = palette.components.get_mut(tiles::WALL) {
        // walls are hard
        tiles.hardness = Hardness(1.);
        tiles.tile_attributes.hardness = 1.;
        tiles.tile_attributes.hit_points = 200;
    }
    
    if let Some(mut tiles) = palette.components.get_mut(tiles::BRICK) {
        // brick walls are beefier
        tiles.hardness = Hardness(1.);
        tiles.tile_attributes.hardness = 1.;
        tiles.tile_attributes.hit_points = 800;
    }
    if let Some(mut tiles) = palette.components.get_mut(tiles::BRICK_DOOR) {
        // open doors
        tiles.hardness = Hardness(0.5);
        tiles.tile_attributes.hardness = 1.;
        tiles.tile_attributes.hit_points = 1;
        tiles.interaction = lab_world::Interaction { call: |ctx| {            
            InteractionResult::ChangeTile(TileAttributes { hardness: 0.0, sprite_idx: Some(ctx.tile_palette.unwrap().components.get(tiles::BRICK_DOOR_OPEN).unwrap().sprite.atlas_sprite), ..Default::default()}).into()
        },
            description: "Open Door",}
    }
    if let Some(mut tiles) = palette.components.get_mut(tiles::ENEMY) {
        // open doors
        tiles.interaction = lab_world::Interaction { call: |ctx| {            
            if let Some(source) = ctx.source_type {
                return vec![
                    InteractionResult::Message("Hello, you are my enemy. Lets fight.".into()).into(),
                    InteractionResult::Block.into()];
            }
            InteractionResult::Block.into()
        },
            description: "Enemy Interaction",}
    }
    if let Some(tiles) = palette.components.get(tiles::BRICK_DOOR) {
        // open doors
        let mut new_tile = tiles.clone();

        new_tile.hardness = Hardness(0.5);
        new_tile.tile_attributes.hardness = 1.;
        new_tile.tile_attributes.hit_points = 1;
        new_tile.interaction = lab_world::Interaction { call: |ctx| {    
            let palette = ctx.tile_palette.unwrap_or_else(|| panic!("Did not receive a palette in the interaction context."));

            // poor state tracking right now TODO Refactor and make safer
            let open_sprite = palette.components.get(tiles::BRICK_DOOR_OPEN).unwrap().sprite.atlas_sprite;
            let current_sprite = ctx.sprite_info.unwrap().atlas_sprite;

            if open_sprite == current_sprite {
                return InteractionResult::None.into()
            }

            if let Some(inventory) = ctx.inventory 
            {

                if inventory.has(|i| i.item_type == ItemType::Key && i.id == 1){
                    return vec![
                            InteractionResult::ChangeTile(TileAttributes { hardness: 0.0, sprite_idx: Some(open_sprite), message: Some(""), ..Default::default()}),
                            InteractionResult::Message("You have the key, Unlocked the door!".into())];
                }
            }
            vec![InteractionResult::Block, InteractionResult::Message("The door is locked, maybe there's a key somewhere".into())]
        },
            description: "Open Door",};

        palette.components.insert("locked_door".to_string(), new_tile);
    }
    if let Some(mut tiles) = palette.components.get_mut(tiles::BRICK_WINDOW) {

        tiles.hardness = Hardness(0.5);
        tiles.tile_attributes.hardness = 1.;
        tiles.tile_attributes.hit_points = 1;
        tiles.interaction = lab_world::Interaction { call: |ctx| {  
            let open_sprite = Some(ctx.tile_palette.unwrap().components.get(tiles::BRICK_WINDOW_OPEN).unwrap().sprite.atlas_sprite);
                 
            // if a non-player hits a window, crash it if not block it 
            if let Some(source_type) = ctx.source_type {
                return match source_type {
                    InteractableType::Item |  InteractableType::Npc => InteractionResult::ChangeTile(TileAttributes { hardness: 0.0, sprite_idx: open_sprite, ..Default::default()}).into(),
                    _ => vec![InteractionResult::Block,InteractionResult::Message("The window looks breakable.".to_string())]
                };
            } else {
                vec![InteractionResult::Block,InteractionResult::Message("The window looks breakable.".to_string())]
            }
        },
            description: "Break Window",}
    }
    if let Some(mut tiles) = palette.components.get_mut(tiles::ITEM) {
        // break windows
        tiles.interaction = lab_world::Interaction { call: |ctx| { 
            // demoooo   
            let item = Item { 
                id : 1,
                name: "Key To Building 2".to_string(),
                weight: Weight(0.1),
                item_type: ItemType::Key,
                item_slot: ItemSlot::LeftHand
            };
            
            // do domestuff now
            if let Some(inventory) = ctx.inventory {
                inventory.items.push(item.clone())
            }

            if let Some(source_type) = ctx.source_type {
                if source_type == InteractableType::Player {
                    return vec![ 
                        InteractionResult::Despawn, 
                        InteractionResult::Message(format!("You picked up an item: {}", item.name).to_string())
                        ]
                };
            };
            
            InteractionResult::None.into()
        },
            description: "Get Item",}
    }
    
    let mut mb = MapBuilder::new(
        Rc::new(palette.clone()), // may have to share the pallete later, so adding resource counting now
        &Location::default()
    );

    &mut mb
            .add_tiles(RelativePosition::RightOf, 5, tiles::WALL.to_string())
            .add_tiles(RelativePosition::Below, 5, tiles::WALL.to_string())
            .add_tiles(RelativePosition::LeftOf, 1, tiles::WALL.to_string())
            .add_tiles(RelativePosition::LeftOf, 1, tiles::BRICK_DOOR.to_string())
            .add_tiles(RelativePosition::LeftOf, 1, tiles::WALL.to_string())
            .add_tiles(RelativePosition::LeftOf, 2, tiles::WALL.to_string())
            .add_tiles(RelativePosition::Above, 5, tiles::WALL.to_string())
            .add_tiles_to_area(&Location(0.,0.,0., WorldLocation::World), Area(6., 6.), tiles::FLOOR.to_string())
            .to_blueprint("basic_house");

    mb
    .add_tiles(RelativePosition::RightOf, 5, tiles::BRICK.to_string())
    .add_tiles(RelativePosition::Below, 5, tiles::BRICK.to_string())
    .add_tiles(RelativePosition::LeftOf, 2, tiles::BRICK_WINDOW.to_string())
    .add_tiles(RelativePosition::LeftOf, 1, tiles::LOCKED_DOOR.to_string())
    .add_tiles(RelativePosition::LeftOf, 1, tiles::BRICK_WINDOW.to_string())
    .add_tiles(RelativePosition::LeftOf, 1, tiles::BRICK.to_string())
    .add_tiles(RelativePosition::Above, 5, tiles::BRICK.to_string())
    .add_tiles_to_area(&Location(0.,0.,0., WorldLocation::World), Area(6., 6.), tiles::FLOOR.to_string())
    .to_blueprint("brick_house");

    mb
    .add_tiles_to_area(&Location::default(), Area(2., 6.), tiles::FLOOR.to_string())
    .to_blueprint("walkway");


    mb
        .add_tiles_from_blueprint("basic_house")
        .add_tiles_from_blueprint("brick_house")
        .add_tiles_from_blueprint("walkway")
        .add_tiles_from_blueprint("basic_house")
        .set_position(Location(16.,0.,3., WorldLocation::World))
        .add_tiles(RelativePosition::Below, 1,  tiles::ITEM.to_string())
        .set_position(Location(-32.,64.,3., WorldLocation::World))
        .add_tiles(RelativePosition::Below, 5,  tiles::ENEMY.to_string());
        //.add_tiles_from_blueprint("walkway");*/
         //.add_tiles_from_blueprint("basic_house_2");
    


    for comp in mb.iter() {
        commands.spawn(comp.clone()).with_bundle(comp.sprite.to_components(comp.location.into(), Scale(1.)));
    }

    //commands.spawn((Moveable, Location(TILE_SIZE*2.,TILE_SIZE*2.,2.), Visible));
}
