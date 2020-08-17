
use crate::world::*;
use bevy::prelude::*;

pub struct DemoPlugin;

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(simple_map.system());
    }
}

const tile_size : f32 = 96.;

/// Adds a simple map for the purposes of a demo.

fn simple_map(mut commands: Commands) {

    let mut mb = MapBuilder::new(Vec2::new(tile_size,tile_size), Location(0.,0.,0.));

    mb.add_tiles(RelativePosition::RightOf, 5, TileType::Wall(Hardness(1.)));
    mb.add_tiles(RelativePosition::Below, 2, TileType::Wall(Hardness(1.)));
    mb.add_tiles(RelativePosition::Below, 1, TileType::Floor);
    mb.add_tiles(RelativePosition::Below, 2, TileType::Wall(Hardness(1.)));
    mb.add_tiles(RelativePosition::LeftOf, 5, TileType::Wall(Hardness(1.)));
    mb.add_tiles(RelativePosition::Above, 5, TileType::Wall(Hardness(1.)));

    mb.add_tiles_to_area(Location(0.,0.,0.), Area(5., 5.), TileType::Floor);

    for comp in mb.iter() {
        println!("{:?}", comp);
        commands.spawn(comp.clone());
    }

    commands.spawn((Moveable, Location(tile_size*2.,tile_size*2.,2.), Visible));
}
