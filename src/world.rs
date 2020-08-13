
use bevy::math::Vec2;

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Location (pub f32, pub f32, pub f32);
#[derive(Clone, PartialEq)]
pub struct Area(pub f32, pub f32);

#[derive(Clone, Debug, Copy)]
pub struct Tile (pub TileType, pub Location);
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    Lava,
    Bar,
    Grass,
    Key
}
pub struct Collidable;
pub struct Visible;


pub struct AreaMap;

pub struct MapBuilder {
    tile_size : Vec2,
    current_tile : Tile,
    tiles : Box<Vec<Tile>>
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
            current_tile : Tile (TileType::Key, starting_location.clone()),
            tiles: Box::new(Vec::new())
        }
    }
    pub fn add_tiles_to_area(&mut self, loc : Location, area: Area, tile_type: TileType){
        let tiles = self.tiles.as_mut();
                 

        for x in 0..area.0 as u32 {
            for y in 0..area.1 as u32 {                
                tiles.push(Tile(tile_type, Location(loc.0 + (x as f32 * self.tile_size.x()), loc.1 - (y as f32 * self.tile_size.y()), loc.2)));            
            }
        }
    }
    pub fn add_tiles(&mut self, pos : RelativePosition, count : u32, tile_type: TileType){
        let tiles = self.tiles.as_mut();

        for i in 0..count {
            let mut loc = &self.current_tile.1;
            let new_tile = match pos {
                RelativePosition::LeftOf => {                                    
                    Tile(tile_type, Location(loc.0 - self.tile_size.x(), loc.1, loc.2))
                }
                RelativePosition::RightOf => {
                    Tile(tile_type, Location(loc.0 + self.tile_size.x(), loc.1, loc.2))
                }
                RelativePosition::Above => {
                    Tile(tile_type, Location(loc.0, loc.1 + self.tile_size.y(), loc.2))
                }
                RelativePosition::Below => {
                    Tile(tile_type, Location(loc.0, loc.1 - self.tile_size.y(), loc.2))
                }
            };

            tiles.push(new_tile);
            self.current_tile = new_tile;
        }
    }
    pub fn iter(&mut self) -> std::slice::Iter<'_, Tile> {
        self.tiles.iter()
    }
}