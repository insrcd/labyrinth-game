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
