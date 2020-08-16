
use bevy::{prelude::Translation, math::Vec2, ecs::Bundle, prelude::Properties};

#[derive(Clone, Debug, Copy, PartialEq, Properties)]
pub struct Location (pub f32, pub f32, pub f32);

impl Location {
    pub fn from_translation(translation : Translation) -> Location {
        Location(translation.x(), translation.y(), translation.z())
    }
}
#[derive(Clone, PartialEq)]
pub struct Area(pub f32, pub f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Wall(Hardness),
    Floor,
    Lava,
    Bar,
    Grass,
    Key
}

#[derive(Copy, Clone, Debug)]
pub struct Visible;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Hardness (pub f32);
#[derive(Bundle, Copy, Clone, Debug)]
pub struct TileComponents {
    hardness: Hardness,
    tile_type: TileType,
    location: Location,
    visible: Visible
}

impl Default for TileComponents {
    fn default() -> Self {
        TileComponents {
            hardness: Hardness(0.),
            tile_type: TileType::Key,
            location: Location(0.,0.,0.),
            visible: Visible
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pushable;

pub struct MapBuilder {
    tile_size : Vec2,
    current_location : Location,
    tiles : Box<Vec<TileComponents>>
}

pub enum RelativePosition {
    LeftOf,
    RightOf,
    Above,
    Below
}

impl MapBuilder {
    pub fn new(tile_size : Vec2, starting_location: Location) -> MapBuilder {
        MapBuilder {
            tile_size : tile_size.clone(),
            current_location : starting_location,
            tiles: Box::new(Vec::new())
        }
    }
    pub fn add_tiles_to_area(&mut self, loc : Location, area: Area, tile_type: TileType){
        let tiles = self.tiles.as_mut();
                 

        for x in 0..area.0 as u32 {
            for y in 0..area.1 as u32 {                
                tiles.push(TileComponents {
                   tile_type: tile_type, 
                   location: Location(loc.0 + (x as f32 * self.tile_size.x()), loc.1 - (y as f32 * self.tile_size.y()), loc.2),
                   hardness: Hardness(0.),
                   ..Default::default()
                });            
            }
        }
    }
    pub fn add_tiles(&mut self, pos : RelativePosition, count : u32, tile_type: TileType){
        let tiles = self.tiles.as_mut();

        for _ in 0..count {
            let mut loc = &self.current_location;
            let location = match pos {
                RelativePosition::LeftOf => {                                    
                    Location(loc.0 - self.tile_size.x(), loc.1, loc.2)
                }
                RelativePosition::RightOf => {
                    Location(loc.0 + self.tile_size.x(), loc.1, loc.2)
                }
                RelativePosition::Above => {
                    Location(loc.0, loc.1 + self.tile_size.y(), loc.2)
                }
                RelativePosition::Below => {
                    Location(loc.0, loc.1 - self.tile_size.y(), loc.2)
                }
            };

            let hardness = match tile_type {
                TileType::Wall(h ) =>  {
                    h
                }, 
                _ => Hardness(0.),
            };

            tiles.push(TileComponents {
                tile_type: tile_type, 
                location: location,
                hardness: hardness,
                ..Default::default()
             });

            self.current_location = location;
        }
    }

    pub fn iter(&mut self) -> std::slice::Iter<'_, TileComponents> {
        self.tiles.iter()
    }

}