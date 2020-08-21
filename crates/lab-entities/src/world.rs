
use bevy::prelude::*;
use strum_macros::EnumIter;
use crate::objs::*;
use crate::player::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WorldLocation {
    World,
    Inventory,
    Labyrinth,
    BarRoom
}
#[derive(Clone, Debug, Copy, PartialEq, Properties)]
pub struct Location (pub f32, pub f32, pub f32, 
    #[property(ignore)] pub WorldLocation);
/*
impl Into<Vec3> for Location {
    fn into(self) -> Vec3 {
        Vec3::new(self.0, self.1, self.2)
    }
}*/

impl Default for Location {
    fn default() -> Self {
        return Location(0.,0.,0.,WorldLocation::World)
    }
    
}
impl From<Location> for Vec3 {
    fn from(x: Location) -> Self {
        Vec3::new(x.0, x.1, x.2)
    }
    
}

impl From<Translation> for Location {
    fn from(t : Translation) -> Self {
        Location (t.0.x(), t.0.y(), t.0.z(), WorldLocation::World)
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
    Key,
    Mug
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


pub enum InteractionResult {
    ChangeTile(TileType),
    Damage(u32),
    ChangeSprite(Sprite),
    Move(Location),
    PickUp(Item),
    None
}
#[derive(Copy, Clone, Debug)]
pub struct Interaction {
    pub call : fn (Attributes) -> InteractionResult
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
    pub fn hardness_from_tile(tile_type: TileType) -> Hardness {
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
            interaction: Interaction { call: |_attributes| { InteractionResult::None } }
        }
    }
}

pub struct Despawn;

#[derive(Default, Clone)]
pub struct Attributes {
    pub inventory: Option<Inventory>,
    pub player: Option<Entity>,
    pub player_location: Option<Location>,
    pub interaction_location: Option<Location>
}
#[derive(Debug, Clone, Copy)]
pub struct Moveable;

#[derive(Debug, Clone, Copy)]
pub struct Solid;

/*
#[derive(Debug)]
pub struct InteractionResult {
    message: String,
    colliding_entity : Option<Entity>
}
*/