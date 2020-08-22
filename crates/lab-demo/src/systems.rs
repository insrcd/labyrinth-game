use lab_world::*;
use lab_builder::prelude::*;
use lab_entities::prelude::*;
use crate::*;
use bevy::prelude::*;

const TILE_SIZE : f32 = 96.;

/// Adds a simple map using the map builder for the purposes of a demo.

pub fn create_simple_map_system(mut commands: Commands) {


    let mut mb = MapBuilder::new(
        Vec2::new(TILE_SIZE,TILE_SIZE),
        &Location::default()
    );

    mb
        .add_tiles(RelativePosition::RightOf, 5, TileType::BrickDoorClosed(Hardness(1.)))
        .add_tiles(RelativePosition::Below, 5, TileType::BrickDoorClosed(Hardness(1.)))
        .add_tiles(RelativePosition::LeftOf, 2, TileType::BrickDoorClosed(Hardness(1.)))
        .add_tiles(RelativePosition::LeftOf, 1, TileType::BrickDoorClosed(Hardness(1.)))
        .add_tiles(RelativePosition::LeftOf, 1, TileType::BrickDoorClosed(Hardness(1.)))
        .add_tiles(RelativePosition::LeftOf, 1, TileType::BrickDoorClosed(Hardness(1.)))
        .add_tiles(RelativePosition::Above, 5, TileType::BrickDoorClosed(Hardness(1.)))
        .add_tiles_to_area(&Location(TILE_SIZE, -TILE_SIZE,5., WorldLocation::World), Area(4., 4.), TileType::Floor);

    mb.to_blueprint("basic_house");

    mb
    .add_tiles(RelativePosition::RightOf, 2, TileType::Brick(Hardness(1.)))
    .add_tiles(RelativePosition::RightOf, 1, TileType::BrickWindow(Hardness(1.)))
    .add_tiles(RelativePosition::RightOf, 1, TileType::BrickDoorClosed(Hardness(1.)))
    .add_tiles(RelativePosition::RightOf, 1, TileType::Brick(Hardness(1.)))
    .add_tiles(RelativePosition::Below, 5, TileType::Brick(Hardness(1.)))
    .add_tiles(RelativePosition::LeftOf, 5, TileType::Brick(Hardness(1.)))
    .add_tiles(RelativePosition::Above, 5, TileType::Brick(Hardness(1.)))
    .add_tiles_to_area(&Location::default(), Area(7., 7.), TileType::Floor)
    .to_blueprint("basic_house_2");

    mb
      .add_tiles(RelativePosition::Current, 1, TileType::Grass)
      .add_tiles(RelativePosition::Below, 5, TileType::Grass)
      .add_tiles(RelativePosition::RightOf, 1, TileType::Grass)
      .add_tiles(RelativePosition::Above, 5, TileType::Grass)
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
