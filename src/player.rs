
use crate::world::Location;

struct Position(u32, u32);

#[derive(PartialEq, Debug)]
pub struct Moving(pub Location, pub Location, pub Direction);

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stationary
}

pub enum Job {
    BeerWizard,
    Brewer,
    BeerBitch,
    Regular,
    Undecided
}