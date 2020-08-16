



use bevy::{ prelude::* , prelude::Properties};

    
use std::fmt::{Formatter, Result};


use crate::world::Location;

struct Position(u32, u32);

#[derive(PartialEq, Debug, Default)]
pub struct Moving(pub Location, pub Location, pub Direction);

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Stationary,
    Up,
    Down,
    Left,
    Right
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "A player!")
    }
}

pub enum Job {
    BeerWizard,
    Brewer,
    BeerBitch,
    Regular,
    Undecided
}