
use crate::world::*;
use bevy::prelude::*;

pub struct DemoPlugin;

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(simple_map.system());
    }
}

const TILE_SIZE : f32 = 96.;

/// Adds a simple map using the map builder for the purposes of a demo.

fn simple_map(mut commands: Commands) {

    let starting_location = Location(TILE_SIZE * 5., TILE_SIZE * 5.   ,0.);

    let mut mb = MapBuilder::new(
        Vec2::new(TILE_SIZE,TILE_SIZE),
        &starting_location 
);

    mb.add_tiles(RelativePosition::RightOf, 5, TileType::Brick(Hardness(1.)));
    mb.add_tiles(RelativePosition::Below, 2, TileType::Brick(Hardness(1.)));
    mb.add_tiles(RelativePosition::Below, 1, TileType::BrickDoorClosed(Hardness(1.)));
    mb.add_tiles(RelativePosition::Below, 2, TileType::Brick(Hardness(1.)));
    mb.add_tiles(RelativePosition::LeftOf, 5, TileType::Brick(Hardness(1.)));
    mb.add_tiles(RelativePosition::Above, 5, TileType::Brick(Hardness(1.)));

    mb.add_tiles_to_area(&starting_location, Area(5., 5.), TileType::Floor);

    for comp in mb.iter() {
        commands.spawn(comp.clone());
    }

    //commands.spawn((Moveable, Location(TILE_SIZE*2.,TILE_SIZE*2.,2.), Visible));
}
