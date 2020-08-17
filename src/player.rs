



use bevy::{ prelude::* , prelude::Properties};



use crate::world::Location;

struct Position(u32, u32);

#[derive(PartialEq, Debug)]
pub struct Moving(pub Location, pub Location, pub Direction);
impl Default for Moving {
    fn default() -> Moving {
        Moving(Location::default(), Location::default(), Direction::Stationary)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Stationary,
    Up,
    Down,
    Left,
    Right
}

pub enum Job {
    BeerWizard,
    Brewer,
    BeerBitch,
    Regular,
    Undecided
}