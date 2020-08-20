
use bevy::prelude::Translation;

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
