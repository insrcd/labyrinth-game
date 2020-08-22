
use bevy::prelude::*;
use lab_entities::prelude::*;

use crate::*;
use lab_world::settings::WORLD_TILE_SIZE;

#[derive(Clone, Debug)]
pub struct Blueprint {
    pub name : String,
    pub tiles : Vec<TileComponents>
}

impl Blueprint {
    fn top_right(&self) -> Location {
        let mut location = Location::default();
        
        for tiles in self.tiles.as_slice() {
            if tiles.location.0 > location.0 {
                location.0 = tiles.location.0;
            }
            if tiles.location.1 > location.1 {
                location.1 = tiles.location.1;
            }
        }

        //location.0 += WORLD_TILE_SIZE;


        location
    }
}

pub struct MapBuilder {
    pub tile_size : Vec2,
    pub starting_location : Location,
    pub current_location : Location,
    pub tiles : Vec<TileComponents>,
    pub blueprints : Vec<Blueprint>
}

impl<'a>  MapBuilder {
    pub fn new(tile_size : Vec2, starting_location: &Location) -> MapBuilder {
        MapBuilder {
            tile_size : tile_size.clone(),
            starting_location : starting_location.clone(),
            current_location : starting_location.clone(),
            tiles: Vec::new(),
            blueprints: Vec::new()
        }
    }
    pub fn to_blueprint(&mut self, name : &str) -> &MapBuilder{
        self.blueprints.push(Blueprint {
               name: name.to_string(),
               tiles: self.tiles.clone()
        });

        self.tiles = Vec::new();

        self.current_location = self.starting_location;

        println!("Adding blueprint");
        

        self
    }

    pub fn add_tiles_from_blueprint(&mut self, name : &str) -> &mut MapBuilder {
        
        println!("Adding blueprint tiles at {:?}", self.current_location);

        for bp in self.blueprints.iter() {
            if bp.name != name {
                continue;
            }

            // right now just put to the right of the last tile
            self.current_location.0 += WORLD_TILE_SIZE;

            for tile in bp.tiles.as_slice() {
                let mut c =  tile.clone();

                c.location.0 += self.current_location.0;
                c.location.1 += self.current_location.1;

                self.tiles.push(c);
            }
            
            let offset = bp.top_right();

            self.current_location.0 += offset.0;
            self.current_location.1 += offset.1;
        }
        self
    }

    pub fn add_tiles_to_area(&mut self, loc : &Location, area: Area, tile_type: TileType) -> &mut  MapBuilder{
                 

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

        self
    }
    pub fn add_tiles(&mut self, pos : RelativePosition, count : u32, tile_type: TileType) -> &mut MapBuilder {
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

            println!("Adding tile at {:?} last location: {:?}", self.current_location, location);

            let interaction: fn (Attributes) -> InteractionResult = match tile_type {
                TileType::BrickWindow(_) => |_| {InteractionResult::ChangeTile(TileType::BrickWindowBroken)},
                TileType::BrickDoorClosed(_) => |_| {InteractionResult::ChangeTile(TileType::BrickDoorOpen)},
                _ => |_| {InteractionResult::None}
            };

            
            self.tiles.push(TileComponents {
                tile_type: tile_type, 
                location: location.clone(),
                hardness: TileComponents::hardness_from_tile(tile_type),
                interaction: lab_entities::world::Interaction { call: interaction },
                visible: Visible
             });

            self.current_location = location.to_owned();
        }

        self
    }

    pub fn iter(&mut self) -> std::slice::Iter<'_, TileComponents> {
        self.tiles.iter()
    }

}