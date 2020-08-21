
use bevy::prelude::*;
use lab_entities::prelude::*;

use crate::*;

pub struct MapBuilder {
    pub tile_size : Vec2,
    pub current_location : Location,
    pub tiles : Vec<TileComponents>
}

impl<'a>  MapBuilder {
    pub fn new(tile_size : Vec2, starting_location: & Location) -> MapBuilder {
        MapBuilder {
            tile_size : tile_size.clone(),
            current_location : starting_location.to_owned(),
            tiles: Vec::new()
        }
    }
    pub fn add_tiles_to_area(&mut self, loc : &Location, area: Area, tile_type: TileType){
                 

        for x in 0..area.0 as u32 {
            for y in 0..area.1 as u32 {                
                self.tiles.push(TileComponents {
                   tile_type: tile_type, 
                   location: Location(loc.0 + (x as f32 * self.tile_size.x()), loc.1 - (y as f32 * self.tile_size.y()), loc.2,  world::WorldLocation::World),
                   hardness:TileComponents::hardness_from_tile(tile_type),
                   visible: Visible,
                   ..Default::default()
                });            
            }
        }
    }
    pub fn add_tiles(&mut self, pos : RelativePosition, count : u32, tile_type: TileType){
       

        for _ in 0..count {
            let loc = self.current_location;
            let location = match pos {
                RelativePosition::LeftOf => {                                    
                    Location(loc.0 - self.tile_size.x(), loc.1, loc.2, world::WorldLocation::World)
                }
                RelativePosition::RightOf => {
                    Location(loc.0 + self.tile_size.x(), loc.1, loc.2, world::WorldLocation::World)
                }
                RelativePosition::Above => {
                    Location(loc.0, loc.1 + self.tile_size.y(), loc.2, world::WorldLocation::World)
                }
                RelativePosition::Below => {
                    Location(loc.0, loc.1 - self.tile_size.y(), loc.2, world::WorldLocation::World)
                }
            };
            
            self.tiles.push(TileComponents {
                tile_type: tile_type, 
                location: location.clone(),
                hardness: TileComponents::hardness_from_tile(tile_type),
                ..Default::default()
             });

            self.current_location = location.to_owned();
        }
    }

    pub fn iter(&mut self) -> std::slice::Iter<'_, TileComponents> {
        self.tiles.iter()
    }

}