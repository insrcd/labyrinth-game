use lab_world::*;
use lab_builder::prelude::*;
use lab_entities::prelude::*;
use crate::*;
use bevy::prelude::*;
use std::rc::Rc;

mod tiles {
    pub const wall : &'static str = "wall";
    pub const floor : &'static str = "floor";
}

/// Adds a simple map using the map builder for the purposes of a demo.

pub fn create_simple_map_system(mut commands: Commands, mut palette: ResMut<TilePalette>) {

    if let Some(mut tiles) = palette.components.get_mut(tiles::wall) {
        tiles.hardness = Hardness(1.);
        tiles.tile_attributes.hit_points = 1;
    }
    
    let mut mb = MapBuilder::new(
        Rc::new(palette.clone()), // may have to share the pallete later, so adding resource counting now
        &Location::default()
    );

    // wall tiles are hard in my world
    

    &mut mb
            .add_tiles(RelativePosition::RightOf, 5, tiles::wall.to_string())
            .add_tiles(RelativePosition::Below, 5, tiles::wall.to_string())
            .add_tiles(RelativePosition::LeftOf, 2, tiles::wall.to_string())
            .add_tiles(RelativePosition::LeftOf, 1, tiles::wall.to_string())
            .add_tiles(RelativePosition::LeftOf, 1, tiles::wall.to_string())
            .add_tiles(RelativePosition::LeftOf, 1, tiles::wall.to_string())
            .add_tiles(RelativePosition::Above, 5, tiles::wall.to_string())
            .add_tiles_to_area(&Location(0.,0.,0., WorldLocation::World), Area(6., 6.), tiles::floor.to_string())
            .to_blueprint("basic_house");

    mb
    .add_tiles(RelativePosition::RightOf, 5, tiles::wall.to_string())
    .add_tiles(RelativePosition::Below, 5, tiles::wall.to_string())
    .add_tiles(RelativePosition::LeftOf, 2, tiles::wall.to_string())
    .add_tiles(RelativePosition::LeftOf, 1, tiles::wall.to_string())
    .add_tiles(RelativePosition::LeftOf, 1, tiles::wall.to_string())
    .add_tiles(RelativePosition::LeftOf, 1, tiles::wall.to_string())
    .add_tiles(RelativePosition::Above, 5, tiles::wall.to_string())
    .add_tiles_to_area(&Location(0.,0.,0., WorldLocation::World), Area(6., 6.), tiles::floor.to_string())
    .to_blueprint("basic_house_2");

    mb
    .add_tiles_to_area(&Location::default(), Area(2., 8.), tiles::floor.to_string())
    .to_blueprint("walkway");


    mb.add_tiles_to_area(&Location::default(), Area(2., 2.), tiles::wall.to_string());
        /*.add_tiles_from_blueprint("basic_house")
        .add_tiles_from_blueprint("basic_house")
        .add_tiles_from_blueprint("walkway")
        .add_tiles_from_blueprint("basic_house")
        .add_tiles_from_blueprint("walkway");*/
        //.add_tiles_from_blueprint("walkway");*/
         //.add_tiles_from_blueprint("basic_house_2");
    


    for comp in mb.iter() {
        commands.spawn(comp.clone()).with_bundle(comp.sprite.to_components(comp.location.into(), Scale(1.)));
    }

    //commands.spawn((Moveable, Location(TILE_SIZE*2.,TILE_SIZE*2.,2.), Visible));
}
