



use bevy::{ prelude::* , prelude::Properties};

    
use std::fmt::{Formatter, Result};


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

#[derive(Clone, Copy, Default, Properties)]
pub struct Player;


impl Player {
    pub fn add_to_world(mut commands: Commands, name: &str) {
        commands.spawn((Player, crate::Named(name.to_string()), Job::BeerWizard));
    }
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