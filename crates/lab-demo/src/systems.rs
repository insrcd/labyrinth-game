use lab_world::*;
use lab_builder::prelude::*;
use lab_entities::prelude::*;
use crate::*;
use bevy::prelude::*;
use std::rc::Rc;

const TILE_SIZE : f32 = 96.;

mod tiles {
    pub const wall : &'static str = "wall";
    pub const floor : &'static str = "floor";
}

/// Adds a simple map using the map builder for the purposes of a demo.

pub fn create_simple_map_system(mut commands: Commands, mut palette: ResMut<TilePalette>) {
    let p = &*palette;

    let mut mb = MapBuilder::new(
        Rc::new(p.clone()), // may have to share the pallete later, so adding resource counting now
        &Location::default()
    );

    &mut mb
            .add_tiles(RelativePosition::RightOf, 5, tiles::wall.to_string())
            .add_tiles(RelativePosition::Below, 5, tiles::wall.to_string())
            .add_tiles(RelativePosition::LeftOf, 2, tiles::wall.to_string())
            .add_tiles(RelativePosition::LeftOf, 1, tiles::wall.to_string())
            .add_tiles(RelativePosition::LeftOf, 1, tiles::wall.to_string())
            .add_tiles(RelativePosition::LeftOf, 1, tiles::wall.to_string())
            .add_tiles(RelativePosition::Above, 5, tiles::wall.to_string())
            .add_tiles_to_area(&Location(TILE_SIZE, -TILE_SIZE,5., WorldLocation::World), Area(4., 4.), tiles::floor.to_string())
            .to_blueprint("basic_house");

    mb
    .add_tiles(RelativePosition::RightOf, 5, tiles::wall.to_string())
    .add_tiles(RelativePosition::Below, 5, tiles::wall.to_string())
    .add_tiles(RelativePosition::LeftOf, 2, tiles::wall.to_string())
    .add_tiles(RelativePosition::LeftOf, 1, tiles::wall.to_string())
    .add_tiles(RelativePosition::LeftOf, 1, tiles::wall.to_string())
    .add_tiles(RelativePosition::LeftOf, 1, tiles::wall.to_string())
    .add_tiles(RelativePosition::Above, 5, tiles::wall.to_string())
    .add_tiles_to_area(&Location(TILE_SIZE, -TILE_SIZE,5., WorldLocation::World), Area(4., 4.), tiles::floor.to_string())
    .to_blueprint("basic_house_2");

    mb
      .add_tiles(RelativePosition::Current, 1, "grass".to_string())
      .add_tiles(RelativePosition::Below, 5, "grass".to_string())
      .add_tiles(RelativePosition::RightOf, 1, "grass".to_string())
      .add_tiles(RelativePosition::Above, 5, "grass".to_string())
    //.add_tiles_to_area(&Location::default(), Area(2., 8.), TileType::Floor)
    .to_blueprint("walkway");


    mb
        .add_tiles_from_blueprint("basic_house")
        .add_tiles_from_blueprint("basic_house")
        .add_tiles_from_blueprint("walkway")
        .add_tiles_from_blueprint("basic_house")
        .add_tiles_from_blueprint("walkway");
        //.add_tiles_from_blueprint("walkway");*/
         //.add_tiles_from_blueprint("basic_house_2");
    


    for comp in mb.iter() {
        commands.spawn(comp.clone());
    }

    //commands.spawn((Moveable, Location(TILE_SIZE*2.,TILE_SIZE*2.,2.), Visible));
}
