
use bevy::prelude::*;
use strum_macros::EnumIter;

#[derive(Clone, Debug, Copy, PartialEq, Properties, Default)]
pub struct Location (pub f32, pub f32, pub f32);

impl Location {
    pub fn from_translation(translation : Translation) -> Location {
        Location(translation.x(), translation.y(), translation.z())
    }
}

#[derive(Clone, PartialEq)]
pub struct Area(pub f32, pub f32);

#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
pub enum TileType {
    Wall(Hardness),
    Floor,
    Brick(Hardness),
    BrickDoorOpen,
    BrickDoorClosed(Hardness),
    BrickWindow(Hardness),
    BrickWindowBroken,
    Lava,
    Bar,
    Grass,
    Chair,
    Shelf,
    Bed,
    Table,
    Fridge,
    Key
}

#[derive(Copy, Clone, Debug)]
pub struct Visible;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Hardness (pub f32);

impl Default for Hardness {
    fn default() -> Hardness {
        return Hardness(1.)
    }
}


#[derive(Copy, Clone, Debug)]
pub struct Interaction {
    pub call : fn (Attributes) -> (bool, TileType)
}

#[derive(Bundle, Copy, Clone, Debug)]
pub struct TileComponents {
    pub hardness: Hardness,
    pub tile_type: TileType,
    pub location: Location,
    pub visible: Visible,
    pub interaction: Interaction
}

impl TileComponents {
    fn hardness_from_tile(tile_type: TileType) -> Hardness {
        match tile_type {
            TileType::Wall(h ) => h, 
            TileType::Brick(h ) =>  h,
            TileType::BrickWindow(h ) =>  h,
            TileType::BrickDoorClosed(h ) => h, 
            _ => Hardness(0.),
        }
    }
}

impl Default for TileComponents {
    fn default() -> Self {
        TileComponents {
            hardness: Hardness(0.),
            tile_type: TileType::Key,
            location: Location::default(),
            visible: Visible,
            interaction: Interaction { call: |_attributes| { (false, TileType::Key) } }
        }
    }
}

pub struct Despawn;

pub struct Attributes; /* {
    settings: HashMap<String, u32>
}*/
#[derive(Debug, Clone, Copy)]
pub struct Moveable;

#[derive(Debug, Clone, Copy)]
pub struct Solid;


#[derive(Debug)]
pub struct InteractionResult {
    message: String,
    colliding_entity : Option<Entity>
}
pub struct MapBuilder {
    pub tile_size : Vec2,
    pub current_location : Location,
    pub tiles : Vec<TileComponents>
}