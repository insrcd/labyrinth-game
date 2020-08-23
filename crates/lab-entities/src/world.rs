use bevy::{prelude::{Translation, Properties, Bundle, Entity}, math::Vec3};
use lab_sprites::{TileAnimation, SpriteInfo};
use lab_core::Zoomable;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Placeable,
    Breakable(Hardness),
    Immutable,
    Floor
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
    ChangeTile(TileAttributes),
    Damage(u32),
    ChangeSprite(SpriteInfo),
    Move(Location),
    PickUp(crate::objs::Item),
    None
}
#[derive(Copy, Clone, Debug)]
pub struct Interaction {
    pub call : fn (Attributes) -> InteractionResult
}


/// Attributes for a tile. These are meant to be changed by the player or interactions.

#[derive(Default, Clone, Copy, Properties, Debug)]
pub struct TileAttributes {
    pub hit_points: u32,
    pub hardness: f32, 
    pub sprite_idx: i32
}

#[derive(Bundle, Clone, Debug)]
pub struct TileComponents {
    pub hardness: Hardness,
    pub tile_type: TileType,
    pub location: Location,
    pub visible: Visible,
    pub interaction: Interaction,
    pub sprite: SpriteInfo,
    pub animation: TileAnimation,
    pub tile_attributes: TileAttributes,
    pub zoomable: Zoomable
}

impl TileComponents {
    pub fn hardness_from_tile(tile_type: TileType) -> Hardness {
        match tile_type {
            TileType::Immutable => Hardness(999.), 
            TileType::Breakable(h ) =>  h,
            _ => Hardness(0.),
        }
    }
}

impl Default for TileComponents {
    fn default() -> Self {
        TileComponents {
            hardness: Hardness(0.),
            tile_type: TileType::Floor,
            location: Location::default(),
            visible: Visible,
            interaction: Interaction { call: |_attributes| { InteractionResult::None } },
            tile_attributes: TileAttributes { hit_points: 0, hardness: 0.0, sprite_idx: 0 },
            sprite: SpriteInfo::default(),
            animation: TileAnimation::default(),
            zoomable: Zoomable
        }
    }
}

#[derive(Default, Clone)]
pub struct Attributes {
    pub inventory: Option<crate::player::Inventory>,
    pub player: Option<Entity>,
    pub player_location: Option<Location>,
    pub interaction_location: Option<Location>,
    pub tile_attributes: Option<TileAttributes>
}
